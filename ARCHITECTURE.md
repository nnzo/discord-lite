# Architecture Documentation

## Project Structure

```
discord-lite/
├── src/
│   ├── main.rs      # Application entry point, message handling
│   ├── api.rs       # Discord REST API client
│   ├── state.rs     # Application state management
│   └── ui.rs        # User interface views and components
├── Cargo.toml       # Dependencies and project metadata
├── README.md        # User-facing documentation
├── USAGE.md         # Detailed usage instructions
└── ARCHITECTURE.md  # This file
```

## Core Components

### 1. Main Application (`main.rs`)

**Purpose:** Entry point and message routing

**Key Structures:**
- `DiscordLite` - Main application struct implementing `iced::Application`
- `Message` - Enum containing all possible UI events and actions

**Responsibilities:**
- Initialize the application
- Handle user interactions via the `update()` method
- Route commands to the API layer
- Manage application state transitions
- Render UI via the `view()` method

**Message Flow:**
```
User Action → Message Event → update() → API Call → Result Message → State Update → UI Re-render
```

### 2. API Layer (`api.rs`)

**Purpose:** Discord REST API communication

**Key Structures:**
- `User` - Discord user information
- `Guild` - Server information
- `Channel` - Channel information
- `Message` - Chat message data

**API Endpoints Used:**
- `GET /users/@me` - Verify token and get current user
- `GET /users/@me/guilds` - Fetch user's servers
- `GET /guilds/{guild_id}/channels` - Fetch channels in a server
- `GET /channels/{channel_id}/messages` - Fetch messages (limit: 50)
- `POST /channels/{channel_id}/messages` - Send a message

**Error Handling:**
- All functions return `Result<T, String>`
- Network errors are caught and converted to user-friendly messages
- HTTP status codes are checked and reported

### 3. State Management (`state.rs`)

**Purpose:** Centralized application state

**State Fields:**
```rust
AppState {
    // Authentication
    token_input: String,           // User input for token
    token: Option<String>,         // Authenticated token
    logged_in: bool,               // Login status
    current_user: Option<User>,    // Current user info

    // Data Collections
    guilds: Vec<Guild>,            // All user's servers
    channels: Vec<Channel>,        // Channels in selected server
    messages: Vec<Message>,        // Messages in selected channel

    // UI Selection State
    selected_guild: Option<String>,   // Currently selected server ID
    selected_channel: Option<String>, // Currently selected channel ID

    // Input State
    message_input: String,         // Current message being typed

    // Error State
    error: Option<String>,         // Error message to display
}
```

**State Transitions:**
1. Initial → Login Screen
2. Login → Fetch Guilds → Guild List
3. Select Guild → Fetch Channels → Channel List
4. Select Channel → Fetch Messages → Chat View
5. Send Message → Refresh Messages

### 4. UI Layer (`ui.rs`)

**Purpose:** Render user interface using Iced widgets

**View Hierarchy:**
```
view()
├── login_view()                    (if not logged in)
│   ├── Title text
│   ├── Token input field
│   ├── Login button
│   └── Error display
│
└── main_view()                     (if logged in)
    ├── guild_list()
    │   ├── Username display
    │   └── Server buttons
    │
    ├── channel_list()
    │   ├── Server name header
    │   └── Channel buttons
    │
    └── chat_view()
        ├── Channel name header
        ├── Messages scroll area
        │   └── message_view() (for each message)
        ├── Message input field
        └── Send button
```

**Styling:**
- Dark theme by default
- Primary buttons for selected items
- Secondary buttons for unselected items
- Color-coded text (usernames, timestamps, errors)

## Data Flow

### Login Flow
```
1. User enters token
   └→ TokenInputChanged message → Update state.token_input

2. User clicks Login
   └→ Login message
      └→ api::verify_token()
         ├→ Success → LoginResult(Ok(user))
         │   ├→ Set logged_in = true
         │   ├→ Store user info
         │   └→ api::fetch_guilds()
         │      └→ GuildsLoaded(Ok(guilds))
         │         └→ Display guild list
         │
         └→ Error → LoginResult(Err(e))
            └→ Display error message
```

### Navigation Flow
```
1. User selects server
   └→ SelectGuild(guild_id)
      ├→ Set selected_guild
      ├→ Clear selected_channel and messages
      └→ api::fetch_channels(guild_id)
         └→ ChannelsLoaded(Ok(channels))
            └→ Display channel list

2. User selects channel
   └→ SelectChannel(channel_id)
      ├→ Set selected_channel
      └→ api::fetch_messages(channel_id)
         └→ MessagesLoaded(Ok(messages))
            └→ Display messages
```

### Messaging Flow
```
1. User types message
   └→ MessageInputChanged(text)
      └→ Update state.message_input

2. User sends message
   └→ SendMessage
      ├→ Validate (not empty)
      ├→ Clear input field
      └→ api::send_message(token, channel_id, content)
         ├→ Success → MessageSent(Ok(()))
         │   └→ api::fetch_messages() to refresh
         │      └→ MessagesLoaded(Ok(messages))
         │
         └→ Error → MessageSent(Err(e))
            └→ Display error message
```

## Async Architecture

**Runtime:** Tokio (full features)

**Pattern:** Iced's built-in async Command system
- Commands are spawned by `update()` method
- Run asynchronously without blocking UI
- Results are sent back as messages
- UI updates when results arrive

**Example:**
```rust
Command::perform(
    api::fetch_guilds(token),  // Async function
    Message::GuildsLoaded      // Result handler
)
```

## Error Handling Strategy

**Levels:**
1. **Network Errors:** Caught by reqwest, converted to String
2. **HTTP Errors:** Status codes checked, meaningful messages
3. **Parse Errors:** JSON deserialization failures
4. **User Errors:** Validation (empty messages, etc.)

**Display:**
- Errors stored in `state.error: Option<String>`
- Displayed in UI with red text
- Cleared on successful operations

## Security Considerations

**Token Storage:**
- Token stored in memory only (never written to disk)
- Cleared when application closes
- Not logged or printed

**API Communication:**
- Uses HTTPS (enforced by Discord)
- Token sent in Authorization header
- No token in URL parameters

**Best Practices:**
- Input validation before API calls
- No sensitive data in error messages
- Minimal logging in release builds

## Performance Optimizations

**Message Loading:**
- Limited to 50 messages per request (Discord's recommended limit)
- Messages loaded only when channel selected
- No automatic polling (reduces bandwidth)

**UI Rendering:**
- Iced's reactive system only re-renders on state changes
- Scrollable containers for large lists
- Lazy rendering of off-screen content (built into Iced)

**Memory:**
- Old messages cleared when switching channels
- No caching of historical data
- Minimal state kept in memory

## Dependencies

### Core Dependencies
- **iced** (0.12): GUI framework
- **tokio** (1.35): Async runtime
- **reqwest** (0.11): HTTP client
- **serde** (1.0): Serialization/deserialization
- **serde_json** (1.0): JSON parsing

### Why These Choices?
- **Iced:** Cross-platform, reactive, Rust-native GUI
- **Tokio:** Industry-standard async runtime
- **Reqwest:** Simple, reliable HTTP client with async support
- **Serde:** De-facto standard for JSON in Rust

## Limitations & Trade-offs

**No WebSocket:**
- Simpler implementation
- Less resource usage
- Manual refresh required
- Trade-off: Real-time updates for simplicity

**No Database:**
- Stateless design
- No message history
- Fresh data on each session
- Trade-off: Persistence for lightweight operation

**REST API Only:**
- Discord recommends Gateway for bots
- User tokens work with REST API
- Sufficient for basic messaging
- Trade-off: Advanced features for simplicity

**Single-threaded UI:**
- Iced's standard model
- API calls are async but UI updates are sequential
- Acceptable for this use case
- Trade-off: Simplicity for maximum performance

## Future Architecture Improvements

### Potential Enhancements:

1. **WebSocket Support:**
   - Add Gateway connection
   - Real-time message updates
   - Presence information
   - Requires: `tokio-tungstenite` crate

2. **Local Caching:**
   - Cache messages in SQLite
   - Faster channel switching
   - Offline viewing
   - Requires: `rusqlite` crate

3. **Modular UI:**
   - Split ui.rs into multiple files
   - Separate components for each view
   - Better maintainability

4. **Plugin System:**
   - Allow custom message formatters
   - Theme plugins
   - Extension API

5. **Configuration:**
   - Config file support
   - Token encryption at rest
   - User preferences
   - Requires: `config`, `keyring` crates

## Testing Strategy

### Current State:
- No automated tests (MVP phase)
- Manual testing via UI

### Recommended Tests:

```rust
// Unit Tests
#[cfg(test)]
mod tests {
    // Test state transitions
    // Test message parsing
    // Test URL formatting
}

// Integration Tests
#[test]
fn test_api_endpoints() {
    // Mock Discord API
    // Test error handling
}
```

## Building & Deployment

**Debug Build:**
```bash
cargo build
# Faster compilation, larger binary, includes debug symbols
```

**Release Build:**
```bash
cargo build --release
# Optimized, smaller binary, no debug symbols
```

**Target Platforms:**
- Windows (tested)
- macOS (should work)
- Linux (should work)

Iced is cross-platform, so this should work on all major platforms.

## Code Style & Conventions

**Naming:**
- Snake_case for functions and variables
- PascalCase for structs and enums
- SCREAMING_SNAKE_CASE for constants

**Error Messages:**
- User-friendly descriptions
- Include context (what failed)
- No technical jargon in UI errors

**Comments:**
- Document complex logic
- Explain why, not what
- Keep comments up-to-date

## Version History

**v0.1.0 - Initial Release**
- Basic messaging functionality
- Login with token
- Server and channel navigation
- Send and receive messages
- Dark theme UI

---

*Last Updated: 2024*