# Usage Guide

## Quick Start

1. **Launch the application:**
   ```bash
   cargo run --release
   ```

2. **Get your Discord token** (see below)

3. **Login:**
   - Paste your token in the login screen
   - Press Enter or click "Login"

4. **Navigate:**
   - Click a server on the left panel
   - Click a channel in the middle panel
   - View and send messages in the right panel

## Getting Your Discord Token

### Browser Method (Easiest)

1. Open Discord in your browser: https://discord.com/app
2. Press `F12` to open Developer Tools
3. Click the **Console** tab
4. Paste this code and press Enter:

```javascript
(webpackChunkdiscord_app.push([[''],{},e=>{m=[];for(let c in e.c)m.push(e.c[c])}]),m).find(m=>m?.exports?.default?.getToken!==void 0).exports.default.getToken()
```

5. Your token will appear - copy it (it starts with something like `MTA...` or `ODc...`)

### Alternative: Network Tab Method

1. Open Discord in your browser
2. Press `F12` → Go to **Network** tab
3. Refresh the page (`F5`)
4. Type "api" in the filter box
5. Click any request to discord.com/api
6. Look in **Headers** → **Request Headers** → **Authorization**
7. Copy the token value

## Interface Layout

```
┌─────────────┬──────────────┬────────────────────────────┐
│   Servers   │   Channels   │         Chat Area          │
│             │              │                            │
│  [Server 1] │  # general   │  # channel-name            │
│  [Server 2] │  # random    │  ┌──────────────────────┐  │
│  [Server 3] │  # memes     │  │ User: message text   │  │
│             │              │  │ User: message text   │  │
│             │              │  │ User: message text   │  │
│             │              │  └──────────────────────┘  │
│             │              │  [Type message...] [Send]  │
└─────────────┴──────────────┴────────────────────────────┘
```

## Keyboard Shortcuts

- **Enter** in login field → Login
- **Enter** in message field → Send message
- **Ctrl+C** in terminal → Quit application

## Tips

- **Token Security:** Never share your token with anyone
- **First Time:** It may take a few seconds to load all servers
- **Messages:** Only the last 50 messages are loaded per channel
- **Refresh:** Select the channel again to refresh messages
- **Text Only:** Only text channels are shown (no voice channels)

## Common Issues

### "Invalid token" Error
- Check that you copied the complete token
- Make sure there are no extra spaces
- The token might have expired - get a fresh one

### Can't See Servers
- Wait a few seconds for them to load
- Check your internet connection
- Verify you have access to servers on the Discord account

### Messages Not Updating
- This client doesn't auto-refresh
- Click the channel name again to reload messages
- Consider it a feature, not a bug (saves bandwidth!)

## Token Format

Your Discord token should look like one of these:
- `MTA1234567890.XXXXXX.YYYYYYYYYYYYYYYYYYYYYY`
- `ODc1234567890.XXXXXX.YYYYYYYYYYYYYYYYYYYYYY`
- `NzY1234567890.XXXXXX.YYYYYYYYYYYYYYYYYYYYYY`

It typically has three parts separated by dots.

## Security Best Practices

1. ✅ **DO:**
   - Use on your personal account for testing
   - Keep your token private
   - Close the app when not in use
   - Use a test/secondary account if concerned

2. ❌ **DON'T:**
   - Share your token with anyone
   - Commit tokens to Git
   - Use on shared computers
   - Run untrusted code with your token

## API Rate Limits

Discord has rate limits on their API:
- Message sending: ~5 messages per 5 seconds per channel
- If you hit a limit, wait a few seconds and try again

## What's Not Included

This is a **lite** client. The following features are intentionally excluded:

- Voice/Video calls
- Rich embeds (shows as plain text)
- Reactions and emojis
- File uploads/downloads
- Direct Messages (DMs)
- User profiles
- Server settings
- Notifications
- Typing indicators
- Real-time updates (no WebSocket)
- Image previews
- Link previews
- User search
- Message history beyond 50 messages

## Troubleshooting

### Application Won't Start
```bash
# Check Rust installation
cargo --version

# Clean and rebuild
cargo clean
cargo build --release
```

### Network Errors
- Check firewall settings
- Verify Discord isn't blocked
- Try a different network

### Build Errors
- Update Rust: `rustup update`
- Check Cargo.toml for correct dependencies
- Clear target directory: `cargo clean`

## Development Mode

Run in debug mode for more verbose output:
```bash
cargo run
```

Enable debug features:
```bash
RUST_LOG=debug cargo run
```

## Future Features (Maybe)

Ideas for future improvements:
- WebSocket for real-time updates
- DM support
- Message search
- Better error messages
- Configuration file for token storage (encrypted)
- Themes (light mode)
- Custom notification sounds

## Contributing

Feel free to fork and improve! This is a minimal starting point.

## Questions?

This is a learning project - expect bugs and rough edges!