use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

// TODO: Implement the ui 
// TODO: Get list of ports and display them
// TODO: Make it possible to select a port and connect to it
// TODO: Show port data in the right pane
// TODO: Implement a way to send data to the port
// TODO: Implement a way to save the data to a file

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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
        terminal.draw(|_f| {
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



