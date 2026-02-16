# Building and Running

## Development Mode

### Quick Start

```bash
./run.sh
```

This command will:
1. Check prerequisites (Rust, Bun)
2. Install frontend dependencies (if needed)
3. Build the frontend with Rspack
4. Build the Rust backend with Cargo
5. Run the application

### Step by Step

#### 1. Frontend Build

The frontend is built using Rspack:

```bash
cd frontend
bun install  # Install dependencies
bun run build # Build for production
# or
bun run dev  # Development mode
```

Output is copied to `frontend/dist/` and then to root as `static/`.

#### 2. Backend Build

```bash
cargo build        # Debug build
cargo build --release  # Release build
```

#### 3. Running

```bash
# Development
./target/debug/app

# Release
./target/release/app
```

## Production Build

### Building Release Version

```bash
# Build release
./run.sh --release

# Or manually
cargo build --release
```

### Distribution Packaging

```bash
./build-dist.sh build-release
```

This creates distribution packages in the `dist/` directory.

## Build Configuration

### Frontend (Rspack)

Configuration files:
- `rspack.config.ts` - Production build
- `rspack.config.dev.ts` - Development build

Key options:
- Entry: `src/main.ts`
- Output: `dist/`
- Minification enabled for production

### Backend (Cargo)

Key dependencies:
- `webui-rs` - Native window embedding
- `rusqlite` - SQLite database
- `tokio` - Async runtime
- `tokio-tungstenite` - WebSocket

## Troubleshooting

### Build Fails

1. **Missing dependencies**: Run `cargo fetch` and `bun install`
2. **Outdated toolchain**: Run `rustup update` and update Bun
3. **Clean rebuild**: `./run.sh --clean`

### Runtime Errors

1. **WebUI not found**: Ensure WebUI libraries are installed
2. **Database errors**: Check `app.db` permissions
3. **Port conflicts**: The WebSocket port is auto-assigned

### Common Issues

| Issue | Solution |
|-------|----------|
| Frontend build fails | Check Node.js/Bun version |
| Rust build fails | Run `cargo update` |
| App doesn't start | Check `application.log` |
| Blank window | Verify `frontend/dist/index.html` exists |
