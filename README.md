# Rust WebUI Vue Rspack Starter

A modern desktop application starter kit combining Rust, WebUI, Vue.js, and Rspack for building cross-platform desktop applications with a web-based UI and native performance.

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Technology Stack](#technology-stack)
- [Architecture](#architecture)
- [Development Setup](#development-setup)
- [Building and Running](#building-and-running)
- [Potential Improvements](#potential-improvements)

## Overview

This project serves as a starter template for building desktop applications using a hybrid architecture:
- **Rust backend** for performance-critical operations and system integration
- **Vue.js frontend** for reactive UI development
- **WebUI** for embedding the web interface in a native window
- **Rspack** for fast frontend bundling

The application demonstrates a full-stack desktop architecture with bidirectional communication between Rust and JavaScript, SQLite integration, and modern development practices.

## Project Structure

```
starter-rust-webuivue-rspack/
├── .git/                           # Git version control
├── .gitignore                      # Git ignore rules
├── app.config.toml                 # Application configuration
├── app.db                          # SQLite database (runtime generated)
├── application.log                 # Application logs (runtime generated)
├── build-dist.sh                   # Distribution packaging script
├── build-frontend.js               # Frontend build configuration
├── build.rs                        # Rust build script
├── Cargo.lock                      # Rust dependency lock file
├── Cargo.toml                      # Rust project manifest
├── docs/                           # Documentation
├── examples/                       # Example implementations
├── frontend/                       # Vue.js frontend application
│   ├── dist/                       # Frontend build output
│   ├── node_modules/               # Frontend dependencies
│   ├── src/                        # Frontend source code
│   │   ├── components/             # Vue components
│   │   ├── composables/            # Vue composables
│   │   ├── lib/                    # Utility libraries
│   │   ├── models/                 # Data models
│   │   ├── services/               # Service implementations
│   │   ├── stores/                 # State management (Pinia)
│   │   ├── types/                  # TypeScript definitions
│   │   ├── use-cases/              # Business logic components
│   │   ├── viewmodels/             # View model implementations
│   │   ├── views/                  # View components
│   │   ├── main.ts                 # Frontend entry point
│   │   └── MVVM_RESTRUCTURE.md     # MVVM architecture documentation
│   ├── biome.json                  # Biome configuration
│   ├── bun.lock                    # Bun dependency lock
│   ├── index.html                  # HTML template
│   ├── package.json                # Frontend dependencies
│   ├── package-lock.json           # npm lock file
│   ├── rspack.config.dev.ts        # Development build config
│   ├── rspack.config.ts            # Production build config
│   └── tsconfig.json               # TypeScript configuration
├── post-build.sh                   # Post-build processing
├── README.md                       # This file
├── run.sh                          # Main build and run script
├── src/                            # Rust backend source
│   ├── application/                # Application layer
│   ├── domains/                    # Domain layer
│   ├── infrastructure/             # Infrastructure layer
│   ├── presentation/               # Presentation layer
│   ├── shared/                     # Shared utilities
│   ├── use_cases/                  # Use case implementations
│   ├── utilities/                  # Utility functions
│   └── main.rs                     # Application entry point
├── static/                         # Static assets (runtime generated)
├── target/                         # Rust build output
├── thirdparty/                     # Third-party dependencies
└── WINDOW_TRACKING.md              # Window tracking documentation
```

## Technology Stack

### Backend (Rust)
- **Language**: Rust 1.93+
- **Framework**: WebUI for native window embedding
- **Database**: SQLite with rusqlite
- **Concurrency**: Tokio async runtime
- **WebSockets**: tokio-tungstenite
- **Serialization**: Serde with JSON support
- **Logging**: env_logger

### Frontend (Vue.js)
- **Framework**: Vue 3 with Composition API
- **State Management**: Pinia
- **Build Tool**: Rspack (fast Rust-based bundler)
- **Language**: TypeScript
- **Styling**: CSS with scoped styles
- **Code Quality**: Biome (formatter/linter)

### Architecture Patterns
- **Frontend**: MVVM (Model-View-ViewModel)
- **Backend**: Clean Architecture (Domain, Application, Infrastructure, Presentation layers)
- **Communication**: WebSocket-based bidirectional messaging

## Architecture

### Backend Architecture (Clean Architecture)

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

### Frontend Architecture (MVVM)

```
┌─────────────────┐
│ Views           │ ← Vue components, templates
├─────────────────┤
│ ViewModels      │ ← Business logic for views
├─────────────────┤
│ Models          │ ← Data models, services, stores
└─────────────────┘
```

### Communication Layer

The application uses WebSocket-based communication between frontend and backend:
- Frontend connects to dynamically assigned WebSocket port
- Bidirectional messaging with request/response pattern
- Custom event system for notifications
- Connection status tracking

## Development Setup

### Prerequisites

- Rust (1.93+)
- Bun (1.39+)
- Node.js (for some tooling)
- WebKitGTK development libraries (Linux)
- GCC/Clang for C compilation

### Installation

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

# Run the application
./run.sh
```

## Building and Running

### Development Mode

```bash
# Build and run in development mode
./run.sh

# Build frontend only
./run.sh --build-frontend

# Build Rust only
./run.sh --build-rust

# Build both
./run.sh --build
```

### Production Build

```bash
# Build release version
./run.sh --release

# Create distribution package
./build-dist.sh build-release
```

### Available Scripts

- `run.sh`: Main build and run script with multiple options
- `build-dist.sh`: Distribution packaging
- `post-build.sh`: Post-build processing
- `build-frontend.js`: Frontend build configuration

## Potential Improvements

### Project Structure Improvements

1. **Configuration Management**
   - Move configuration parsing to a dedicated module
   - Implement environment variable overrides
   - Add configuration validation
   - Create configuration schema documentation

2. **Error Handling Consistency**
   - Establish a unified error handling pattern across both frontend and backend
   - Create custom error types for different domains
   - Implement centralized error logging
   - Add user-friendly error messages

3. **Testing Strategy**
   - Add unit tests for Rust backend modules
   - Implement integration tests for WebSocket communication
   - Add Vue component tests using Vitest
   - Create end-to-end tests using Playwright or similar

4. **Documentation Enhancement**
   - Add inline documentation for public APIs
   - Create architecture decision records (ADRs)
   - Document deployment procedures
   - Add API reference documentation

5. **Build System Optimization**
   - Cache Rust build artifacts in CI/CD
   - Optimize Rspack configuration for faster builds
   - Add incremental build capabilities
   - Implement build profiling

6. **Dependency Management**
   - Audit and update dependencies regularly
   - Implement dependency pinning for production builds
   - Add security scanning for dependencies
   - Document dependency update procedures

7. **Code Organization**
   - Group related functionality into feature modules
   - Implement consistent naming conventions
   - Add code generation tools for boilerplate
   - Create shared utility libraries

8. **Performance Monitoring**
   - Add performance metrics collection
   - Implement resource usage monitoring
   - Add slow query detection for database operations
   - Create performance benchmarking suite

9. **Security Enhancements**
   - Implement input validation and sanitization
   - Add secure communication protocols
   - Implement proper authentication mechanisms
   - Add security headers for web content

10. **Deployment and DevOps**
    - Create Docker containers for easier deployment
    - Add Kubernetes configuration files
    - Implement health check endpoints
    - Add graceful shutdown procedures