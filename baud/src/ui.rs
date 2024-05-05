use crate::AppState;
use baud_core::serial::SerialPortInfo;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn draw_main_layout(f: &mut Frame, available_ports: &[SerialPortInfo], state: &AppState) {
    let size = f.size();

    // Create the main layout
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(size);

    // Create the serial port selection box
    let serial_port_list = List::new(
        available_ports
            .iter()
            .map(|port| ListItem::new(port.name.clone()))
            .collect::<Vec<ListItem>>(),
    )
    .block(Block::default().title("Serial Ports").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));
    f.render_stateful_widget(serial_port_list.clone(), main_layout[0], &mut list_state);

    // Create the data display area
    let data_display = Paragraph::new(String::from_utf8_lossy(&state.received_data))
        .block(Block::default().title("Data").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    // Create the filters and settings area
    let filters_settings = Paragraph::new("Filters and settings will be displayed here")
        .block(
            Block::default()
                .title("Filters & Settings")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White));

    // Create the data input area
    let data_input = Paragraph::new("Enter data to send")
        .block(Block::default().title("Send Data").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    // Render the widgets
    f.render_widget(serial_port_list, main_layout[0]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(main_layout[1]);

    f.render_widget(data_display, right_layout[0]);
    f.render_widget(filters_settings, right_layout[1]);
    f.render_widget(data_input, right_layout[2]);
}

