use crate::api::Message;
use crate::state::AppState;
use crate::Message as AppMessage;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column};
use iced::{Element, Length};

pub fn view(state: &AppState) -> Element<AppMessage> {
    if !state.logged_in {
        return login_view(state);
    }

    main_view(state)
}

fn login_view(state: &AppState) -> Element<AppMessage> {
    let mut content = column![
        text("Discord Lite").size(32),
        text("Login with your Discord token").size(16),
        text_input("Token", &state.token_input)
            .on_input(AppMessage::TokenInputChanged)
            .on_submit(AppMessage::Login)
            .padding(10)
            .width(400),
        button("Login").on_press(AppMessage::Login).padding(10),
    ]
    .spacing(20)
    .padding(40);

    if let Some(error) = &state.error {
        content = content.push(text(error).style(iced::Color::from_rgb(1.0, 0.3, 0.3)));
    }

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}

fn main_view(state: &AppState) -> Element<AppMessage> {
    let left_panel = guild_list(state);
    let middle_panel = channel_list(state);
    let right_panel = chat_view(state);

    let content = row![left_panel, middle_panel, right_panel]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn guild_list(state: &AppState) -> Element<AppMessage> {
    let mut header_column = Column::new().spacing(5).padding(10).width(200);

    if let Some(user) = &state.current_user {
        header_column = header_column.push(
            text(format!("@{}", user.username))
                .size(14)
                .style(iced::Color::from_rgb(0.7, 0.7, 0.7)),
        );
        header_column = header_column.push(text("-----------").size(12));
    }

    header_column = header_column.push(text("Servers").size(18));

    // Create scrollable guild list
    let mut guild_column = Column::new().spacing(5).width(Length::Fill);

    for guild in &state.guilds {
        // Skip guilds with empty or whitespace-only names
        if guild.name.trim().is_empty() {
            eprintln!("Warning: Skipping guild with empty name, id={}", guild.id);
            continue;
        }

        let is_selected = state
            .selected_guild
            .as_ref()
            .map(|id| id == &guild.id)
            .unwrap_or(false);

        let btn = button(text(&guild.name).size(14))
            .on_press(AppMessage::SelectGuild(guild.id.clone()))
            .padding(8)
            .width(Length::Fill);

        let btn = if is_selected {
            btn.style(iced::theme::Button::Primary)
        } else {
            btn.style(iced::theme::Button::Secondary)
        };

        guild_column = guild_column.push(btn);
    }

    let guild_scroll = scrollable(guild_column)
        .height(Length::Fill)
        .width(Length::Fill);

    header_column = header_column.push(guild_scroll);

    container(header_column)
        .width(200)
        .height(Length::Fill)
        .style(iced::theme::Container::Box)
        .into()
}

fn channel_list(state: &AppState) -> Element<AppMessage> {
    let mut header_column = Column::new().spacing(5).padding(10).width(200);

    if state.selected_guild.is_some() {
        let guild_name = state
            .guilds
            .iter()
            .find(|g| Some(&g.id) == state.selected_guild.as_ref())
            .map(|g| g.name.as_str())
            .unwrap_or("Unknown");

        header_column = header_column.push(text(guild_name).size(16));
        header_column = header_column.push(text("-----------").size(12));
        header_column = header_column.push(text("Channels").size(14));

        let mut channel_column = Column::new().spacing(5).width(Length::Fill);

        for channel in &state.channels {
            let is_selected = state
                .selected_channel
                .as_ref()
                .map(|id| id == &channel.id)
                .unwrap_or(false);

            let channel_name = channel.name.as_deref().unwrap_or("Unknown");

            let btn = button(text(format!("# {}", channel_name)).size(14))
                .on_press(AppMessage::SelectChannel(channel.id.clone()))
                .padding(8)
                .width(Length::Fill);

            let btn = if is_selected {
                btn.style(iced::theme::Button::Primary)
            } else {
                btn.style(iced::theme::Button::Secondary)
            };

            channel_column = channel_column.push(btn);
        }

        let channel_scroll = scrollable(channel_column)
            .height(Length::Fill)
            .width(Length::Fill);

        header_column = header_column.push(channel_scroll);
    } else {
        header_column = header_column.push(text("Select a server").size(14));
    }

    container(header_column)
        .width(200)
        .height(Length::Fill)
        .style(iced::theme::Container::Box)
        .into()
}

fn chat_view(state: &AppState) -> Element<AppMessage> {
    if state.selected_channel.is_none() {
        return container(
            text("Select a channel to view messages")
                .size(20)
                .style(iced::Color::from_rgb(0.5, 0.5, 0.5)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    }

    let channel_name = state
        .channels
        .iter()
        .find(|c| Some(&c.id) == state.selected_channel.as_ref())
        .and_then(|c| c.name.as_deref())
        .unwrap_or("Unknown");

    let mut chat_content = Column::new().spacing(10).padding(10).width(Length::Fill);

    // Header
    chat_content = chat_content.push(
        container(text(format!("# {}", channel_name)).size(20))
            .padding(10)
            .width(Length::Fill),
    );

    // Messages
    let mut messages_column = Column::new().spacing(8).padding(10);

    for message in &state.messages {
        let msg_view = message_view(message);
        messages_column = messages_column.push(msg_view);
    }

    let messages_scroll = scrollable(messages_column)
        .height(Length::Fill)
        .width(Length::Fill);

    chat_content = chat_content.push(messages_scroll);

    // Input
    let input_row = row![
        text_input("Type a message...", &state.message_input)
            .on_input(AppMessage::MessageInputChanged)
            .on_submit(AppMessage::SendMessage)
            .padding(10)
            .width(Length::Fill),
        button("Send").on_press(AppMessage::SendMessage).padding(10),
    ]
    .spacing(10)
    .padding(10)
    .width(Length::Fill);

    chat_content = chat_content.push(input_row);

    // Error display
    if let Some(error) = &state.error {
        chat_content = chat_content
            .push(container(text(error).style(iced::Color::from_rgb(1.0, 0.3, 0.3))).padding(10));
    }

    container(chat_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn message_view(message: &Message) -> Element<AppMessage> {
    let author_text = text(format!("{}:", message.author.username))
        .size(14)
        .style(iced::Color::from_rgb(0.4, 0.7, 1.0));

    let content_text = text(&message.content).size(14);

    let msg_column = column![
        row![
            author_text,
            text(format_timestamp(&message.timestamp))
                .size(12)
                .style(iced::Color::from_rgb(0.5, 0.5, 0.5))
        ]
        .spacing(10),
        content_text
    ]
    .spacing(2)
    .padding(5);

    container(msg_column)
        .width(Length::Fill)
        .style(iced::theme::Container::Box)
        .into()
}

fn format_timestamp(timestamp: &str) -> String {
    // Simple timestamp formatting - just show time
    // Discord timestamp format: 2023-01-01T12:34:56.000000+00:00
    if let Some(time_part) = timestamp.split('T').nth(1) {
        if let Some(time) = time_part.split('.').next() {
            return format!(" {}", time);
        }
    }
    String::new()
}
