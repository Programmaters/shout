use color_eyre::owo_colors::OwoColorize;
use crate::app::App;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap};
use ratatui::Frame;
use crate::models::screen::{ChatSection, Screen};

pub fn render_chat(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // channels
            Constraint::Percentage(60), // chat
            Constraint::Percentage(20), // members
        ])
        .split(area);

    render_channels(app, frame, chunks[0]);
    render_messages(app, frame, chunks[1]);
    render_members(app, frame, chunks[2]);
}


fn render_channels(app: &App, frame: &mut Frame, area: Rect) {
    let title_color = if app.screen == Screen::Chat(ChatSection::ChannelList) {
        Color::Yellow
    } else {
        Color::Gray
    };
    let title = Line::from("Channels").style(Style::default().fg(title_color));
    let channels = List::new(
        app.channels.iter().map(|c| c.clone().name).collect::<Vec<String>>()
    ).block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(channels, area);
}

fn render_messages(app: &App, frame: &mut Frame, area: Rect) {
    let channel_selected = &app.channel_selected.as_ref().unwrap();
    let channel_name = format!("#{}", &channel_selected.name);
    let logged_user = &app.logged_user.as_ref().unwrap();

    let messages_text: Text = channel_selected.messages.iter()
        .flat_map(|m| {
            let align = if m.sender == logged_user.username { Alignment::Right } else { Alignment::Left };
            let header = Line::from(Span::styled(
                format!(" {} ", m.datetime.format("%H:%M")),
                Style::default().fg(Color::Gray)
            ));
            let message = Line::from(format!( "{}: {} ", m.sender.clone(), m.content.clone()));
            vec![
                header.alignment(align),
                message.alignment(align),
                Line::from(""), // spacer
            ]
        })
        .collect();

    let total_lines = messages_text.lines.len() as u16;
    let middle_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0), // chat
            Constraint::Length(3), // input box
        ])
        .split(area);

    let viewport_height = middle_chunk[0].height.saturating_sub(2);
    let max_scroll_offset = if total_lines > viewport_height {
        total_lines - viewport_height
    } else {
        0
    };

    let effective_scroll_offset = (app.scroll_offset as u16).min(max_scroll_offset);
    let first_visible_line = max_scroll_offset.saturating_sub(effective_scroll_offset);
    let title_color = if app.screen == Screen::Chat(ChatSection::Chat) {
        Color::Yellow
    } else {
        Color::Gray
    };
    let title = Line::from(channel_name.clone()).style(Style::default().fg(title_color));
    let messages = Paragraph::new(messages_text)
        .block(Block::default().borders(Borders::ALL).title(title))
        .scroll((first_visible_line, 0))
        .wrap(Wrap { trim: true });

    let scrollbar_total = max_scroll_offset + 1;
    let mut scrollbar_state = ScrollbarState::new(scrollbar_total as usize)
        .position(first_visible_line as usize);
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .track_symbol(Some("│"));

    let input_field_content = if app.input_field.is_empty() {
        format!("Message {}", &channel_name)
    } else {
        app.input_field.clone()
    };

    let mut input_vec = vec![Span::raw(input_field_content)];
    if app.screen == Screen::Chat(ChatSection::Chat) && !app.input_field.is_empty() {
        let cursor = Span::styled("│", Style::default().bold().add_modifier(Modifier::RAPID_BLINK));
        input_vec.push(cursor);
    }
    let input_text = Line::from(input_vec);
    let input_field = Paragraph::new(input_text)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(messages, middle_chunk[0]);
    frame.render_stateful_widget(scrollbar, middle_chunk[0], &mut scrollbar_state);
    frame.render_widget(input_field, middle_chunk[1]);
}

fn render_members(app: &App, frame: &mut Frame, area: Rect) {
    let channel = app.channel_selected.as_ref().unwrap();
    let title_color = if app.screen == Screen::Chat(ChatSection::MemberList) {
        Color::Yellow
    } else {
        Color::Gray
    };
    let title = Line::from("Members").style(Style::default().fg(title_color));
    let members = List::new(
        channel.members.iter().map(|m| m.clone().username).collect::<Vec<String>>()
    ).block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(members, area);
}
