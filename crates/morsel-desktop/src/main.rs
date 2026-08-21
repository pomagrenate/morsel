//! # morsel-desktop
//!
//! Desktop tray application and background service launcher for Morsel Clipboard Manager.

use anyhow::{Context, Result};
use auto_launch::AutoLaunchBuilder;
use morsel_clipboard::{ClipboardMonitor, InMemoryClipboard, MonitorConfig};
use morsel_storage::{SqliteStorage, StorageBackend, StorageConfig};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tracing::{error, info, Level};
use tray_icon::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tray_icon::{TrayIconBuilder, TrayIconEvent};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging subscriber
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting Morsel Desktop Service...");

    // Setup Auto-Start Manager
    let current_exe = std::env::current_exe().unwrap_or_default();
    let auto_launch = AutoLaunchBuilder::new()
        .set_app_name("MorselClipboardManager")
        .set_app_path(&current_exe.to_string_lossy())
        .set_args(&["--minimized"])
        .build();

    let is_autostart_enabled = auto_launch
        .as_ref()
        .map(|al| al.is_enabled().unwrap_or(false))
        .unwrap_or(false);

    info!("OS Auto-Start enabled: {}", is_autostart_enabled);

    // Shared state for pausing monitoring
    let monitoring_active = Arc::new(AtomicBool::new(true));

    // Spawn Background Daemon Task
    let monitoring_active_clone = monitoring_active.clone();
    tokio::spawn(async move {
        if let Err(e) = run_background_daemon(monitoring_active_clone).await {
            error!("Background daemon error: {}", e);
        }
    });

    // Create Tray Menu
    let tray_menu = Menu::new();

    let open_item = MenuItem::new("📋 Open Quick Search", true, None);
    let pause_item = CheckMenuItem::new("⏯️ Pause Monitoring", true, false, None);
    let autostart_item = CheckMenuItem::new("🚀 Launch on System Boot", true, is_autostart_enabled, None);
    let separator = PredefinedMenuItem::separator();
    let exit_item = MenuItem::new("❌ Exit Morsel", true, None);

    tray_menu.append(&open_item)?;
    tray_menu.append(&pause_item)?;
    tray_menu.append(&autostart_item)?;
    tray_menu.append(&separator)?;
    tray_menu.append(&exit_item)?;

    // Load or generate Tray Icon RGBA data (16x16 icon)
    let icon_data = create_default_tray_icon();
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Morsel Clipboard Manager (Running)")
        .with_icon(tray_icon::Icon::from_rgba(icon_data, 16, 16).expect("Valid icon data"))
        .build()?;

    info!("System Tray Icon initialized successfully.");

    // TAO Event Loop for System Tray & Desktop Events
    let event_loop = EventLoopBuilder::new().build();
    let tray_channel = TrayIconEvent::receiver();
    let menu_channel = tray_icon::menu::MenuEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(100),
        );

        // Process Tray Icon Click Events
        if let Ok(_event) = tray_channel.try_recv() {
            info!("Tray icon clicked. Launching Morsel TUI...");
            spawn_tui_process();
        }

        // Process Tray Menu Action Events
        if let Ok(event) = menu_channel.try_recv() {
            if event.id == open_item.id() {
                info!("Menu: Open Quick Search selected.");
                spawn_tui_process();
            } else if event.id == pause_item.id() {
                let is_paused = pause_item.is_checked();
                monitoring_active.store(!is_paused, Ordering::Relaxed);
                info!("Menu: Monitoring active state set to {}", !is_paused);
            } else if event.id == autostart_item.id() {
                let enable = autostart_item.is_checked();
                if let Ok(ref al) = auto_launch {
                    if enable {
                        let _ = al.enable();
                        info!("Auto-start enabled in OS settings.");
                    } else {
                        let _ = al.disable();
                        info!("Auto-start disabled in OS settings.");
                    }
                }
            } else if event.id == exit_item.id() {
                info!("Menu: Exit Morsel selected. Shutting down...");
                let _ = tray_icon.set_visible(false);
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}

/// Run the background clipboard monitoring engine
async fn run_background_daemon(active_flag: Arc<AtomicBool>) -> Result<()> {
    info!("Initializing Background Daemon Database...");
    let config_dir = dirs::config_dir().context("Failed to locate config dir")?;
    let db_path = config_dir.join("morsel").join("morsel.db");
    
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let storage_config = StorageConfig {
        db_path: db_path.to_string_lossy().to_string(),
        ..Default::default()
    };

    let storage = Arc::new(SqliteStorage::new(storage_config)?);
    storage.initialize().await?;

    let clipboard = Arc::new(InMemoryClipboard::new());
    let monitor_config = MonitorConfig {
        poll_interval_ms: 250,
        ignore_duplicates: true,
        max_buffer_size: 1000,
        detect_ownership_changes: true,
        max_content_size: 0,
    };

    let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard, monitor_config);
    monitor.start().await?;

    info!("Daemon listener running smoothly.");

    while let Some(event) = receiver.recv().await {
        if !active_flag.load(Ordering::Relaxed) {
            continue;
        }

        if let morsel_clipboard::ClipboardEvent::NewItem(item) = event {
            info!("Captured clipboard item: {}", item.id);
            if let Err(e) = storage.insert(&item).await {
                error!("Failed to store item: {}", e);
            }
        }
    }

    Ok(())
}

/// Spawn the Morsel TUI interactive interface
fn spawn_tui_process() {
    let _ = std::process::Command::new("morsel-tui").spawn();
}

/// Create a default 16x16 RGBA icon for the system tray
fn create_default_tray_icon() -> Vec<u8> {
    let mut icon = Vec::with_capacity(16 * 16 * 4);
    for y in 0..16 {
        for x in 0..16 {
            // Draw a stylish blue/cyan square icon with rounded inner highlight
            let is_border = x == 0 || x == 15 || y == 0 || y == 15;
            if is_border {
                icon.extend_from_slice(&[30, 144, 255, 255]); // Dodger Blue border
            } else {
                icon.extend_from_slice(&[0, 191, 255, 230]);  // Deep Sky Blue center
            }
        }
    }
    icon
}
