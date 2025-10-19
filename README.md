# Discord Lite

A lightweight, minimal Discord client built with Rust and Iced GUI framework. This client focuses on the essentials: viewing servers, channels, and messaging - no bloat, no unnecessary features.

## Features

- ✅ Login with Discord token
- ✅ View your servers (guilds)
- ✅ View text channels
- ✅ Read messages
- ✅ Send messages
- ✅ Clean, dark-themed UI
- ✅ No bloatware - just the basics

## Requirements

- Rust 1.70 or higher
- A Discord user token (see below)

## Installation

1. Clone this repository:
```bash
git clone https://github.com/nnzo/discord-lite
cd discord-lite
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

## Getting Your Discord Token

**⚠️ WARNING: Your Discord token is like your password. Never share it with anyone!**

### Method 1: Browser Developer Tools (Recommended)

1. Open Discord in your web browser (discord.com)
2. Press `F12` to open Developer Tools
3. Go to the "Network" tab
4. Refresh the page (`F5`)
5. Look for any request to `discord.com/api`
6. Click on it and go to "Headers"
7. Find the "Authorization" header - this is your token

### Method 2: Application (Windows)

1. Press `Ctrl + Shift + I` in Discord desktop app to open DevTools
2. Go to the "Console" tab
3. Paste this code:
```javascript
(webpackChunkdiscord_app.push([[''],{},e=>{m=[];for(let c in e.c)m.push(e.c[c])}]),m).find(m=>m?.exports?.default?.getToken!==void 0).exports.default.getToken()
```
4. Press Enter - your token will be displayed

**Note:** Using user tokens for automation can violate Discord's Terms of Service. Use this client at your own risk. This is intended for educational purposes.

## Usage

1. Launch the application
2. Enter your Discord token in the login screen
3. Click "Login"
4. Select a server from the left panel
5. Select a channel from the middle panel
6. View messages and send new ones in the right panel

## Architecture

The project is organized into clean, modular components:

- `main.rs` - Application entry point and message handling
- `api.rs` - Discord API client (REST API calls)
- `state.rs` - Application state management
- `ui.rs` - User interface views and layouts

## Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run

# Run tests (if any)
cargo test
```

## Technologies Used

- **Rust** - Systems programming language
- **Iced** - Cross-platform GUI library
- **Tokio** - Async runtime
- **Reqwest** - HTTP client for Discord API
- **Serde** - Serialization/deserialization

## Limitations

This is a **lite** client, which means:

- No voice/video support
- No rich embeds rendering (shows as plain text)
- No notifications
- No user presence/status
- No DMs (currently)
- No reactions support
- No file uploads
- Manual refresh only (no real-time updates via Gateway/WebSocket)

## Future Improvements (Maybe)

- [ ] Add WebSocket support for real-time updates
- [ ] Support for Direct Messages
- [ ] Better message formatting
- [ ] Image preview in chat
- [ ] Auto-refresh messages
- [ ] Search functionality

## Security Notes

- Your token is stored in memory only while the app is running
- The token is never written to disk
- Always be cautious with your Discord token
- Consider using a throwaway/test account for experimentation

## License

MIT License - feel free to use, modify, and distribute as you wish.

## Disclaimer

This project is not affiliated with Discord Inc. Use at your own risk. Automated user accounts (selfbots) may violate Discord's Terms of Service.
