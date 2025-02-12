use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use rpassword::read_password;
use std::{io, panic, process::exit};

/// Hardcoded password (for demonstration purposes)
const CORRECT_PASSWORD: &str = "123";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Request password
    println!("enter master password:");
    let password = read_password().expect("Failed to read password");

    // Step 2: Check password
    if password != CORRECT_PASSWORD {
        println!("incorrect password...");
        exit(1);
    }

    println!("launching...");

    // Step 3: Start the TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Ensure terminal restores on panic
    let _guard = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        run_tui(&mut terminal).unwrap();
    }));

    // Restore terminal state after exiting
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}

/// Runs the TUI interface
fn run_tui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut selected = 0;
    let mut selected_domain = 0;
    let items = ["display password", "add new password", "delete password"];
    let domains = ["Discord", "Gmail", "Steam"];
    let mut selected_block = 0;
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)]) // ✅ Fixed constraints
                .split(size);

            let list_items: Vec<ListItem> = items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == selected {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(item.to_string()).style(style) // ✅ Fixed ownership issue
                })
                .collect();

            let list = List::new(list_items)
                .block(Block::default().title("pwd mngr").borders(Borders::ALL));
            let list_domains: Vec<ListItem> = domains
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == selected_domain {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(item.to_string()).style(style) // ✅ Fixed ownership issue
                })
                .collect();

            let info_block = List::new(list_domains).block(
                Block::default()
                    .title("domains")
                    .borders(Borders::ALL)
                    .style(Style::default()),
            );

            f.render_widget(list, chunks[0]);
            f.render_widget(info_block, chunks[1]);
        })?;

        // Handle user input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down => {
                    if selected_block == 0 && selected < items.len() - 1 {
                        selected += 1;
                    }
                    if selected_block == 1 && selected_domain < domains.len() - 1 {
                        selected_domain += 1;
                    }
                }
                KeyCode::Up => {
                    if selected_block == 0 && selected > 0 {
                        selected -= 1;
                    }
                    if selected_block == 1 && selected_domain > 0 {
                        selected_domain -= 1;
                    }
                }
                KeyCode::Right => {
                    if selected_block == 0 {
                        selected_block += 1;
                    }
                }
                KeyCode::Left => {
                    if selected_block == 1 {
                        selected_block -= 1;
                    }
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    Ok(())
}
