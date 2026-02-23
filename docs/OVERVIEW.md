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
- **Built-in DevTools**: Collapsible bottom panel for debugging frontend and backend

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
- **DevTools**: Custom bottom panel

## Project Structure

```
starter-rust-webuivue-rspack/
├── src/                      # Rust backend
│   ├── core/                 # Core utilities (error, plugin)
│   ├── plugins/              # Plugin implementations
│   │   ├── database/        # Database plugin
│   │   ├── system_info/      # System info plugin
│   │   └── window_tracking/  # Window tracking plugin
│   ├── commands/            # WebUI command handlers
│   │   ├── counter.rs       # Counter example
│   │   ├── sysinfo.rs       # System info commands
│   │   ├── ui.rs            # UI commands
│   │   └── window_state.rs  # Window state commands
│   ├── db/                  # Database layer
│   │   ├── manager.rs       # Database manager
│   │   └── models.rs        # Data models
│   ├── infrastructure/       # Infrastructure
│   │   ├── config/          # Configuration management
│   │   ├── logging/         # Logging setup
│   │   ├── sysinfo.rs      # System info utilities
│   │   └── websocket.rs    # WebSocket server
│   └── main.rs              # Entry point
├── frontend/                 # Vue.js frontend
│   ├── src/
│   │   ├── components/      # Vue components
│   │   │   ├── DevTools.vue # Debug panel
│   │   │   ├── ErrorBoundary.vue
│   │   │   ├── ErrorLogViewer.vue
│   │   │   ├── AppSidebar.vue
│   │   │   ├── FeatureCard.vue
│   │   │   └── layout/      # Layout components
│   │   ├── composables/     # Vue composables
│   │   ├── stores/          # Pinia stores
│   │   │   ├── userStore.ts
│   │   │   └── systemStore.ts
│   │   ├── services/        # Services
│   │   │   ├── webui.ts     # WebUI IPC service
│   │   │   └── logger.ts    # Logger service
│   │   ├── types/           # TypeScript types
│   │   ├── views/           # Page views
│   │   ├── core/            # Core utilities
│   │   │   └── errorTracker.ts
│   │   └── main.ts         # Entry point
│   ├── rspack.config.ts    # Production config
│   └── rspack.config.dev.ts # Dev config
├── docs/                     # Documentation
├── static/                   # Copied static assets
├── app.config.toml           # Application configuration
├── run.sh                   # Build and run script
├── build-frontend.js        # Frontend build script
└── Cargo.toml               # Rust dependencies
```

## Features

### Backend Features

1. **WebUI Integration**: Native window embedding via webui-rs
2. **SQLite Database**: Persistent storage with rusqlite
3. **WebSocket Server**: Real-time communication support
4. **Plugin System**: Extensible backend functionality
5. **System Information**: Platform-agnostic system metrics
6. **Window Tracking**: Monitor window state changes
7. **Configuration Management**: TOML-based configuration
8. **Logging**: File and console logging with rotation
9. **Error Handling**: Custom error types with thiserror
10. **Async Runtime**: Tokio-based concurrent operations

### Frontend Features

1. **Vue 3 Composition API**: Modern reactive UI development
2. **Pinia State Management**: Centralized state with stores
3. **TypeScript**: Type-safe frontend code
4. **WebUI Bridge**: Unified IPC communication layer
5. **DevTools Panel**: Built-in debugging panel
6. **Error Boundary**: Graceful error handling
7. **Rspack Bundler**: Fast builds with hot reload
8. **Biome Linter**: Code quality enforcement
9. **Responsive Design**: Modern UI with CSS variables

## Communication

The application supports multiple transport mechanisms:

### Transport Options

1. **WebUI IPC** - Direct IPC via WebUI library (recommended for function calls)
2. **WebSocket** - TCP-based real-time messaging
3. **HTTP/REST** - Can be added for REST API endpoints

### Serialization Formats

1. **JSON** - Human-readable, widely supported (current default)
2. **MessagePack** - Binary, compact
3. **CBOR** - Binary, self-describing

## Testing

```bash
# Backend tests
cargo test

# Frontend tests
cd frontend && bun test
```

Current test coverage:
- Backend: 7 unit tests for system information
- Frontend: 19 tests for error handling
