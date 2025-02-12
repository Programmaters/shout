use color_eyre::owo_colors::OwoColorize;
use crossterm::style::Attribute::Bold;
use crate::app::App;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap};
use ratatui::Frame;
use crate::models::screen::{ChatSection, Screen};
use crate::ui::utils::datetime::format_datetime;
use crate::ui::utils::select_state::SelectState;

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
    let in_section = app.screen == Screen::Chat(ChatSection::Channels);
    let select_color = SelectState::from_bool(in_section).to_color();
    let channels = app.channels
        .iter()
        .map(|c| { format!(" #{}", c.name.clone()) })
        .collect::<Vec<_>>();

    let mut state = ListState::default();
    let title = Line::from("Channels").style(Style::default().fg(select_color));
    let list = List::new(channels)
        .block(Block::bordered().title(title).borders(Borders::ALL))
        .highlight_style(Style::new().reversed())
        .repeat_highlight_symbol(true);

    state.select(app.channel_index);
    frame.render_stateful_widget(list, area, &mut state);
}

fn render_messages(app: &App, frame: &mut Frame, area: Rect) {
    let channel_selected = app.get_channel();
    let channel_name = format!("#{}", &channel_selected.name);
    let logged_user = &app.logged_user.as_ref().unwrap();
    let in_section = app.screen == Screen::Chat(ChatSection::Messages);
    let select_color = SelectState::from_bool(in_section).to_color();

    let messages_text: Text = channel_selected.messages.iter()
        .flat_map(|m| {
            let align = if m.sender == logged_user.id { Alignment::Right } else { Alignment::Left };
            let header = Line::from(Span::styled(
                format!(" {} ", format_datetime(m.datetime)),
                Style::default().fg(Color::DarkGray),
            ));
            let message = Line::from(
                vec![
                    Span::styled(m.sender.clone(), Style::default().bold()),
                    Span::raw(format!( ": {} ", m.content.clone()))
                ]);
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
            Constraint::Length(2), // input box
        ])
        .split(area);

    // scrollbar
    let viewport_height = middle_chunk[0].height.saturating_sub(2);
    let max_scroll_offset = total_lines.saturating_sub(viewport_height);
    let effective_scroll_offset = (app.scroll_offset as u16).min(max_scroll_offset);
    let first_visible_line = max_scroll_offset.saturating_sub(effective_scroll_offset);
    let scrollbar_total = max_scroll_offset + 1;
    let mut scrollbar_state = ScrollbarState::new(scrollbar_total as usize).position(first_visible_line as usize);
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight).track_symbol(Some("│"));


    let title = Line::from(channel_name.clone()).style(Style::default().fg(select_color));
    let messages = Paragraph::new(messages_text)
        .block(Block::default().borders(Borders::ALL).title(title))
        .scroll((first_visible_line, 0))
        .wrap(Wrap { trim: true });

    let input_field_content = if app.input_field.is_empty() {
        format!("Message {}", &channel_name)
    } else {
        app.input_field.clone()
    };
    let input_field = Paragraph::new(
        Span::styled(input_field_content, Style::default()))
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM));

    frame.render_widget(messages, middle_chunk[0]);
    frame.render_stateful_widget(scrollbar, middle_chunk[0], &mut scrollbar_state);
    frame.render_widget(input_field, middle_chunk[1]);
}

fn render_members(app: &App, frame: &mut Frame, area: Rect) {
    let channel = app.get_channel();
    let in_section = app.screen == Screen::Chat(ChatSection::Members);
    let select_color = SelectState::from_bool(in_section).to_color();
    let members = channel.members
        .iter()
        .enumerate()
        .map(|(idx, m)| {
            let name_modifier = if m.id == app.logged_user.clone().unwrap().id { Modifier::BOLD } else { Modifier::empty() };
            let name_fg_color = if Some(idx) == app.members_index { Color::Black } else { Color::White };
            let online_status_modifier = if m.online { Modifier::SLOW_BLINK } else { Modifier::empty() };
            let online_status_color = if m.online { Color::LightGreen } else { Color::DarkGray };

            Line::from(vec![
                Span::raw(" "),
                Span::styled(
                    m.display_name.clone(),
                    Style::default()
                        .add_modifier(name_modifier)
                        .fg(name_fg_color),
                ),
                Span::raw(" "),
                Span::styled(
                    "●",
                    Style::default()
                        .add_modifier(online_status_modifier)
                        .fg(online_status_color),
                ),
            ])
        })
        .collect::<Vec<_>>();

    let mut state = ListState::default();
    let title = Line::from("Members").style(Style::default().fg(select_color));
    let list = List::new(members)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::new().bg(Color::White));

    state.select(app.members_index);
    frame.render_stateful_widget(list, area, &mut state);
}
