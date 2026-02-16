# Development Setup

## Prerequisites

### Required Tools

- **Rust** (1.93+)
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

# Install Rust dependencies
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
./run.sh --build-frontend

# Build Rust only
./run.sh --build-rust

# Clean and rebuild
./run.sh --clean
```

### Available Scripts

| Script | Description |
|--------|-------------|
| `./run.sh` | Main build and run script |
| `./build-dist.sh` | Distribution packaging |
| `./post-build.sh` | Post-build processing |

### Build Options

- `--build-frontend`: Build only the frontend
- `--build-rust`: Build only the Rust backend
- `--build`: Build both frontend and backend
- `--release`: Build release version
- `--clean`: Clean previous build artifacts

## Project Structure After Build

After building, the following directories are created:

- `frontend/dist/` - Frontend build output
- `target/debug/` - Debug build (Rust)
- `target/release/` - Release build (Rust)
- `static/` - Copied static assets
- `app.db` - SQLite database (runtime)
- `application.log` - Application logs (runtime)
