# Overview

This project serves as a starter template for building desktop applications using a hybrid architecture:

- **Rust backend** for performance-critical operations and system integration
- **Vue.js frontend** for reactive UI development
- **WebUI** for embedding the web interface in a native window
- **Rspack** for fast frontend bundling

The application demonstrates a full-stack desktop architecture with bidirectional communication between Rust and JavaScript, SQLite integration, and modern development practices.

## Key Features

- **Plugin-Driven Architecture**: Modular plugin system for both backend and frontend
- **MVVM + Clean Architecture**: Modern software design patterns
- **Error Handling**: "Errors as values" pattern in both backend and frontend
- **Real-time Communication**: WebSocket and WebUI IPC support
- **Database Integration**: SQLite with rusqlite

## Technology Stack

### Backend (Rust)
- **Language**: Rust 1.93+
- **Framework**: WebUI for native window embedding
- **Database**: SQLite with rusqlite
- **Concurrency**: Tokio async runtime
- **WebSockets**: tokio-tungstenite
- **Serialization**: Serde with JSON support
- **Logging**: env_logger
- **Error Handling**: thiserror

### Frontend (Vue.js)
- **Framework**: Vue 3 with Composition API
- **State Management**: Pinia
- **Build Tool**: Rspack (fast Rust-based bundler)
- **Language**: TypeScript
- **Code Quality**: Biome (formatter/linter)

## Project Structure

```
starter-rust-webuivue-rspack/
├── src/                      # Rust backend
│   ├── core/                 # Core utilities (error, plugin)
│   ├── plugins/              # Plugin implementations
│   ├── db/                  # Database layer
│   ├── infrastructure/       # Config, logging, websocket
│   └── main.rs              # Entry point
├── frontend/                 # Vue.js frontend
│   ├── src/
│   │   ├── core/            # Core utilities (error, plugin, connection)
│   │   ├── plugins/          # Feature plugins
│   │   ├── components/       # Vue components
│   │   ├── composables/      # Vue composables
│   │   ├── views/           # View components
│   │   └── ...
│   └── ...
├── docs/                     # Documentation
├── app.config.toml           # Application configuration
└── run.sh                    # Build and run script
```
