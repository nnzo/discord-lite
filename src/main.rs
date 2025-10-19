use iced::{executor, Application, Command, Element, Settings, Theme};

mod api;
mod state;
mod ui;

use state::AppState;
use ui::view;

pub struct DiscordLite {
    state: AppState,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Login
    TokenInputChanged(String),
    Login,
    LoginResult(Result<api::User, String>),

    // Guild & Channel selection
    GuildsLoaded(Result<Vec<api::Guild>, String>),
    SelectGuild(String),
    ChannelsLoaded(Result<Vec<api::Channel>, String>),
    SelectChannel(String),

    // Messages
    MessagesLoaded(Result<Vec<api::Message>, String>),
    MessageInputChanged(String),
    SendMessage,
    MessageSent(Result<(), String>),

    // Status
    ToggleStatusMenu,
    ChangeStatus(state::UserStatus),
    StatusChanged(Result<(), String>),
}

impl Application for DiscordLite {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            DiscordLite {
                state: AppState::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Discord Lite")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TokenInputChanged(token) => {
                self.state.token_input = token;
                Command::none()
            }

            Message::Login => {
                let token = self.state.token_input.clone();
                Command::perform(api::verify_token(token.clone()), move |result| {
                    Message::LoginResult(result.map(|user| user))
                })
            }

            Message::LoginResult(Ok(user)) => {
                self.state.logged_in = true;
                self.state.current_user = Some(user);
                self.state.token = Some(self.state.token_input.clone());

                let token = self.state.token.clone().unwrap();
                Command::perform(api::fetch_guilds(token), Message::GuildsLoaded)
            }

            Message::LoginResult(Err(e)) => {
                self.state.error = Some(format!("Login failed: {}", e));
                Command::none()
            }

            Message::GuildsLoaded(Ok(guilds)) => {
                self.state.guilds = guilds;
                self.state.error = None;
                Command::none()
            }

            Message::GuildsLoaded(Err(e)) => {
                self.state.error = Some(format!("Failed to load guilds: {}", e));
                Command::none()
            }

            Message::SelectGuild(guild_id) => {
                self.state.selected_guild = Some(guild_id.clone());
                self.state.selected_channel = None;
                self.state.messages.clear();

                let token = self.state.token.clone().unwrap();
                Command::perform(
                    api::fetch_channels(token, guild_id),
                    Message::ChannelsLoaded,
                )
            }

            Message::ChannelsLoaded(Ok(channels)) => {
                self.state.channels = channels;
                self.state.error = None;
                Command::none()
            }

            Message::ChannelsLoaded(Err(e)) => {
                self.state.error = Some(format!("Failed to load channels: {}", e));
                Command::none()
            }

            Message::SelectChannel(channel_id) => {
                self.state.selected_channel = Some(channel_id.clone());

                let token = self.state.token.clone().unwrap();
                Command::perform(
                    api::fetch_messages(token, channel_id),
                    Message::MessagesLoaded,
                )
            }

            Message::MessagesLoaded(Ok(messages)) => {
                self.state.messages = messages;
                self.state.error = None;
                Command::none()
            }

            Message::MessagesLoaded(Err(e)) => {
                self.state.error = Some(format!("Failed to load messages: {}", e));
                Command::none()
            }

            Message::MessageInputChanged(text) => {
                self.state.message_input = text;
                Command::none()
            }

            Message::SendMessage => {
                if self.state.message_input.trim().is_empty() {
                    return Command::none();
                }

                let token = self.state.token.clone().unwrap();
                let channel_id = self.state.selected_channel.clone().unwrap();
                let content = self.state.message_input.clone();

                self.state.message_input.clear();

                Command::perform(
                    api::send_message(token.clone(), channel_id.clone(), content),
                    move |result| Message::MessageSent(result),
                )
            }

            Message::MessageSent(Ok(())) => {
                // Refresh messages after sending
                let token = self.state.token.clone().unwrap();
                let channel_id = self.state.selected_channel.clone().unwrap();

                Command::perform(
                    api::fetch_messages(token, channel_id),
                    Message::MessagesLoaded,
                )
            }

            Message::MessageSent(Err(e)) => {
                self.state.error = Some(format!("Failed to send message: {}", e));
                Command::none()
            }

            Message::ToggleStatusMenu => {
                self.state.status_menu_open = !self.state.status_menu_open;
                Command::none()
            }

            Message::ChangeStatus(new_status) => {
                self.state.status_menu_open = false;
                self.state.current_status = new_status;

                let token = self.state.token.clone().unwrap();
                Command::perform(api::set_status(token, new_status), Message::StatusChanged)
            }

            Message::StatusChanged(Ok(())) => {
                self.state.error = None;
                Command::none()
            }

            Message::StatusChanged(Err(e)) => {
                self.state.error = Some(format!("Failed to change status: {}", e));
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        view(&self.state)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() -> iced::Result {
    DiscordLite::run(Settings {
        window: iced::window::Settings {
            size: [1200.0, 800.0].into(),
            ..Default::default()
        },
        ..Default::default()
    })
}
