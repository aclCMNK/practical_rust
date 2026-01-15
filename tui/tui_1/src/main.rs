use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use std::io::{self, stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal - temporarily simplified for debugging
    let mut stdout = stdout();

    // Try to enable raw mode, but continue if it fails
    if let Err(e) = enable_raw_mode() {
        eprintln!("Warning: Could not enable raw mode: {:?}", e);
        eprintln!("This might not work properly in some environments.");
        // Continue anyway for testing
    }

    if let Err(e) = execute!(stdout, EnterAlternateScreen) {
        eprintln!("Warning: Could not enter alternate screen: {:?}", e);
        // Continue anyway
    }

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal);

    // Restore terminal - also handle errors gracefully
    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            ui(f);
        })?;
    }
}

fn ui(f: &mut Frame) {
    let size = f.size();

    // Create a simple paragraph widget
    let greeting = Paragraph::new("Hello, TUI World!")
        .block(Block::default().title("Welcome").borders(Borders::ALL));

    // Render it in the center
    let area = centered_rect(60, 20, size);
    f.render_widget(greeting, area);
}

// Helper function to center a rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
