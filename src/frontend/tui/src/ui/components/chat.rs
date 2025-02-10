use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, Paragraph, Wrap};
use crate::app::App;

pub fn chat(app: &App, frame: &mut Frame, rect: Rect) {
    let server_selected = &app.server_selected.as_ref().unwrap();
    let channel_selected = &app.channel_selected.as_ref().unwrap();
    let channel_name = format!("#{}", &channel_selected.name);
    let logged_user = &app.logged_user.as_ref().unwrap();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // channels
            Constraint::Percentage(60), // chat
            Constraint::Percentage(20), // members
        ])
        .split(rect);

    let channels = List::new(
        server_selected.channels.iter().map(|c| c.clone()).collect::<Vec<String>>()
    ).block(Block::default().borders(Borders::ALL).title("Channels"));

    let messages_text: Text = channel_selected.messages.iter()
        .flat_map(|m| {
            let align = if m.sender == logged_user.username { Alignment::Right } else { Alignment::Left };
            let header = Line::from(Span::styled(m.datetime.format("%H:%M:%S").to_string(), Style::default().fg(Color::Gray)));
            let message = Line::from(format!("{}: {}", m.sender.clone(), m.content.clone()));
            vec![
                header.alignment(align),
                message.alignment(align),
                Line::from("") // spacer
            ]
        })
        .collect();

    let messages = Paragraph::new(messages_text)
        .block(Block::default().borders(Borders::ALL).title(channel_name.clone()))
        .wrap(Wrap { trim: true });

    let middle_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0), // chat
            Constraint::Length(3), // input box
        ])
        .split(chunks[1]);

    let input_box_value = if app.input_box.is_empty() {
        format!("Message {}", &channel_name)
    } else {
        app.input_box.clone()
    };
    let input_box = Paragraph::new(input_box_value).block(Block::default().borders(Borders::ALL));
    let users = List::new(
        server_selected.members.iter().map(|m| m.clone()).collect::<Vec<String>>()
    ).block(Block::default().borders(Borders::ALL).title("Members"));

    frame.render_widget(channels, chunks[0]);
    frame.render_widget(messages, middle_chunk[0]);
    frame.render_widget(input_box, middle_chunk[1]);
    frame.render_widget(users, chunks[2]);
}
