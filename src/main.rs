use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set up a channel to listen for SIGINT (Ctrl+C)
    let ctrl_c_signal = ctrl_c();
    tokio::spawn(async move {
        ctrl_c_signal.await.expect("Failed to listen for Ctrl+C");
        disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
        std::process::exit(0);
    });

    // Run the TUI app
    if let Err(e) = run_app(&mut terminal).await {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        eprintln!("Error: {}", e);
    }

    // Restore the terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| {
            // Implement layout and widgets here
        })?;

        // Handle input

        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                // catch Ctrl+C
                KeyCode::Char('c') if event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}



