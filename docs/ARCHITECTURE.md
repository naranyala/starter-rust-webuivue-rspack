# Architecture

This document describes the architectural patterns and design decisions used in the project.

## Backend Architecture (Clean Architecture)

The backend follows Clean Architecture with clear separation of concerns:

```
┌─────────────────┐
│ Presentation    │ ← Event handlers, API controllers
├─────────────────┤
│ Application     │ ← Use cases, business logic orchestration  
├─────────────────┤
│ Domain          │ ← Entities, business rules, interfaces
├─────────────────┤
│ Infrastructure  │ ← Database, external services, configuration
└─────────────────┘
```

### Core Layer (`src/core/`)

- **error.rs**: Error types using thiserror crate with "errors as values" pattern
- **plugin.rs**: Plugin trait and PluginManager for extensible functionality

### Plugins Layer (`src/plugins/`)

- **system_info/**: System information handlers
- **database/**: Database operations
- **window_tracking/**: Window state tracking

### Infrastructure Layer (`src/infrastructure/`)

- **config/**: Application configuration management
- **logging/**: Logging setup with file and console output
- **websocket/**: WebSocket server for real-time communication

## Frontend Architecture (MVVM)

The frontend follows the Model-View-ViewModel pattern:

```
┌─────────────────┐
│ Views           │ ← Vue components, templates
├─────────────────┤
│ ViewModels      │ ← Business logic for views
├─────────────────┤
│ Models          │ ← Data models, services, stores
└─────────────────┘
```

### Core Layer (`frontend/src/core/`)

- **error.ts**: Error handling with Result type and factory methods
- **plugin.ts**: Plugin interface and PluginManager
- **connection.ts**: WebUI connection management

### Plugins Layer (`frontend/src/plugins/`)

- **home/**: Dashboard feature
- **system-info/**: System information display
- **database/**: Database interaction

## Communication Layer

The application supports multiple transport mechanisms:

### Transport Options

1. **WebUI IPC** - Direct IPC via WebUI library (recommended for function calls)
2. **WebSocket** - TCP-based real-time messaging
3. **HTTP/REST** - Can be added for REST API endpoints

### Serialization Formats

1. **JSON** - Human-readable, widely supported (current default)
2. **MessagePack** - Binary, compact
3. **CBOR** - Binary, self-describing
4. **BinCode** - Binary, Rust-specific

## Error Handling

### Backend (Rust)

Uses the "errors as values" pattern with custom error types:

```rust
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    // ... more variants
}

pub type AppResult<T> = Result<T, AppError>;
```

### Frontend (TypeScript)

Type-safe error handling with discriminated unions:

```typescript
export type Result<T, E = AppError> = 
  | { ok: true; value: T }
  | { ok: false; error: E };
```

## Plugin System

Both backend and frontend implement a plugin-based architecture:

### Backend Plugin Trait

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn register(&self, window: &mut webui::Window);
    fn init(&self) -> Result<(), Box<dyn std::error::Error>>;
}
```

### Frontend Plugin Interface

```typescript
export interface Plugin {
  name: string;
  version: string;
  initialize(): void;
  register(): void;
}
```
