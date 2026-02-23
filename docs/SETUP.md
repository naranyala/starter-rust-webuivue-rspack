# Development Setup

## Prerequisites

### Required Tools

- **Rust** (1.75+)
  - Install via [rustup](https://rustup.rs/)
  - Run `rustup update` to get the latest version

- **Bun** (1.39+)
  - Install via: `curl -fsSL https://bun.sh/install | bash`

- **Node.js** (for some tooling)

### Platform-Specific Requirements

#### Linux

- WebKitGTK development libraries
  - Arch Linux: `sudo pacman -S webkit2gtk-4.1`
  - Ubuntu/Debian: `sudo apt install libwebkit2gtk-4.1-dev`

- GCC/Clang for C compilation

#### macOS

- Xcode Command Line Tools: `xcode-select --install`

#### Windows

- Visual Studio Build Tools with C++ support

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd starter-rust-webuivue-rspack

# Install Rust dependencies (first build)
cargo build

# Install frontend dependencies
cd frontend
bun install
cd ..
```

## Environment Configuration

The application is configured via `app.config.toml`:

```toml
[application]
name = "app"
version = "1.0.0"

[logging]
level = "info"
file = "application.log"
append = true

[database]
path = "app.db"

[server]
port = 0  # 0 = auto-assign

[executable]
name = "app"

[build]
create_sample_data = true
```

## Development Workflow

### Running the Application

```bash
# Build and run in development mode
./run.sh

# Build frontend only
bun run build-frontend.js

# Build Rust only
cargo build

# Clean and rebuild
./run.sh --clean
```

### Available Scripts

| Script | Description |
|--------|-------------|
| `./run.sh` | Main build and run script |
| `./build-dist.sh` | Distribution packaging |
| `./build-frontend.js` | Frontend build script |

### Build Options

- `--build-frontend`: Build only the frontend
- `--build-rust`: Build only the Rust backend
- `--build`: Build both frontend and backend
- `--release`: Build release version
- `--clean`: Clean previous build artifacts

## Frontend Development

### Commands

```bash
cd frontend

# Install dependencies
bun install

# Development server with hot reload
bun run dev

# Production build
bun run build

# Run tests
bun test

# Lint and format check
bun run check
```

### Environment Variables

The frontend uses rspack's DefinePlugin for environment variables:

```typescript
// Available in code
import.meta.env.MODE           // 'development' or 'production'
import.meta.env.VITE_APP_VERSION  // App version
```

## Backend Development

### Commands

```bash
# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Watch mode (requires cargo-watch)
cargo watch -x build -x test
```

### Logging

Backend logs are written to:
- Console (stdout)
- File: `application.log`

## Project Structure After Build

After building, the following directories are created:

- `frontend/dist/` - Frontend build output
- `target/debug/` - Debug build (Rust)
- `target/release/` - Release build (Rust)
- `static/` - Copied static assets (js/, css/)
- `app.db` - SQLite database (runtime)
- `application.log` - Application logs (runtime)

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
| Import errors | Check rspack resolve extensions |

## Code Quality

### Frontend (Biome)

```bash
cd frontend

# Check for issues
bun run check

# Auto-fix issues
bun run check --write

# Format code
bun run format --write
```

### Backend (Rust)

```bash
# Clippy lints
cargo clippy

# Format code
cargo fmt
```
