use serde::{Deserialize, Serialize};

const API_BASE: &str = "https://discord.com/api/v10";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    #[serde(default)]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub channel_type: i32,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub author: User,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
struct SendMessagePayload {
    content: String,
}

pub async fn verify_token(token: String) -> Result<User, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/users/@me", API_BASE))
        .header("Authorization", token)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Invalid token: {}", response.status()));
    }

    response
        .json::<User>()
        .await
        .map_err(|e| format!("Failed to parse user: {}", e))
}

pub async fn fetch_guilds(token: String) -> Result<Vec<Guild>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/users/@me/guilds", API_BASE))
        .header("Authorization", token)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch guilds: {}", response.status()));
    }

    response
        .json::<Vec<Guild>>()
        .await
        .map_err(|e| format!("Failed to parse guilds: {}", e))
}

pub async fn fetch_channels(token: String, guild_id: String) -> Result<Vec<Channel>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/guilds/{}/channels", API_BASE, guild_id))
        .header("Authorization", token)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch channels: {}", response.status()));
    }

    let mut channels: Vec<Channel> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse channels: {}", e))?;

    // Filter to only text channels (type 0)
    channels.retain(|c| c.channel_type == 0);

    Ok(channels)
}

pub async fn fetch_messages(token: String, channel_id: String) -> Result<Vec<Message>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "{}/channels/{}/messages?limit=50",
            API_BASE, channel_id
        ))
        .header("Authorization", token)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch messages: {}", response.status()));
    }

    let mut messages: Vec<Message> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse messages: {}", e))?;

    // Discord returns messages in reverse chronological order, so reverse them
    messages.reverse();

    Ok(messages)
}

pub async fn send_message(
    token: String,
    channel_id: String,
    content: String,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let payload = SendMessagePayload { content };

    let response = client
        .post(format!("{}/channels/{}/messages", API_BASE, channel_id))
        .header("Authorization", token)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to send message: {}", response.status()));
    }

    Ok(())
}
