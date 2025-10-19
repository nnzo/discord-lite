use crate::api::{Channel, Guild, Message, User};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserStatus {
    Online,
    Idle,
    DoNotDisturb,
    Invisible,
}

impl UserStatus {
    pub fn as_str(&self) -> &str {
        match self {
            UserStatus::Online => "online",
            UserStatus::Idle => "idle",
            UserStatus::DoNotDisturb => "dnd",
            UserStatus::Invisible => "invisible",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            UserStatus::Online => "Online",
            UserStatus::Idle => "Idle",
            UserStatus::DoNotDisturb => "Do Not Disturb",
            UserStatus::Invisible => "Invisible",
        }
    }

    pub fn color(&self) -> [f32; 3] {
        match self {
            UserStatus::Online => [0.3, 0.8, 0.3],       // Green
            UserStatus::Idle => [1.0, 0.7, 0.2],         // Yellow/Orange
            UserStatus::DoNotDisturb => [0.9, 0.3, 0.3], // Red
            UserStatus::Invisible => [0.5, 0.5, 0.5],    // Gray
        }
    }
}

pub struct AppState {
    // Authentication
    pub token_input: String,
    pub token: Option<String>,
    pub logged_in: bool,
    pub current_user: Option<User>,

    // Data
    pub guilds: Vec<Guild>,
    pub channels: Vec<Channel>,
    pub messages: Vec<Message>,

    // Selection
    pub selected_guild: Option<String>,
    pub selected_channel: Option<String>,

    // Input
    pub message_input: String,

    // Status
    pub current_status: UserStatus,
    pub status_menu_open: bool,

    // Error handling
    pub error: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            token_input: String::new(),
            token: None,
            logged_in: false,
            current_user: None,
            guilds: Vec::new(),
            channels: Vec::new(),
            messages: Vec::new(),
            selected_guild: None,
            selected_channel: None,
            message_input: String::new(),
            current_status: UserStatus::Online,
            status_menu_open: false,
            error: None,
        }
    }
}
