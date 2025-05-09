// src/cli/tui_components.rs

//! # Terminal User Interface (TUI) Components
//!
//! This module is a placeholder for TUI components and applications built
//! using libraries like `ratatui`.
//!
//! Building TUIs typically involves:
//! - Setting up the terminal (raw mode, clearing screen).
//! - An event loop to handle user input (keyboard, mouse) and other events.
//! - A state management system for the TUI application.
//! - Rendering logic that draws widgets to the terminal buffer.
//! - Restoring the terminal state on exit.
//!
//! `ratatui` provides a rich set of widgets (paragraphs, lists, tables, charts, etc.)
//! and a layout system to arrange them. It's backend-agnostic and can work with
//! `crossterm`, `termion`, etc.

// The following is a very conceptual and minimal example structure.
// A real TUI app would be much more involved.

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{io, time::{Duration, Instant}};

struct CounterApp {
    counter: i32,
    should_quit: bool,
}

impl Default for CounterApp {
    fn default() -> Self {
        CounterApp {
            counter: 0,
            should_quit: false,
        }
    }
}

impl CounterApp {
    fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Right => {
                self.counter += 1;
            }
            KeyCode::Char('-') | KeyCode::Left => {
                self.counter -= 1;
            }
            _ => {}
        }
    }
}


/// Runs a simple counter TUI application.
pub fn run_counter_tui_app() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = CounterApp::default();
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if crossterm::event::poll(tick_rate)? {
            if let Event::Key(key_event) = event::read()? {
                // Ensure we only react on key press, not release or repeat
                if key_event.kind == KeyEventKind::Press {
                     app.on_key(key_event.code);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &CounterApp) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(80), // Main content area
            Constraint::Percentage(20), // Help text area
        ])
        .split(size);

    let counter_text = format!("Counter: {}", app.counter);
    let paragraph = Paragraph::new(counter_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Counter App"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);

    let help_text = Paragraph::new("Press '+' or Right Arrow to increment, '-' or Left Arrow to decrement. 'q' or Esc to quit.")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);
    f.render_widget(help_text, chunks[1]);
}

// No direct tests for run_counter_tui_app as it's interactive.
// Unit tests can be added for AppState logic if it becomes more complex.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_app_logic() {
        let mut app = CounterApp::default();
        assert_eq!(app.counter, 0);

        app.on_key(KeyCode::Char('+'));
        assert_eq!(app.counter, 1);

        app.on_key(KeyCode::Right);
        assert_eq!(app.counter, 2);
        
        app.on_key(KeyCode::Char('-'));
        assert_eq!(app.counter, 1);

        app.on_key(KeyCode::Left);
        assert_eq!(app.counter, 0);

        assert!(!app.should_quit);
        app.on_key(KeyCode::Char('q'));
        assert!(app.should_quit);

        let mut app2 = CounterApp::default();
        app2.on_key(KeyCode::Esc);
        assert!(app2.should_quit);
    }
}
