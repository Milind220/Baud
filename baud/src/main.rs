mod ui;

use baud_core::connection::SerialConnection;
use baud_core::serial::list_available_ports;
use crossterm::event::{self, Event, KeyCode};
use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::sync::{Arc, Mutex};
use tokio;

struct AppState {
    selected_port: Option<String>,
    received_data: Vec<u8>,
    selected_index: usize,
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Set up the terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Get the list of available serial ports
    let available_ports = list_available_ports().unwrap();

    // Create a state to store the selected port and received data
    let state = Arc::new(Mutex::new(AppState {
        selected_port: None,
        received_data: Vec::new(),
        selected_index: 0,
    }));

    let state_clone = Arc::clone(&state);

    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);

    tokio::spawn(async move {
        while let Some(port_name) = rx.recv().await {
            if let Ok(mut connection) = SerialConnection::connect(&port_name, 9600).await {
                if let Ok(data) = connection.read_data().await {
                    // Update the application state with the received data
                    let mut state = state_clone.lock().unwrap();
                    state.received_data = data;
                }
            }
        }
    });

    // Run the main loop
    loop {
        terminal.draw(|f| {
            // Call the draw_main_layout function from the ui module
            let state = state.lock().unwrap();
            ui::draw_main_layout(f, &available_ports, &state);
        })?;

        // Handle input and events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Up => {
                    let mut state = state.lock().unwrap();
                    if state.selected_index > 0 {
                        state.selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    let mut state = state.lock().unwrap();
                    if state.selected_index < available_ports.len() - 1 {
                        state.selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    let mut state = state.lock().unwrap();
                    if let Some(port) = available_ports.get(state.selected_index) {
                        state.selected_port = Some(port.name.clone());
                        // Send the selected port name to the serial task
                        tx.send(port.name.clone())
                            .await
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                    }
                }
                _ => {} // Catch-all pattern to handle the remaining variants
            }
        }
    }
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
