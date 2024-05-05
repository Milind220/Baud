mod ui;

use baud_core::connection::SerialConnection;
use baud_core::serial::list_available_ports;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tokio;

struct AppState {
    selected_port: Option<String>,
    received_data: Vec<u8>,
    selected_index: usize,
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Set up the terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Get the list of available serial ports
    let available_ports = list_available_ports().unwrap();

    // Create a state to store the selected port and received data
    let mut state = AppState {
        selected_port: None,
        received_data: Vec::new(),
        selected_index: 0,
    };

    // Run the main loop
    loop {
        terminal.draw(|f| {
            // Call the draw_main_layout function from the ui module
            ui::draw_main_layout(f, &available_ports, &state);
        })?;

        // Handle input and events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Up => {
                    if state.selected_index > 0 {
                        state.selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if state.selected_index < available_ports.len() - 1 {
                        state.selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    if let Some(port) = available_ports.get(state.selected_index) {
                        state.selected_port = Some(port.name.clone());
                        // Establish serial connection and receive data
                        let mut connection = SerialConnection::connect(&port.name, 9600).await?;
                        let data = connection.read_data().await?;
                        state.received_data = data;
                    }
                }
                _ => {} // Catch-all pattern to handle the remaining variants
            }
        }
    }

    Ok(())
}

