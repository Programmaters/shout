use crate::app::App;
use crate::screens::chat::{ChatScreen, ChatSection};
use crate::ui::utils::datetime::format_datetime;
use crate::ui::utils::popup::popup_area;
use crate::ui::utils::select_state::SelectState;
use crate::ui::widgets::button::Button;
use color_eyre::owo_colors::OwoColorize;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::Margin;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
    Block, Borders, Clear, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Widget, Wrap,
};
use ratatui::Frame;
use crate::ui::widgets::input_field::InputField;
use crate::ui::widgets::radio_button::RadioButton;
use crate::ui::widgets::radio_selection::RadioSelection;

pub fn render_chat(app: &App, chat: &ChatScreen, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // channels
            Constraint::Percentage(60), // chat
            Constraint::Percentage(20), // members
        ])
        .split(area);

    render_channels(app, chat, frame, chunks[0]);
    render_messages(app, chat, frame, chunks[1]);
    render_members(app, chat, frame, chunks[2]);
    render_popup(app, chat, frame, area);
}

fn render_channels(_app: &App, chat: &ChatScreen, frame: &mut Frame, area: Rect) {
    let in_section = chat.section == ChatSection::Channels;
    let select_color = SelectState::from_bool(in_section).to_color();
    let channels = chat
        .channels
        .iter()
        .map(|c| {
            let selected = Some(c.id.clone()) == chat.channel_selected;
            let selected_symbol = if selected { "> " } else { "  " };
            format!("{}# {}", selected_symbol, c.name.clone())
        })
        .collect::<Vec<_>>();

    let mut state = ListState::default();
    let list = List::new(channels)
        .block(
            Block::bordered()
                .title("Channels")
                .borders(Borders::ALL)
                .fg(select_color),
        )
        .highlight_style(Style::new().reversed());

    state.select(chat.channels_index);
    frame.render_stateful_widget(list, area, &mut state);
}

fn render_messages(app: &App, chat: &ChatScreen, frame: &mut Frame, area: Rect) {
    let channel_selected = chat.get_channel();
    let channel_name = format!("#{}", &channel_selected.name);
    let logged_user = &app.logged_user.as_ref().unwrap();
    let in_section = chat.section == ChatSection::Messages;
    let select_color = SelectState::from_bool(in_section).to_color();

    let messages_text: Text = channel_selected
        .messages
        .iter()
        .flat_map(|m| {
            let align = if m.sender == logged_user.id {
                Alignment::Right
            } else {
                Alignment::Left
            };
            let header = Line::from(Span::styled(
                format!(" {} ", format_datetime(m.datetime)),
                Style::default().fg(Color::DarkGray),
            ));
            let sender = app.get_user(m.sender.clone());
            let message = Line::from(vec![
                Span::styled(sender.username, Style::default().bold()),
                Span::raw(format!(": {} ", m.content.clone())),
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
            Constraint::Min(0),    // chat
            Constraint::Length(2), // input box
        ])
        .split(area);

    // scrollbar
    let viewport_height = middle_chunk[0].height.saturating_sub(2);
    let max_scroll_offset = total_lines.saturating_sub(viewport_height);
    let effective_scroll_offset = (chat.scroll_offset as u16).min(max_scroll_offset);
    let first_visible_line = max_scroll_offset.saturating_sub(effective_scroll_offset);
    let scrollbar_total = max_scroll_offset + 1;
    let mut scrollbar_state =
        ScrollbarState::new(scrollbar_total as usize).position(first_visible_line as usize);
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight).track_symbol(Some("│"));

    let messages = Paragraph::new(messages_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(channel_name.clone())
                .fg(select_color),
        )
        .scroll((first_visible_line, 0))
        .wrap(Wrap { trim: true });

    let chat_field = InputField {
        value: chat.input_field.clone(),
        placeholder: Some(format!("Message {}", &channel_name)),
        style: Default::default(),
        block: Block::default()
            .borders(Borders::ALL.difference(Borders::TOP))
            .fg(select_color),
    };

    frame.render_widget(messages, middle_chunk[0]);
    frame.render_stateful_widget(scrollbar, middle_chunk[0], &mut scrollbar_state);
    frame.render_widget(chat_field, middle_chunk[1]);
}

fn render_members(app: &App, chat: &ChatScreen, frame: &mut Frame, area: Rect) {
    let channel = chat.get_channel();
    let in_section = chat.section == ChatSection::Members;
    let select_color = SelectState::from_bool(in_section).to_color();
    let members = channel
        .members
        .iter()
        .enumerate()
        .map(|(idx, m)| {
            let name_modifier = if m.id == app.logged_user.clone().unwrap().id {
                Modifier::BOLD
            } else {
                Modifier::empty()
            };
            let name_fg_color = if Some(idx) == chat.members_index {
                Color::Black
            } else {
                select_color
            };
            let online_status_style = if m.online {
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::SLOW_BLINK)
            } else {
                Style::default()
            };
            Line::from(vec![
                Span::raw(" "),
                Span::styled(
                    m.display_name.clone(),
                    Style::default()
                        .add_modifier(name_modifier)
                        .fg(name_fg_color),
                ),
                Span::raw(" "),
                Span::styled("●", online_status_style),
            ])
        })
        .collect::<Vec<_>>();

    let mut state = ListState::default();
    let list = List::new(members)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Members")
                .fg(select_color),
        )
        .highlight_style(Style::new().bg(Color::White));

    state.select(chat.members_index);
    frame.render_stateful_widget(list, area, &mut state);
}

fn render_popup(_app: &App, chat: &ChatScreen, frame: &mut Frame, area: Rect) {
    if chat.section != ChatSection::Popup {
        return;
    }
    let area = popup_area(area, 50, 70);
    frame.render_widget(Clear, area); // clear area for popup

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // members
            Constraint::Percentage(50), // settings
        ])
        .split(area.inner(Margin::new(1, 1)));

    let members_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // add member
            Constraint::Min(0),    // members
        ])
        .split(chunks[0].inner(Margin::new(1, 1)));

    let settings_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // update name
            Constraint::Length(4), // update privacy
            Constraint::Length(3), // delete channel
        ])
        .split(chunks[1].inner(Margin::new(1, 1)));

    let block = Block::bordered()
        .title("Manage Channel")
        .title_alignment(Alignment::Center)
        .white();
    frame.render_widget(block, area);

    let mut list_state = ListState::default();
    let channel_members = chat
        .get_channel()
        .members
        .iter()
        .map(|m| m.display_name.clone());

    let add_member_field = InputField {
        value: format!("@{}", "".to_string()),
        placeholder: None,
        style: Default::default(),
        block: Block::bordered().title("Add Member").title_alignment(Alignment::Left),
    };

    let members_str = format!("Members ({})", chat.get_channel().members.len());
    let members_list = List::new(channel_members)
        .highlight_style(Style::new().bg(Color::White))
        .block(Block::bordered().title(members_str));

    let update_channel_field = InputField {
        value: format!("#{}", chat.get_channel().name),
        placeholder: None,
        style: Default::default(),
        block: Block::bordered().title("Channel Name").title_alignment(Alignment::Left),
    };

    let privacy_select = RadioSelection {
        options: vec!["Public", "Private"],
        selected_index: 0,
        block: Block::bordered().title("Channel Privacy").title_alignment(Alignment::Left),
    };

    let delete_channel_button = Button {
        label: String::from("Delete Channel"),
        is_pressed: false,
        style: Style::default().fg(Color::LightRed),
    };

    frame.render_widget(add_member_field, members_area[0]);
    frame.render_stateful_widget(members_list, members_area[1], &mut list_state);
    frame.render_widget(update_channel_field, settings_area[0]);
    frame.render_widget(privacy_select, settings_area[1]);
    frame.render_widget(delete_channel_button, settings_area[2]);
}
