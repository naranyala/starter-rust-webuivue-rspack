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

### Layer Structure

#### Core Layer (`src/core/`)

- **error.rs**: Error types using thiserror crate with "errors as values" pattern
- **plugin.rs**: Plugin trait and PluginManager for extensible functionality

#### Commands Layer (`src/commands/`)

- **counter.rs**: Counter state management
- **sysinfo.rs**: System information commands
- **ui.rs**: UI-related commands
- **window_state.rs**: Window state tracking commands

#### Plugins Layer (`src/plugins/`)

- **system_info/**: System information handlers
- **database/**: Database operations
- **window_tracking/**: Window state tracking

#### Infrastructure Layer (`src/infrastructure/`)

- **config/**: Application configuration management (TOML)
- **logging/**: Logging setup with file and console output
- **websocket/**: WebSocket server for real-time communication
- **sysinfo.rs**: Platform-agnostic system information utilities

#### Database Layer (`src/db/`)

- **manager.rs**: SQLite database manager
- **models.rs**: Data models and schemas

## Frontend Architecture (MVVM)

The frontend follows the Model-View-ViewModel pattern with Pinia:

```
┌─────────────────┐
│ Views           │ ← Vue components, templates
├─────────────────┤
│ ViewModels      │ ← Composables, business logic for views
├─────────────────┤
│ Models          │ ← Pinia stores, data models, services
└─────────────────┘
```

### Layer Structure

#### Components (`frontend/src/components/`)

- **DevTools.vue**: Collapsible debugging panel
- **ErrorBoundary.vue**: Error handling wrapper
- **ErrorLogViewer.vue**: Error display component
- **AppSidebar.vue**: Application sidebar
- **FeatureCard.vue**: Feature display cards
- **layout/**: Layout components

#### Composables (`frontend/src/composables/`)

- **useWindowManager.ts**: Window state management

#### Stores (`frontend/src/stores/`)

- **userStore.ts**: User data management
- **systemStore.ts**: System information state

#### Services (`frontend/src/services/`)

- **webui.ts**: Unified WebUI IPC communication
- **logger.ts**: Logging service

#### Types (`frontend/src/types/`)

- **index.ts**: Central TypeScript type definitions

#### Core (`frontend/src/core/`)

- **errorTracker.ts**: Error tracking and reporting

## Communication Layer

The application uses WebUI IPC as the primary communication mechanism:

### WebUI Service (`services/webui.ts`)

```typescript
class WebUIService {
  async call<T>(action: string, data?: unknown): Promise<T>
  bind(event: string, callback: (data: unknown) => void): void
}
```

### Usage

```typescript
import { webui } from '@/services/webui';

// Call backend function
const response = await webui.call<ApiResponse<User[]>>('get_users');

// Bind to backend events
webui.bind('event_name', (data) => {
  console.log('Received:', data);
});
```

## Error Handling

### Backend (Rust)

Uses the "errors as values" pattern with custom error types:

```rust
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Configuration error: {0}")]
    Config(#[from] toml::de::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("WebUI error: {0}")]
    WebUI(String),
}

pub type AppResult<T> = Result<T, AppError>;
```

### Frontend (TypeScript)

Type-safe error handling with Result type:

```typescript
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export type Result<T, E = string> = 
  | { success: true; data: T }
  | { success: false; error: E };
```

### Global Error Handling

- **Backend**: Panic handler with backtrace support
- **Frontend**: Global error listeners, ErrorBoundary component, DevTools panel

## Plugin System

Both backend and frontend implement a plugin-based architecture.

### Backend Plugin Trait

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn register(&self, window: &mut webui::Window);
    fn init(&self) -> Result<(), Box<dyn std::error::Error>>;
}
```

### Registered Plugins

1. **SystemInfoPlugin**: Provides system information to frontend
2. **DatabasePlugin**: Handles SQLite operations
3. **WindowTrackingPlugin**: Monitors window state changes

## Configuration

### Application Config (`app.config.toml`)

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

## State Management (Pinia)

### Store Pattern

```typescript
import { defineStore } from 'pinia';

export const useUserStore = defineStore('user', {
  state: () => ({
    users: [],
    loading: false,
    error: null,
  }),
  
  getters: {
    activeUsers: (state) => state.users.filter(u => u.status === 'Active'),
  },
  
  actions: {
    async fetchUsers() {
      // Implementation
    },
  },
});
```

## DevTools Panel

The application includes a built-in DevTools panel with:

- **Console**: Log viewing with filtering
- **Errors**: Error tracking with stack traces
- **Network**: HTTP request monitoring
- **System**: Frontend and backend system info
- **State**: Pinia store inspection
- **Actions**: Test actions and utilities

Access: Click the bottom panel header to expand/collapse.
