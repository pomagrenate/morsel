//! # morsel-clipboard
//!
//! Clipboard monitoring and abstraction for the morsel clipboard manager.
//!
//! This crate provides a platform-agnostic interface for clipboard operations,
//! including monitoring clipboard changes and retrieving clipboard content.

use morsel_core::{ClipboardItem, CoreError};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Result type alias for clipboard operations.
pub type ClipboardResult<T> = std::result::Result<T, ClipboardError>;

/// Error types for clipboard operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ClipboardError {
    #[error("Clipboard not available")]
    NotAvailable,

    #[error("Clipboard access denied")]
    AccessDenied,

    #[error("Clipboard content is empty")]
    EmptyContent,

    #[error("Unsupported clipboard format: {0}")]
    UnsupportedFormat(String),

    #[error("Clipboard ownership lost")]
    OwnershipLost,

    #[error("Core error: {0}")]
    CoreError(#[from] CoreError),

    #[error("IO error: {0}")]
    IoError(String),
}

/// Trait for clipboard providers.
///
/// This trait defines the interface for clipboard operations across different
/// platforms. Implementations are provided for each supported platform.
#[async_trait::async_trait]
pub trait ClipboardProvider: Send + Sync {
    /// Get the current clipboard content as text.
    async fn get_text(&self) -> ClipboardResult<String>;

    /// Set the clipboard content as text.
    async fn set_text(&self, text: String) -> ClipboardResult<()>;

    /// Check if the clipboard has content.
    async fn has_content(&self) -> ClipboardResult<bool>;

    /// Get the current clipboard content as a ClipboardItem.
    async fn get_item(&self) -> ClipboardResult<ClipboardItem> {
        let text = self.get_text().await?;
        if text.is_empty() {
            return Err(ClipboardError::EmptyContent);
        }
        Ok(ClipboardItem::new(text))
    }

    /// Clear the clipboard.
    async fn clear(&self) -> ClipboardResult<()> {
        self.set_text(String::new()).await
    }
}

/// Configuration for clipboard monitoring.
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    /// Polling interval in milliseconds.
    pub poll_interval_ms: u64,
    /// Whether to ignore duplicate consecutive clipboard content.
    pub ignore_duplicates: bool,
    /// Maximum number of items to keep in the monitoring buffer.
    pub max_buffer_size: usize,
    /// Whether to detect clipboard ownership changes.
    pub detect_ownership_changes: bool,
    /// Maximum content size to capture (in bytes). 0 means no limit.
    pub max_content_size: usize,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 250, // 250ms default polling
            ignore_duplicates: true,
            max_buffer_size: 1000,
            detect_ownership_changes: true,
            max_content_size: 0, // No limit by default
        }
    }
}

/// Event emitted by the clipboard monitor.
#[derive(Debug, Clone)]
pub enum ClipboardEvent {
    /// New clipboard content detected.
    NewItem(ClipboardItem),
    /// Clipboard was cleared.
    Cleared,
    /// Clipboard ownership changed to another application.
    OwnershipChanged,
    /// Error occurred during clipboard monitoring.
    Error(ClipboardError),
}

/// Clipboard monitor that watches for clipboard changes.
pub struct ClipboardMonitor {
    provider: Arc<dyn ClipboardProvider>,
    config: MonitorConfig,
    last_content: Option<String>,
    sender: mpsc::UnboundedSender<ClipboardEvent>,
    running: Arc<tokio::sync::RwLock<bool>>,
}

impl ClipboardMonitor {
    /// Create a new clipboard monitor.
    pub fn new(
        provider: Arc<dyn ClipboardProvider>,
        config: MonitorConfig,
    ) -> (Self, mpsc::UnboundedReceiver<ClipboardEvent>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let monitor = Self {
            provider,
            config,
            last_content: None,
            sender,
            running: Arc::new(tokio::sync::RwLock::new(false)),
        };
        (monitor, receiver)
    }

    /// Start monitoring the clipboard.
    pub async fn start(&mut self) -> ClipboardResult<()> {
        let mut running = self.running.write().await;
        if *running {
            return Err(ClipboardError::IoError("Monitor already running".to_string()));
        }
        *running = true;
        drop(running);

        info!("Starting clipboard monitor with poll interval: {}ms", self.config.poll_interval_ms);

        let provider = Arc::clone(&self.provider);
        let config = self.config.clone();
        let sender = self.sender.clone();
        let running = Arc::clone(&self.running);
        let mut last_content = self.last_content.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                tokio::time::Duration::from_millis(config.poll_interval_ms),
            );

            loop {
                interval.tick().await;

                // Check if we should stop
                {
                    let r = running.read().await;
                    if !*r {
                        debug!("Clipboard monitor stopping");
                        break;
                    }
                }

                // Get current clipboard content
                match provider.get_text().await {
                    Ok(content) => {
                        // Check content size limit
                        if config.max_content_size > 0 && content.len() > config.max_content_size {
                            warn!("Clipboard content exceeds size limit: {} bytes", content.len());
                            if config.detect_ownership_changes {
                                let _ = sender.send(ClipboardEvent::OwnershipChanged);
                            }
                            continue;
                        }

                        if content.is_empty() {
                            // Clipboard was cleared
                            if last_content.is_some() {
                                debug!("Clipboard cleared");
                                let _ = sender.send(ClipboardEvent::Cleared);
                                last_content = None;
                            }
                            continue;
                        }

                        // Check for duplicates if configured
                        if config.ignore_duplicates {
                            if let Some(ref last) = last_content {
                                if last == &content {
                                    continue;
                                }
                            }
                        }

                        // Detect ownership change (content changed but we didn't set it)
                        if config.detect_ownership_changes && last_content.is_some() {
                            let _ = sender.send(ClipboardEvent::OwnershipChanged);
                        }

                        // Create new item
                        let item = ClipboardItem::new(content.clone());
                        debug!("New clipboard item detected: {}", item.id);

                        if sender.send(ClipboardEvent::NewItem(item)).is_err() {
                            error!("Failed to send clipboard event - receiver dropped");
                            break;
                        }

                        last_content = Some(content);
                    }
                    Err(ClipboardError::UnsupportedFormat(format)) => {
                        warn!("Unsupported clipboard format: {}", format);
                        if config.detect_ownership_changes {
                            let _ = sender.send(ClipboardEvent::OwnershipChanged);
                        }
                    }
                    Err(ClipboardError::OwnershipLost) => {
                        debug!("Clipboard ownership lost");
                        let _ = sender.send(ClipboardEvent::OwnershipChanged);
                    }
                    Err(e) => {
                        warn!("Clipboard error: {}", e);
                        let _ = sender.send(ClipboardEvent::Error(e));
                    }
                }
            }
        });

        Ok(())
    }

    /// Stop monitoring the clipboard.
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
        info!("Clipboard monitor stop requested");
    }

    /// Check if the monitor is currently running.
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

impl Drop for ClipboardMonitor {
    fn drop(&mut self) {
        // The monitor will stop when the running flag is set to false
        // This is handled in the spawned task
    }
}

/// A simple in-memory clipboard provider for testing.
#[derive(Debug, Default)]
pub struct InMemoryClipboard {
    content: Arc<tokio::sync::RwLock<String>>,
}

impl InMemoryClipboard {
    /// Create a new in-memory clipboard.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the content directly (for testing).
    pub async fn set_content_direct(&self, content: String) {
        let mut data = self.content.write().await;
        *data = content;
    }
}

#[async_trait::async_trait]
impl ClipboardProvider for InMemoryClipboard {
    async fn get_text(&self) -> ClipboardResult<String> {
        let data = self.content.read().await;
        Ok(data.clone())
    }

    async fn set_text(&self, text: String) -> ClipboardResult<()> {
        let mut data = self.content.write().await;
        *data = text;
        Ok(())
    }

    async fn has_content(&self) -> ClipboardResult<bool> {
        let data = self.content.read().await;
        Ok(!data.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_in_memory_clipboard() {
        let clipboard = InMemoryClipboard::new();
        
        // Initially empty
        assert!(!clipboard.has_content().await.unwrap());
        assert_eq!(clipboard.get_text().await.unwrap(), "");
        
        // Set content
        clipboard.set_text("test content".to_string()).await.unwrap();
        assert!(clipboard.has_content().await.unwrap());
        assert_eq!(clipboard.get_text().await.unwrap(), "test content");
        
        // Clear content
        clipboard.clear().await.unwrap();
        assert!(!clipboard.has_content().await.unwrap());
    }

    #[tokio::test]
    async fn test_clipboard_monitor_new_item() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig {
            poll_interval_ms: 50,
            ignore_duplicates: true,
            max_buffer_size: 100,
            detect_ownership_changes: true,
            max_content_size: 0,
        };
        
        let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config);
        
        monitor.start().await.unwrap();
        
        // Set clipboard content
        clipboard.set_content_direct("test content".to_string()).await;
        
        // Wait for event
        sleep(Duration::from_millis(100)).await;
        
        let event = receiver.recv().await.unwrap();
        match event {
            ClipboardEvent::NewItem(item) => {
                assert_eq!(item.content, "test content");
            }
            _ => panic!("Expected NewItem event"),
        }
        
        monitor.stop().await;
    }

    #[tokio::test]
    async fn test_clipboard_monitor_ignore_duplicates() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig {
            poll_interval_ms: 50,
            ignore_duplicates: true,
            max_buffer_size: 100,
            detect_ownership_changes: true,
            max_content_size: 0,
        };
        
        let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config);
        
        monitor.start().await.unwrap();
        
        // Set clipboard content
        clipboard.set_content_direct("test content".to_string()).await;
        sleep(Duration::from_millis(100)).await;
        
        // Should receive one event
        let event1 = receiver.recv().await.unwrap();
        assert!(matches!(event1, ClipboardEvent::NewItem(_)));
        
        // Try to receive another event (should timeout since duplicate is ignored)
        let timeout = sleep(Duration::from_millis(100));
        tokio::select! {
            _ = timeout => {
                // Expected - no event received
            }
            event = receiver.recv() => {
                panic!("Unexpected event: {:?}", event);
            }
        }
        
        monitor.stop().await;
    }

    #[tokio::test]
    async fn test_clipboard_monitor_cleared() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig {
            poll_interval_ms: 50,
            ignore_duplicates: true,
            max_buffer_size: 100,
            detect_ownership_changes: true,
            max_content_size: 0,
        };
        
        let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config);
        
        monitor.start().await.unwrap();
        
        // Set clipboard content
        clipboard.set_content_direct("test content".to_string()).await;
        sleep(Duration::from_millis(100)).await;
        
        // Receive the new item event
        let _ = receiver.recv().await.unwrap();
        
        // Clear clipboard
        clipboard.clear().await.unwrap();
        sleep(Duration::from_millis(100)).await;
        
        // Should receive cleared event
        let event = receiver.recv().await.unwrap();
        assert!(matches!(event, ClipboardEvent::Cleared));
        
        monitor.stop().await;
    }

    #[tokio::test]
    async fn test_clipboard_item_creation() {
        let item = ClipboardItem::new("test content".to_string());
        assert_eq!(item.content, "test content");
        assert_eq!(item.content_type, ContentType::Text);
        assert!(!item.is_favorite);
        assert!(item.tags.is_empty());
    }

    #[tokio::test]
    async fn test_clipboard_item_url_detection() {
        let item = ClipboardItem::new("https://example.com".to_string());
        assert_eq!(item.content_type, ContentType::Url);
    }

    #[tokio::test]
    async fn test_clipboard_monitor_ownership_change() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig {
            poll_interval_ms: 50,
            ignore_duplicates: false,
            detect_ownership_changes: true,
            max_buffer_size: 100,
            max_content_size: 0,
        };
        
        let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config);
        
        monitor.start().await.unwrap();
        
        // Set initial content
        clipboard.set_content_direct("first content".to_string()).await;
        sleep(Duration::from_millis(100)).await;
        
        // Receive the new item event
        let _ = receiver.recv().await.unwrap();
        
        // Change content (simulating ownership change)
        clipboard.set_content_direct("second content".to_string()).await;
        sleep(Duration::from_millis(100)).await;
        
        // Should receive ownership changed event
        let event = receiver.recv().await.unwrap();
        assert!(matches!(event, ClipboardEvent::OwnershipChanged));
        
        monitor.stop().await;
    }

    #[tokio::test]
    async fn test_clipboard_monitor_content_size_limit() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig {
            poll_interval_ms: 50,
            ignore_duplicates: false,
            detect_ownership_changes: true,
            max_buffer_size: 100,
            max_content_size: 100, // Limit to 100 bytes
        };
        
        let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config);
        
        monitor.start().await.unwrap();
        
        // Set content within limit
        clipboard.set_content_direct("small content".to_string()).await;
        sleep(Duration::from_millis(100)).await;
        
        let event = receiver.recv().await.unwrap();
        assert!(matches!(event, ClipboardEvent::NewItem(_)));
        
        // Set content exceeding limit
        let large_content = "x".repeat(200);
        clipboard.set_content_direct(large_content).await;
        sleep(Duration::from_millis(100)).await;
        
        // Should receive ownership changed event (not new item)
        let event = receiver.recv().await.unwrap();
        assert!(matches!(event, ClipboardEvent::OwnershipChanged));
        
        monitor.stop().await;
    }

    #[tokio::test]
    async fn test_clipboard_monitor_error_handling() {
        struct FailingClipboard;
        
        #[async_trait::async_trait]
        impl ClipboardProvider for FailingClipboard {
            async fn get_text(&self) -> ClipboardResult<String> {
                Err(ClipboardError::AccessDenied)
            }
            
            async fn set_text(&self, _text: String) -> ClipboardResult<()> {
                Ok(())
            }
            
            async fn has_content(&self) -> ClipboardResult<bool> {
                Ok(false)
            }
        }
        
        let clipboard = Arc::new(FailingClipboard);
        let config = MonitorConfig::default();
        
        let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard, config);
        
        monitor.start().await.unwrap();
        sleep(Duration::from_millis(100)).await;
        
        let event = receiver.recv().await.unwrap();
        assert!(matches!(event, ClipboardEvent::Error(_)));
        
        monitor.stop().await;
    }

    #[tokio::test]
    async fn test_clipboard_monitor_start_stop() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig::default();
        
        let (mut monitor, _receiver) = ClipboardMonitor::new(clipboard.clone(), config);
        
        assert!(!monitor.is_running().await);
        
        monitor.start().await.unwrap();
        assert!(monitor.is_running().await);
        
        monitor.stop().await;
        sleep(Duration::from_millis(100)).await;
        assert!(!monitor.is_running().await);
    }

    #[tokio::test]
    async fn test_clipboard_monitor_double_start() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig::default();
        
        let (mut monitor, _receiver) = ClipboardMonitor::new(clipboard, config);
        
        monitor.start().await.unwrap();
        let result = monitor.start().await;
        assert!(result.is_err());
        
        monitor.stop().await;
    }

    #[tokio::test]
    async fn stress_test_rapid_clipboard_changes() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig {
            poll_interval_ms: 10,
            ignore_duplicates: false,
            detect_ownership_changes: false,
            max_buffer_size: 1000,
            max_content_size: 0,
        };
        
        let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config);
        
        monitor.start().await.unwrap();
        
        // Simulate rapid clipboard changes
        for i in 0..50 {
            clipboard.set_content_direct(format!("content {}", i)).await;
            sleep(Duration::from_millis(5)).await;
        }
        
        sleep(Duration::from_millis(200)).await;
        
        // Count received events
        let mut event_count = 0;
        while let Ok(_) = receiver.try_recv() {
            event_count += 1;
        }
        
        // Should have captured most changes (may miss some due to polling)
        assert!(event_count > 30, "Expected at least 30 events, got {}", event_count);
        
        monitor.stop().await;
    }

    #[tokio::test]
    async fn stress_test_large_clipboard_content() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig {
            poll_interval_ms: 50,
            ignore_duplicates: false,
            detect_ownership_changes: false,
            max_buffer_size: 100,
            max_content_size: 0,
        };
        
        let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config);
        
        monitor.start().await.unwrap();
        
        // Test with various content sizes
        let sizes = vec![100, 1000, 10000, 100000];
        for size in sizes {
            // Drain any pending events
            while receiver.try_recv().is_ok() {}
            
            let content = "x".repeat(size);
            clipboard.set_content_direct(content).await;
            sleep(Duration::from_millis(100)).await;
            
            let event = receiver.recv().await.unwrap();
            match event {
                ClipboardEvent::NewItem(item) => {
                    assert_eq!(item.content.len(), size);
                }
                _ => panic!("Expected NewItem event"),
            }
        }
        
        monitor.stop().await;
    }

    #[tokio::test]
    async fn stress_test_concurrent_monitoring() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig {
            poll_interval_ms: 50,
            ignore_duplicates: false,
            detect_ownership_changes: false,
            max_buffer_size: 100,
            max_content_size: 0,
        };
        
        // Create multiple monitors for the same clipboard
        let mut monitors = Vec::new();
        let mut receivers = Vec::new();
        
        for _ in 0..3 {
            let (monitor, receiver) = ClipboardMonitor::new(clipboard.clone(), config.clone());
            monitors.push(monitor);
            receivers.push(receiver);
        }
        
        // Start all monitors
        for monitor in &mut monitors {
            monitor.start().await.unwrap();
        }
        
        // Change clipboard content
        clipboard.set_content_direct("test content".to_string()).await;
        sleep(Duration::from_millis(200)).await;
        
        // All monitors should receive the event
        for receiver in &mut receivers {
            let event = receiver.recv().await.unwrap();
            assert!(matches!(event, ClipboardEvent::NewItem(_)));
        }
        
        // Stop all monitors
        for monitor in monitors {
            monitor.stop().await;
        }
    }

    #[tokio::test]
    async fn stress_test_monitor_lifecycle() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig {
            poll_interval_ms: 50,
            ignore_duplicates: false,
            detect_ownership_changes: false,
            max_buffer_size: 100,
            max_content_size: 0,
        };
        
        // Test multiple start/stop cycles
        for _ in 0..5 {
            let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config.clone());
            
            monitor.start().await.unwrap();
            clipboard.set_content_direct("test".to_string()).await;
            sleep(Duration::from_millis(100)).await;
            
            let event = receiver.recv().await.unwrap();
            assert!(matches!(event, ClipboardEvent::NewItem(_)));
            
            monitor.stop().await;
            sleep(Duration::from_millis(50)).await;
        }
    }

    #[tokio::test]
    async fn stress_test_buffer_overflow() {
        let clipboard = Arc::new(InMemoryClipboard::new());
        let config = MonitorConfig {
            poll_interval_ms: 10,
            ignore_duplicates: false,
            detect_ownership_changes: false,
            max_buffer_size: 10, // Small buffer
            max_content_size: 0,
        };
        
        let (mut monitor, _receiver) = ClipboardMonitor::new(clipboard.clone(), config);
        
        monitor.start().await.unwrap();
        
        // Generate more events than buffer size
        for i in 0..20 {
            clipboard.set_content_direct(format!("content {}", i)).await;
            sleep(Duration::from_millis(5)).await;
        }
        
        sleep(Duration::from_millis(100)).await;
        
        // Monitor should still be running despite buffer overflow
        assert!(monitor.is_running().await);
        
        monitor.stop().await;
    }
}
