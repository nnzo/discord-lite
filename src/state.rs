use crate::api::{Channel, Guild, Message, User};

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
            error: None,
        }
    }
}
