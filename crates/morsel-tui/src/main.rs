//! # morsel TUI
//!
//! Terminal user interface for the morsel clipboard manager.

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use morsel_clipboard::{ClipboardProvider, InMemoryClipboard};
use morsel_core::ClipboardItem;
use morsel_search::{SearchEngine, SearchQuery};
use morsel_storage::{SqliteStorage, StorageBackend, StorageConfig};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, Level};

/// Application state
struct AppState {
    items: Vec<ClipboardItem>,
    filtered_items: Vec<ClipboardItem>,
    selected_index: usize,
    search_query: String,
    search_engine: SearchEngine,
    storage: Arc<SqliteStorage>,
    clipboard: Arc<InMemoryClipboard>,
    show_help: bool,
    is_loading: bool,
    error_message: Option<String>,
    previous_clipboard: Option<String>,
}

impl AppState {
    fn new(storage: Arc<SqliteStorage>, clipboard: Arc<InMemoryClipboard>) -> Self {
        Self {
            items: Vec::new(),
            filtered_items: Vec::new(),
            selected_index: 0,
            search_query: String::new(),
            search_engine: SearchEngine::new(),
            storage,
            clipboard,
            show_help: false,
            is_loading: true,
            error_message: None,
            previous_clipboard: None,
        }
    }

    async fn refresh_items(&mut self) -> Result<()> {
        self.is_loading = true;
        self.error_message = None;
        
        match self.storage.list().await {
            Ok(items) => {
                self.items = items;
                self.filtered_items = self.items.clone();
                
                // Rebuild search index
                self.search_engine = SearchEngine::new();
                for item in &self.items {
                    self.search_engine.index_item(item.clone());
                }
                
                self.selected_index = 0;
                self.is_loading = false;
                Ok(())
            }
            Err(e) => {
                self.is_loading = false;
                self.error_message = Some(format!("Failed to load items: {}", e));
                Err(e.into())
            }
        }
    }

    fn filter_items(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_items = self.items.clone();
        } else {
            let query = SearchQuery::new(self.search_query.clone())
                .fuzzy(true)
                .limit(100);
            
            if let Ok(results) = self.search_engine.search(&query) {
                self.filtered_items = results.into_iter().map(|s| s.item).collect();
            } else {
                self.filtered_items = Vec::new();
            }
        }
        self.selected_index = 0;
    }

    fn selected_item(&self) -> Option<&ClipboardItem> {
        self.filtered_items.get(self.selected_index)
    }

    fn next_item(&mut self) {
        if !self.filtered_items.is_empty() {
            self.selected_index = (self.selected_index + 1).min(self.filtered_items.len() - 1);
        }
    }

    fn prev_item(&mut self) {
        if !self.filtered_items.is_empty() {
            self.selected_index = self.selected_index.saturating_sub(1);
        }
    }
}

/// Draw the main UI
fn draw_ui(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Search input
            Constraint::Min(0),    // Item list
            Constraint::Length(10), // Item preview
        ])
        .split(f.size());

    // Search input
    let search_block = Block::default()
        .borders(Borders::ALL)
        .title("Search (Ctrl+K)")
        .title_style(Style::default().fg(Color::Cyan));
    
    let search_text = Text::from(vec![Line::from(vec![
        Span::styled("> ", Style::default().fg(Color::Cyan)),
        Span::raw(&state.search_query),
        Span::styled("█", Style::default().fg(Color::Gray)),
    ])]);
    
    let search_paragraph = Paragraph::new(search_text)
        .block(search_block)
        .wrap(Wrap { trim: false });
    
    f.render_widget(search_paragraph, chunks[0]);

    // Item list
    let list_items: Vec<ListItem> = if state.is_loading {
        vec![ListItem::new("Loading clipboard items...")]
    } else if let Some(ref error) = state.error_message {
        vec![ListItem::new(format!("Error: {}", error))]
    } else if state.filtered_items.is_empty() {
        vec![ListItem::new("No clipboard items found. Press 'q' to quit.")]
    } else {
        state.filtered_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let preview = if item.content.len() > 60 {
                    format!("{}...", &item.content[..60])
                } else {
                    item.content.clone()
                };
                
                let style = if i == state.selected_index {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                
                let prefix = if item.is_favorite { "★ " } else { "  " };
                ListItem::new(format!("{}{}", prefix, preview)).style(style)
            })
            .collect()
    };

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Clipboard History ({})", state.filtered_items.len()))
        .title_style(Style::default().fg(Color::Cyan));

    let list = List::new(list_items)
        .block(list_block)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    
    f.render_widget(list, chunks[1]);

    // Item preview
    if state.is_loading {
        let loading_preview = Text::from("Loading...");
        let preview_block = Block::default()
            .borders(Borders::ALL)
            .title("Preview")
            .title_style(Style::default().fg(Color::Cyan));

        let preview_paragraph = Paragraph::new(loading_preview)
            .block(preview_block)
            .wrap(Wrap { trim: true });
        
        f.render_widget(preview_paragraph, chunks[2]);
    } else if let Some(ref error) = state.error_message {
        let error_preview = Text::from(format!("Error: {}", error));
        let preview_block = Block::default()
            .borders(Borders::ALL)
            .title("Preview")
            .title_style(Style::default().fg(Color::Red));

        let preview_paragraph = Paragraph::new(error_preview)
            .block(preview_block)
            .wrap(Wrap { trim: true });
        
        f.render_widget(preview_paragraph, chunks[2]);
    } else if let Some(item) = state.selected_item() {
        let preview_text = Text::from(vec![
            Line::from(vec![
                Span::styled("ID: ", Style::default().fg(Color::Cyan)),
                Span::raw(item.id.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Type: ", Style::default().fg(Color::Cyan)),
                Span::raw(item.content_type.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Size: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{} bytes", item.size)),
            ]),
            Line::from(vec![
                Span::styled("Created: ", Style::default().fg(Color::Cyan)),
                Span::raw(item.created_at.format("%Y-%m-%d %H:%M:%S").to_string()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Content:", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(item.content.chars().take(200).collect::<String>()),
        ]);

        let preview_block = Block::default()
            .borders(Borders::ALL)
            .title("Preview")
            .title_style(Style::default().fg(Color::Cyan));

        let preview_paragraph = Paragraph::new(preview_text)
            .block(preview_block)
            .wrap(Wrap { trim: true });
        
        f.render_widget(preview_paragraph, chunks[2]);
    } else {
        let empty_preview = Text::from("Select an item to preview its content");
        let preview_block = Block::default()
            .borders(Borders::ALL)
            .title("Preview")
            .title_style(Style::default().fg(Color::Cyan));

        let preview_paragraph = Paragraph::new(empty_preview)
            .block(preview_block)
            .wrap(Wrap { trim: true });
        
        f.render_widget(preview_paragraph, chunks[2]);
    }

    // Help overlay
    if state.show_help {
        let help_text = Text::from(vec![
            Line::from("Keyboard Shortcuts:"),
            Line::from(""),
            Line::from("↑/↓ - Navigate items"),
            Line::from("Enter - Paste selected item"),
            Line::from("Esc - Close TUI"),
            Line::from("Ctrl+K - Focus search"),
            Line::from("Delete - Delete item"),
            Line::from("P - Pin/Unpin item"),
            Line::from("? - Toggle help"),
        ]);

        let help_block = Block::default()
            .borders(Borders::ALL)
            .title("Help")
            .title_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(Color::Black));

        let help_paragraph = Paragraph::new(help_text)
            .block(help_block)
            .wrap(Wrap { trim: true });
        
        let area = Rect::new(
            f.size().width / 4,
            f.size().height / 4,
            f.size().width / 2,
            f.size().height / 2,
        );
        
        f.render_widget(help_paragraph, area);
    }
}

/// Run the TUI application
async fn run_app() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = StorageConfig::default();
    let storage = Arc::new(SqliteStorage::new(config)?);
    storage.initialize().await?;

    let clipboard = Arc::new(InMemoryClipboard::new());
    let state = Arc::new(Mutex::new(AppState::new(storage.clone(), clipboard.clone())));
    
    {
        let mut state = state.lock().await;
        // Save current clipboard content
        if let Ok(content) = clipboard.get_text().await {
            state.previous_clipboard = Some(content);
        }
        state.refresh_items().await?;
    }

    let result = run_app_loop(&mut terminal, state.clone()).await;

    // Restore previous clipboard content
    {
        let state = state.lock().await;
        if let Some(ref previous) = state.previous_clipboard {
            if let Err(e) = clipboard.set_text(previous.clone()).await {
                error!("Failed to restore clipboard: {}", e);
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Main application event loop
async fn run_app_loop<B: Backend>(
    terminal: &mut Terminal<B>,
    state: Arc<Mutex<AppState>>,
) -> Result<()> {
    loop {
        {
            let state_guard = state.lock().await;
            terminal.draw(|f| draw_ui(f, &state_guard))?;
        }

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let mut state_guard = state.lock().await;
                
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        return Ok(());
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        if let Some(item) = state_guard.selected_item() {
                            let item_id = item.id;
                            let storage = state_guard.storage.clone();
                            drop(state_guard);
                            if let Ok(mut item) = storage.get(item_id).await {
                                item.set_favorite(!item.is_favorite);
                                if let Err(e) = storage.update(&item).await {
                                    error!("Failed to update item: {}", e);
                                }
                            }
                            let mut state_guard = state.lock().await;
                            state_guard.refresh_items().await?;
                        }
                    }
                    KeyCode::Char('?') => {
                        state_guard.show_help = !state_guard.show_help;
                    }
                    KeyCode::Char(c) => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) && c == 'k' {
                            // Focus search (already focused by default)
                        } else {
                            state_guard.search_query.push(c);
                            state_guard.filter_items();
                        }
                    }
                    KeyCode::Backspace => {
                        state_guard.search_query.pop();
                        state_guard.filter_items();
                    }
                    KeyCode::Up => {
                        state_guard.prev_item();
                    }
                    KeyCode::Down => {
                        state_guard.next_item();
                    }
                    KeyCode::Enter => {
                        if let Some(item) = state_guard.selected_item() {
                            let item_id = item.id;
                            let item_content = item.content.clone();
                            let clipboard = state_guard.clipboard.clone();
                            drop(state_guard);
                            
                            if let Err(e) = clipboard.set_text(item_content).await {
                                error!("Failed to paste item: {}", e);
                            } else {
                                info!("Pasted item: {}", item_id);
                                return Ok(());
                            }
                        }
                    }
                    KeyCode::Delete => {
                        if let Some(item) = state_guard.selected_item() {
                            let item_id = item.id;
                            let storage = state_guard.storage.clone();
                            drop(state_guard);
                            if let Err(e) = storage.delete(item_id).await {
                                error!("Failed to delete item: {}", e);
                            }
                            let mut state_guard = state.lock().await;
                            state_guard.refresh_items().await?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Morsel TUI starting...");

    if let Err(e) = run_app().await {
        error!("TUI error: {}", e);
        return Err(e);
    }

    Ok(())
}
