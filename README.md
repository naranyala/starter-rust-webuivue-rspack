# Rust WebUI Vue Rspack Starter

A modern desktop application starter kit combining Rust, WebUI, Vue.js, and Rspack for building cross-platform desktop applications with a web-based UI and native performance.

## Table of Contents

- [Overview](./docs/OVERVIEW.md)
  - Project introduction
  - Key features
  - Technology stack
  - Project structure

- [Architecture](./docs/ARCHITECTURE.md)
  - Backend Clean Architecture
  - Frontend MVVM pattern
  - Communication layer
  - Plugin system
  - Error handling patterns

- [Development Setup](./docs/SETUP.md)
  - Prerequisites
  - Installation
  - Environment configuration
  - Development workflow

- [Building and Running](./docs/BUILD.md)
  - Development mode
  - Production build
  - Build configuration
  - Troubleshooting

- [Potential Improvements](./docs/IMPROVEMENTS.md)
  - Configuration management
  - Error handling
  - Testing strategy
  - Security enhancements
  - And more...

## Quick Start

```bash
# Clone and run
git clone <repository-url>
cd starter-rust-webuivue-rspack
./run.sh
```

## Documentation Index

| Document | Description |
|----------|-------------|
| [Overview](./docs/OVERVIEW.md) | Project overview and features |
| [Architecture](./docs/ARCHITECTURE.md) | Technical architecture details |
| [Setup](./docs/SETUP.md) | Development environment setup |
| [Build](./docs/BUILD.md) | Building and running the app |
| [Improvements](./docs/IMPROVEMENTS.md) | Future enhancement ideas |

## Key Features

- **Plugin-Driven Architecture**: Modular plugin system for both backend and frontend
- **MVVM + Clean Architecture**: Modern software design patterns
- **Error Handling**: "Errors as values" pattern in both backend and frontend
- **Real-time Communication**: WebSocket and WebUI IPC support
- **Database Integration**: SQLite with rusqlite
- **Built-in DevTools**: Collapsible bottom panel for debugging

## Technology Stack

### Backend (Rust)
- **Language**: Rust 1.75+
- **Framework**: WebUI for native window embedding
- **Database**: SQLite with rusqlite
- **Concurrency**: Tokio async runtime
- **WebSockets**: tokio-tungstenite
- **Serialization**: Serde with JSON support
- **Logging**: env_logger with file output
- **Error Handling**: thiserror

### Frontend (Vue.js)
- **Framework**: Vue 3 with Composition API
- **State Management**: Pinia
- **Build Tool**: Rspack (fast Rust-based bundler)
- **Language**: TypeScript
- **Code Quality**: Biome (formatter/linter)
- **Testing**: Bun test

## Development Commands

```bash
# Run the application
./run.sh

# Development server (frontend only)
cd frontend && bun run dev

# Build frontend
bun run build-frontend.js

# Build Rust backend
cargo build

# Run tests
cargo test                    # Backend tests
cd frontend && bun test       # Frontend tests

# Code quality
cd frontend && bun run check   # Lint/format check
```

## Project Structure

```
starter-rust-webuivue-rspack/
├── src/                      # Rust backend
│   ├── core/                 # Core utilities (error, plugin)
│   ├── plugins/              # Plugin implementations
│   ├── commands/            # WebUI command handlers
│   ├── db/                  # Database layer
│   ├── infrastructure/       # Config, logging, websocket, sysinfo
│   └── main.rs              # Entry point
├── frontend/                 # Vue.js frontend
│   ├── src/
│   │   ├── components/      # Vue components
│   │   ├── composables/     # Vue composables
│   │   ├── stores/          # Pinia stores
│   │   ├── services/       # API/webui services
│   │   ├── types/          # TypeScript types
│   │   ├── views/          # Page views
│   │   └── main.ts         # Entry point
│   ├── rspack.config.ts    # Production config
│   └── rspack.config.dev.ts # Dev config
├── docs/                     # Documentation
├── static/                   # Copied static assets
├── app.config.toml           # Application configuration
├── run.sh                   # Build and run script
└── Cargo.toml               # Rust dependencies
```

## License

MIT
