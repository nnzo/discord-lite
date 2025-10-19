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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildFolder {
    #[serde(default)]
    pub guild_ids: Vec<String>,
    #[serde(default)]
    pub id: Option<serde_json::Value>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSettings {
    #[serde(default)]
    pub guild_positions: Vec<String>,
    #[serde(default)]
    pub guild_folders: Vec<GuildFolder>,
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

pub async fn fetch_user_settings(token: String) -> Result<UserSettings, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/users/@me/settings", API_BASE))
        .header("Authorization", token)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch user settings: {}",
            response.status()
        ));
    }

    response
        .json::<UserSettings>()
        .await
        .map_err(|e| format!("Failed to parse user settings: {}", e))
}

pub async fn fetch_guilds(token: String) -> Result<Vec<Guild>, String> {
    let client = reqwest::Client::new();

    // Fetch guilds
    let guilds_response = client
        .get(format!("{}/users/@me/guilds", API_BASE))
        .header("Authorization", token.clone())
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !guilds_response.status().is_success() {
        return Err(format!(
            "Failed to fetch guilds: {}",
            guilds_response.status()
        ));
    }

    let mut guilds: Vec<Guild> = guilds_response
        .json()
        .await
        .map_err(|e| format!("Failed to parse guilds: {}", e))?;

    // Debug: Print guild info
    eprintln!("Fetched {} guilds", guilds.len());
    for guild in &guilds {
        eprintln!("Guild: id={}, name={}", guild.id, guild.name);
    }

    // Fetch user settings to get guild order
    if let Ok(settings) = fetch_user_settings(token).await {
        eprintln!("Guild positions: {:?}", settings.guild_positions);
        eprintln!("Guild folders: {:?}", settings.guild_folders);

        // Build ordered list from guild_folders (newer Discord format)
        let mut ordered_ids: Vec<String> = Vec::new();

        for folder in &settings.guild_folders {
            ordered_ids.extend(folder.guild_ids.clone());
        }

        // If guild_folders is empty, fall back to guild_positions
        if ordered_ids.is_empty() {
            ordered_ids = settings.guild_positions.clone();
        }

        eprintln!("Ordered IDs: {:?}", ordered_ids);

        if !ordered_ids.is_empty() {
            // Sort guilds based on ordered IDs
            guilds.sort_by_key(|guild| {
                ordered_ids
                    .iter()
                    .position(|id| id == &guild.id)
                    .unwrap_or(usize::MAX) // Put guilds not in positions at the end
            });
        }
    } else {
        eprintln!("Failed to fetch user settings, using default order");
    }

    Ok(guilds)
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
