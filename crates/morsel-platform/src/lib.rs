//! # morsel-platform
//!
//! Platform-specific implementations for the morsel clipboard manager.
//!
//! This crate provides platform-specific clipboard providers, hotkey handling,
//! and system integration for Windows, macOS, and Linux.

use std::sync::Arc;

/// Platform detection.
pub fn platform_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "windows"
    }
    #[cfg(target_os = "macos")]
    {
        "macos"
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "unknown"
    }
}

/// Hotkey representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hotkey {
    pub modifiers: Vec<Modifier>,
    pub key: Key,
}

/// Key modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    Ctrl,
    Alt,
    Shift,
    Meta, // Windows key on Windows, Command on macOS
}

/// Key code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Char(char),
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Space,
    Enter,
    Tab,
    Escape,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
}

impl Hotkey {
    /// Parse a hotkey string (e.g., "Ctrl+Shift+V").
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split('+').collect();
        if parts.is_empty() {
            return Err("Empty hotkey".to_string());
        }

        let mut modifiers = Vec::new();
        let mut key = None;

        for part in parts {
            let part = part.trim().to_lowercase();
            match part.as_str() {
                "ctrl" | "control" => modifiers.push(Modifier::Ctrl),
                "alt" => modifiers.push(Modifier::Alt),
                "shift" => modifiers.push(Modifier::Shift),
                "meta" | "cmd" | "win" | "windows" => modifiers.push(Modifier::Meta),
                _ => {
                    if key.is_none() {
                        key = Some(Self::parse_key(&part)?);
                    } else {
                        return Err(format!("Multiple keys in hotkey: {}", s));
                    }
                }
            }
        }

        let key = key.ok_or_else(|| format!("No key in hotkey: {}", s))?;
        Ok(Self { modifiers, key })
    }

    fn parse_key(s: &str) -> Result<Key, String> {
        match s {
            "f1" => Ok(Key::F1),
            "f2" => Ok(Key::F2),
            "f3" => Ok(Key::F3),
            "f4" => Ok(Key::F4),
            "f5" => Ok(Key::F5),
            "f6" => Ok(Key::F6),
            "f7" => Ok(Key::F7),
            "f8" => Ok(Key::F8),
            "f9" => Ok(Key::F9),
            "f10" => Ok(Key::F10),
            "f11" => Ok(Key::F11),
            "f12" => Ok(Key::F12),
            "space" => Ok(Key::Space),
            "enter" | "return" => Ok(Key::Enter),
            "tab" => Ok(Key::Tab),
            "esc" | "escape" => Ok(Key::Escape),
            "backspace" => Ok(Key::Backspace),
            "delete" | "del" => Ok(Key::Delete),
            "insert" | "ins" => Ok(Key::Insert),
            "home" => Ok(Key::Home),
            "end" => Ok(Key::End),
            "pageup" | "pgup" => Ok(Key::PageUp),
            "pagedown" | "pgdn" => Ok(Key::PageDown),
            "up" => Ok(Key::Up),
            "down" => Ok(Key::Down),
            "left" => Ok(Key::Left),
            "right" => Ok(Key::Right),
            _ if s.len() == 1 => Ok(Key::Char(s.chars().next().unwrap())),
            _ => Err(format!("Unknown key: {}", s)),
        }
    }

    /// Convert to string representation.
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        for modifier in &self.modifiers {
            parts.push(match modifier {
                Modifier::Ctrl => "Ctrl",
                Modifier::Alt => "Alt",
                Modifier::Shift => "Shift",
                Modifier::Meta => "Meta",
            });
        }
        let key_str = match self.key {
            Key::Char(c) => c.to_string(),
            Key::F1 => "F1".to_string(),
            Key::F2 => "F2".to_string(),
            Key::F3 => "F3".to_string(),
            Key::F4 => "F4".to_string(),
            Key::F5 => "F5".to_string(),
            Key::F6 => "F6".to_string(),
            Key::F7 => "F7".to_string(),
            Key::F8 => "F8".to_string(),
            Key::F9 => "F9".to_string(),
            Key::F10 => "F10".to_string(),
            Key::F11 => "F11".to_string(),
            Key::F12 => "F12".to_string(),
            Key::Space => "Space".to_string(),
            Key::Enter => "Enter".to_string(),
            Key::Tab => "Tab".to_string(),
            Key::Escape => "Escape".to_string(),
            Key::Backspace => "Backspace".to_string(),
            Key::Delete => "Delete".to_string(),
            Key::Insert => "Insert".to_string(),
            Key::Home => "Home".to_string(),
            Key::End => "End".to_string(),
            Key::PageUp => "PageUp".to_string(),
            Key::PageDown => "PageDown".to_string(),
            Key::Up => "Up".to_string(),
            Key::Down => "Down".to_string(),
            Key::Left => "Left".to_string(),
            Key::Right => "Right".to_string(),
        };
        parts.push(&key_str);
        parts.join("+")
    }
}

impl std::fmt::Display for Hotkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Hotkey callback type.
pub type HotkeyCallback = Arc<dyn Fn() + Send + Sync>;

/// Platform-specific hotkey manager.
pub struct HotkeyManager {
    callbacks: Arc<std::sync::Mutex<std::collections::HashMap<Hotkey, HotkeyCallback>>>,
}

impl HotkeyManager {
    /// Create a new hotkey manager.
    pub fn new() -> Self {
        Self {
            callbacks: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// Register a global hotkey with a callback.
    pub fn register(&self, hotkey: Hotkey, callback: HotkeyCallback) -> Result<(), Box<dyn std::error::Error>> {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.insert(hotkey.clone(), callback);
        
        // Platform-specific registration
        #[cfg(target_os = "windows")]
        {
            self.register_windows(&hotkey)?;
        }
        #[cfg(target_os = "macos")]
        {
            self.register_macos(&hotkey)?;
        }
        #[cfg(target_os = "linux")]
        {
            self.register_linux(&hotkey)?;
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            return Err("Hotkey registration not supported on this platform".into());
        }
        
        Ok(())
    }

    /// Trigger a hotkey callback.
    pub fn trigger(&self, hotkey: &Hotkey) {
        let callbacks = self.callbacks.lock().unwrap();
        if let Some(callback) = callbacks.get(hotkey) {
            callback();
        }
    }

    #[cfg(target_os = "windows")]
    fn register_windows(&self, hotkey: &Hotkey) -> Result<(), Box<dyn std::error::Error>> {
        use windows::Win32::UI::Input::KeyboardAndMouse::*;
        use windows::Win32::Foundation::*;

        let mut modifiers = HOT_KEY_MODIFIERS(0);
        for modifier in &hotkey.modifiers {
            match modifier {
                Modifier::Ctrl => modifiers.0 |= MOD_CONTROL.0,
                Modifier::Alt => modifiers.0 |= MOD_ALT.0,
                Modifier::Shift => modifiers.0 |= MOD_SHIFT.0,
                Modifier::Meta => modifiers.0 |= MOD_WIN.0,
            }
        }

        let vk = match hotkey.key {
            Key::Char(c) => VIRTUAL_KEY(c as u16),
            Key::F1 => VK_F1,
            Key::F2 => VK_F2,
            Key::F3 => VK_F3,
            Key::F4 => VK_F4,
            Key::F5 => VK_F5,
            Key::F6 => VK_F6,
            Key::F7 => VK_F7,
            Key::F8 => VK_F8,
            Key::F9 => VK_F9,
            Key::F10 => VK_F10,
            Key::F11 => VK_F11,
            Key::F12 => VK_F12,
            Key::Space => VK_SPACE,
            Key::Enter => VK_RETURN,
            Key::Tab => VK_TAB,
            Key::Escape => VK_ESCAPE,
            Key::Backspace => VK_BACK,
            Key::Delete => VK_DELETE,
            Key::Insert => VK_INSERT,
            Key::Home => VK_HOME,
            Key::End => VK_END,
            Key::PageUp => VK_PRIOR,
            Key::PageDown => VK_NEXT,
            Key::Up => VK_UP,
            Key::Down => VK_DOWN,
            Key::Left => VK_LEFT,
            Key::Right => VK_RIGHT,
        };

        // Generate a unique ID for this hotkey
        let hotkey_id = self.generate_hotkey_id(hotkey);

        // Register the hotkey
        unsafe {
            RegisterHotKey(HWND::default(), hotkey_id, modifiers, vk.0 as u32)
                .map_err(|_e| format!("Failed to register hotkey: {}", hotkey))?;
        }
        
        Ok(())
    }

    fn generate_hotkey_id(&self, hotkey: &Hotkey) -> i32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        hotkey.hash(&mut hasher);
        (hasher.finish() % 0x7FFFFFFF) as i32
    }

    #[cfg(target_os = "macos")]
    fn register_macos(&self, hotkey: &Hotkey) -> Result<(), Box<dyn std::error::Error>> {
        use cocoa::base::{id, nil};
        use cocoa::foundation::{NSString, NSAutoreleasePool};
        use cocoa::appkit::{NSEvent, NSApp, NSApplicationActivationPolicyAccessory};
        
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        
        // Convert hotkey to macOS format
        let mut cmd_key = false;
        let mut ctrl_key = false;
        let mut alt_key = false;
        let mut shift_key = false;
        
        for modifier in &hotkey.modifiers {
            match modifier {
                Modifier::Ctrl => ctrl_key = true,
                Modifier::Alt => alt_key = true,
                Modifier::Shift => shift_key = true,
                Modifier::Meta => cmd_key = true,
            }
        }
        
        let key_code = match hotkey.key {
            Key::Char(c) => c as u16,
            Key::F1 => 122,
            Key::F2 => 120,
            Key::F3 => 99,
            Key::F4 => 118,
            Key::F5 => 96,
            Key::F6 => 97,
            Key::F7 => 98,
            Key::F8 => 100,
            Key::F9 => 101,
            Key::F10 => 109,
            Key::F11 => 103,
            Key::F12 => 111,
            Key::Space => 49,
            Key::Enter => 36,
            Key::Tab => 48,
            Key::Escape => 53,
            Key::Backspace => 51,
            Key::Delete => 117,
            Key::Insert => 114,
            Key::Home => 115,
            Key::End => 119,
            Key::PageUp => 116,
            Key::PageDown => 121,
            Key::Up => 126,
            Key::Down => 125,
            Key::Left => 123,
            Key::Right => 124,
        };
        
        // Register global hotkey using NSEvent
        unsafe {
            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);
            
            let hotkey_id = self.generate_hotkey_id(hotkey) as i64;
            let event_monitor = NSEvent::addGlobalMonitorForEventsMatchingMask_handler_(
                cocoa::appkit::NSKeyDownMask,
                Some(macos_hotkey_handler as extern "C" fn(*mut objc::runtime::Object, *mut objc::runtime::Object, *mut objc::runtime::Object)),
            );
            
            // Store the event monitor for cleanup (not implemented in this stub)
            let _ = event_monitor;
        }
        
        unsafe { pool.drain() };
        
        Ok(())
    }

#[cfg(target_os = "macos")]
extern "C" fn macos_hotkey_handler(
    _observer: *mut objc::runtime::Object,
    _event: *mut objc::runtime::Object,
    _data: *mut objc::runtime::Object,
) {
    // TODO: Handle hotkey trigger
}

    #[cfg(target_os = "linux")]
    fn register_linux(&self, hotkey: &Hotkey) -> Result<(), Box<dyn std::error::Error>> {
        use x11::xlib::*;
        use x11::keysym::*;
        
        // Open X display
        let display = unsafe { XOpenDisplay(std::ptr::null()) };
        if display.is_null() {
            return Err("Failed to open X display".into());
        }
        
        // Convert hotkey to X11 format
        let mut modifiers = 0u32;
        for modifier in &hotkey.modifiers {
            match modifier {
                Modifier::Ctrl => modifiers |= ControlMask,
                Modifier::Alt => modifiers |= Mod1Mask,
                Modifier::Shift => modifiers |= ShiftMask,
                Modifier::Meta => modifiers |= Mod4Mask, // Super/Windows key
            }
        }
        
        let keysym = match hotkey.key {
            Key::Char(c) => XStringToKeysym(&c.to_string().as_bytes()[0] as *const i8) as u64,
            Key::F1 => XK_F1,
            Key::F2 => XK_F2,
            Key::F3 => XK_F3,
            Key::F4 => XK_F4,
            Key::F5 => XK_F5,
            Key::F6 => XK_F6,
            Key::F7 => XK_F7,
            Key::F8 => XK_F8,
            Key::F9 => XK_F9,
            Key::F10 => XK_F10,
            Key::F11 => XK_F11,
            Key::F12 => XK_F12,
            Key::Space => XK_space,
            Key::Enter => XK_Return,
            Key::Tab => XK_Tab,
            Key::Escape => XK_Escape,
            Key::Backspace => XK_BackSpace,
            Key::Delete => XK_Delete,
            Key::Insert => XK_Insert,
            Key::Home => XK_Home,
            Key::End => XK_End,
            Key::PageUp => XK_Page_Up,
            Key::PageDown => XK_Page_Down,
            Key::Up => XK_Up,
            Key::Down => XK_Down,
            Key::Left => XK_Left,
            Key::Right => XK_Right,
        };
        
        // Get keycode from keysym
        let keycode = unsafe { XKeysymToKeycode(display, keysym as u32) };
        if keycode == 0 {
            unsafe { XCloseDisplay(display) };
            return Err("Failed to get keycode".into());
        }
        
        // Grab the key globally
        let hotkey_id = self.generate_hotkey_id(hotkey);
        let result = unsafe {
            XGrabKey(
                display,
                keycode,
                modifiers as u32,
                XDefaultRootWindow(display),
                1, // owner events
                GrabModeAsync,
                GrabModeAsync,
            )
        };
        
        if result != 0 {
            unsafe { XCloseDisplay(display) };
            return Err("Failed to grab key".into());
        }
        
        // Flush the output buffer
        unsafe { XFlush(display) };
        
        // Note: In a real implementation, you would need to:
        // 1. Store the display and hotkey_id for cleanup
        // 2. Set up an event loop to listen for X events
        // 3. Call XUngrabKey when unregistering
        
        unsafe { XCloseDisplay(display) };
        
        Ok(())
    }
}

impl Default for HotkeyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Platform-specific clipboard provider.
pub struct PlatformClipboard;

impl PlatformClipboard {
    /// Create a new platform clipboard provider.
    pub fn new() -> Arc<dyn morsel_clipboard::ClipboardProvider> {
        // TODO: Implement platform-specific clipboard providers
        // For now, return a placeholder
        Arc::new(PlaceholderClipboard)
    }
}

/// Placeholder clipboard provider for unimplemented platforms.
struct PlaceholderClipboard;

#[async_trait::async_trait]
impl morsel_clipboard::ClipboardProvider for PlaceholderClipboard {
    async fn get_text(&self) -> morsel_clipboard::ClipboardResult<String> {
        Err(morsel_clipboard::ClipboardError::NotAvailable)
    }

    async fn set_text(&self, _text: String) -> morsel_clipboard::ClipboardResult<()> {
        Err(morsel_clipboard::ClipboardError::NotAvailable)
    }

    async fn has_content(&self) -> morsel_clipboard::ClipboardResult<bool> {
        Err(morsel_clipboard::ClipboardError::NotAvailable)
    }
}
