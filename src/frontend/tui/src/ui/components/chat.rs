use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, List, Paragraph};
use crate::app::App;

pub fn chat(app: &App, frame: &mut Frame, rect: Rect) {
    if app.server_selected.is_none() {
        panic!("no server selected")
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // channels
            Constraint::Percentage(60), // chat
            Constraint::Percentage(20), // members
        ])
        .split(rect);

    let server_selected = &app.server_selected.as_ref().unwrap();
    let channel_selected = &app.channel_selected.as_ref().unwrap();

    let channels = List::new(
        server_selected.channels.iter().map(|c| c.clone()).collect::<Vec<String>>()
    ).block(Block::default().borders(Borders::ALL).title("Channels"));

    let channel_name = format!("#{}", &channel_selected.name);
    let messages = List::new(
        channel_selected.messages.iter().map(|m| m.content.clone()).collect::<Vec<String>>()
    ).block(Block::default().borders(Borders::ALL).title(channel_name.to_string()));

    let middle_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0), // chat
            Constraint::Length(3), // input box
        ])
        .split(chunks[1]);
    let input_box = Paragraph::new("...")
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!("Message {}", &channel_name))
        );

    let users = List::new(
        server_selected.members.iter().map(|m| m.clone()).collect::<Vec<String>>()
    ).block(Block::default().borders(Borders::ALL).title("Members"));

    frame.render_widget(channels, chunks[0]);
    frame.render_widget(messages, middle_chunk[0]);
    frame.render_widget(input_box, middle_chunk[1]);
    frame.render_widget(users, chunks[2]);
}