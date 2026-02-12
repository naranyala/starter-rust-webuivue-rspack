# Rust WebUI Application

A modern desktop application built with **Rust**, **WebUI**, and **Vue.js** featuring SQLite integration. This application demonstrates a full-stack desktop architecture where Rust handles backend logic and data management, while Vue.js provides a reactive web-based frontend rendered in a native window.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Project Structure](#project-structure)
- [Root Directory Files](#root-directory-files)
- [Backend (Rust) Structure](#backend-rust-structure)
- [Frontend (Vue.js) Structure](#frontend-vuejs-structure)
- [Distribution Structure](#distribution-structure)
- [Build System](#build-system)
- [Configuration System](#configuration-system)
- [Communication Flow](#communication-flow)
- [Building and Running](#building-and-running)
- [Cross-Platform Distribution](#cross-platform-distribution)

---

## Architecture Overview

This project implements a **hybrid desktop architecture** that combines:

1. **Rust Backend**: Handles business logic, database operations, system interactions, and serves the frontend
2. **WebUI Framework**: A lightweight library that embeds a web browser (using system WebView on Linux, macOS, and Windows) to render web content in a native window
3. **Vue.js Frontend**: A reactive JavaScript framework that provides the user interface
4. **SQLite Database**: A self-contained, embedded SQL database with bundled SQLite (no external dependencies)

The application uses **bidirectional communication** between Rust and JavaScript:
- Rust exposes functions that JavaScript can call via `window.webui.call()`
- JavaScript can dispatch custom events that Rust handlers process

---

## Project Structure

```
starter-rustwebui-vue/
├── .github/                          # GitHub Actions CI/CD configuration
│   └── workflows/
│       └── cross-build.yml           # Cross-platform build pipeline
│
├── .git/                             # Git version control (auto-generated)
│
├── Cargo.lock                        # Locked dependency versions for Rust
│
├── Cargo.toml                        # Rust project manifest (name, version, dependencies)
│
├── app.config.toml                   # Application configuration (database, logging, etc.)
│
├── app.db                            # SQLite database file (auto-generated at runtime)
│
├── application.log                   # Runtime log file (auto-generated)
│
├── build.rs                          # Rust build script (compiles C dependencies)
│
├── build-frontend.js                 # Frontend build script (Bun + Rsbuild)
│
├── build-dist.sh                     # Cross-platform distribution builder
│
├── dist/                             # Distribution packages (auto-generated)
│   └── app-1.0.0-linux-x64/         # Self-contained package for Linux x64
│       ├── app                       # Executable binary
│       ├── app.config.toml           # Configuration file
│       ├── app.db                    # Database file
│       ├── static/                   # Frontend static files
│       ├── README.txt                # Distribution README
│       └── start.sh                  # Startup script
│
├── docs/                             # Documentation directory (reserved)
│
├── examples/                         # Example and reference code
│   └── webui-temp/                   # WebUI temporary/migrated examples
│       ├── examples/                 # Rust example programs
│       ├── src/                      # Reference source code
│       └── Cargo.toml                # Separate Rust workspace
│
├── frontend/                        # Vue.js frontend application
│   ├── dist/                        # Build output (auto-generated)
│   │   ├── index.html              # Entry HTML file
│   │   └── static/                  # Static assets
│   │       ├── css/                 # Compiled CSS
│   │       └── js/                  # Compiled JavaScript
│   ├── src/                        # Frontend source code
│   │   ├── components/             # Reusable Vue components
│   │   ├── lib/                    # JavaScript utilities
│   │   │   ├── di.js              # Dependency injection
│   │   │   ├── index.js           # Main library exports
│   │   │   ├── logger.js          # Logging utilities
│   │   │   └── webui-bridge.js    # Rust-JavaScript bridge
│   │   ├── types/                  # TypeScript type definitions
│   │   ├── use-cases/             # Feature-specific components
│   │   │   └── App.vue            # Root Vue component
│   │   └── main.ts                # Frontend entry point
│   ├── biome.json                  # Biome linter/formatter config
│   ├── bun.lock                    # Bun lock file
│   ├── index.html                  # Development HTML template
│   ├── package.json               # Node.js dependencies
│   ├── package-lock.json          # npm lock file (fallback)
│   ├── rsbuild.config.dev.ts      # Development build config
│   ├── rsbuild.config.ts          # Production build config
│   └── tsconfig.json              # TypeScript configuration
│
├── post-build.sh                   # Post-build script (renames executables)
│
├── run.sh                          # Master build and run script
│
├── static/                         # Runtime static files (auto-generated)
│   ├── css/                        # Compiled CSS files
│   │   └── index.85b27526.css
│   └── js/                         # Compiled JavaScript files
│       ├── index.72bbc473.js
│       ├── lib-polyfill.c7fb3a2b.js
│       ├── lib-vue.1934806c.js
│       └── lib-vue.1934806c.js.LICENSE.txt
│
├── src/                            # Rust backend source code
│   ├── infrastructure/             # Infrastructure layer (I/O, databases, config)
│   │   ├── config.rs              # TOML configuration loader
│   │   ├── database.rs            # SQLite database abstraction
│   │   ├── di.rs                  # Dependency injection container
│   │   ├── logging.rs             # Logging infrastructure
│   │   └── mod.rs                 # Module exports
│   ├── use_cases/                  # Business logic layer
│   │   ├── handlers/              # Event handlers (UI, database, system)
│   │   │   ├── db_handlers.rs    # Database CRUD handlers
│   │   │   ├── mod.rs             # Handler module exports
│   │   │   ├── sysinfo_handlers.rs # System information handlers
│   │   │   └── ui_handlers.rs     # UI event handlers
│   │   └── mod.rs                  # Use cases module exports
│   └── main.rs                     # Application entry point
│
└── thirdparty/                     # Third-party dependencies (source form)
    └── webui-c-src/               # WebUI C library source code
        ├── bridge/                 # WebUI bridge implementation
        ├── examples/               # C examples
        ├── include/                # C header files
        └── src/                    # C source files
```

---

## Root Directory Files

### Cargo.toml
**Purpose**: Rust project manifest and dependency declaration

**Key Sections**:
- `[package]`: Project name, version, Rust edition
- `[dependencies]`: External crates (rusqlite, webui-rs, serde, chrono, etc.)
- `[build-dependencies]`: Build-time-only dependencies (cc, walkdir, toml)
- `[profile.release]`: Release build optimization settings (LTO, codegen-units)

### Cargo.lock
**Purpose**: Locked dependency tree ensuring reproducible builds

### build.rs
**Purpose**: Rust build script that compiles C dependencies

**What It Does**:
1. Reads `app.config.toml` to get executable name
2. Compiles `webui.c` and `civetweb.c` using `cc` crate
3. Sets compiler flags: `-fPIC`, `-DUSE_CIVETWEB`, `-DNO_SSL`, etc.
4. Links the compiled static library into the Rust binary
5. Outputs build configuration to `OUT_DIR/build_config.rs`

### build-frontend.js
**Purpose**: Frontend build script using Bun and Rsbuild

**What It Does**:
1. Runs `rsbuild build --config rsbuild.config.ts`
2. Flattens the output directory structure
3. Copies JS/CSS files to root `static/` directory
4. Updates `index.html` to reference flattened files

### build-dist.sh
**Purpose**: Cross-platform distribution builder

**What It Does**:
1. Detects current platform and architecture
2. Builds frontend (if needed)
3. Builds Rust application in release mode
4. Creates self-contained package in `dist/` directory
5. Archives as `.tar.gz` (Linux/macOS) or `.zip` (Windows)

### run.sh
**Purpose**: Master build and run script

**Options**:
- `(none)`: Build and run
- `--build`: Build only (frontend + Rust)
- `--build-frontend`: Build frontend only
- `--build-rust`: Build Rust only
- `--release`: Build release version
- `--run`: Run application
- `--clean`: Clean build artifacts
- `--rebuild`: Clean and rebuild
- `--help`: Show help

### post-build.sh
**Purpose**: Post-build executable renaming

**What It Does**:
1. Reads executable name from `app.config.toml`
2. Renames `target/release/rustwebui-app` to `target/release/app`
3. Handles Windows `.exe` renaming
4. Verifies static linking

### app.config.toml
**Purpose**: Application configuration file

**Structure**:
```toml
[executable]
name = "app"                          # Output executable name

[database]
path = "app.db"                        # SQLite database path
create_sample_data = true              # Insert sample data on init

[logging]
level = "info"                         # Log level (debug, info, warn, error)
file = "application.log"              # Log file path
```

---

## Backend (Rust) Structure

### src/main.rs
**Purpose**: Application entry point

**Responsibilities**:
1. Initialize logging system
2. Load configuration from `app.config.toml`
3. Initialize SQLite database
4. Create WebUI window
5. Set up event handlers (UI, database, system)
6. Bind JavaScript functions to Rust handlers
7. Run the application loop

### src/infrastructure/
**Purpose**: Infrastructure layer providing low-level services

#### config.rs
**Responsibilities**:
- Parse `app.config.toml`
- Provide typed access to configuration values
- Support environment variable overrides

#### database.rs
**Responsibilities**:
- Manage SQLite connection (thread-safe via Arc<Mutex<Connection>>)
- Initialize schema (users, products tables)
- Provide CRUD operations:
  - `query()`: Execute SELECT queries, return JSON results
  - `execute()`: Execute INSERT/UPDATE/DELETE, return rows affected
  - `get_all_users()`: Retrieve all users
  - `insert_user()`: Create new user
  - `update_user()`: Update existing user by ID
  - `delete_user()`: Remove user by ID
  - `insert_sample_data()`: Seed database with test data
- Type mappings: SQLite types → serde_json::Value

#### logging.rs
**Responsibilities**:
- Configure env_logger
- Set log level from configuration
- Write to file and/or stderr

#### di.rs
**Responsibilities**:
- Dependency injection container
- Manage singleton instances (Database, etc.)

### src/use_cases/
**Purpose**: Business logic layer

#### handlers/
**Responsibilities**: Handle events from JavaScript

##### db_handlers.rs
**Events Handled**:
- `get_users`: Retrieve all users, dispatch `db_response`
- `create_user`: Create user, dispatch `user_create_response`
- `update_user`: Update user, dispatch `user_update_response`
- `delete_user`: Delete user, dispatch `user_delete_response`
- `get_db_stats`: Get database statistics, dispatch `stats_response`

**Communication Pattern**:
1. JavaScript calls `webui.bind("event_name", handler)`
2. When event fires, handler processes data
3. Handler sends response via `CustomEvent`:
   ```javascript
   window.dispatchEvent(new CustomEvent('response_name', { detail: json_data }))
   ```

##### sysinfo_handlers.rs
**Events Handled**:
- `get_sysinfo`: Retrieve system information
- `get_os_info`: Operating system details
- `get_memory_info`: Memory usage from /proc/meminfo
- `get_cpu_info`: CPU cores, model, usage
- `get_disk_info`: Disk usage from df command
- `get_uptime`: System uptime

##### ui_handlers.rs
**Responsibilities**:
- UI initialization events
- Window management events
- Theme/size configuration

---

## Frontend (Vue.js) Structure

### frontend/src/main.ts
**Purpose**: Frontend entry point

**Responsibilities**:
1. Initialize Vue application
2. Mount root component to DOM
3. Set up WebUI bridge
4. Initialize global libraries (logger, DI)

### frontend/src/lib/

#### webui-bridge.js
**Responsibilities**:
- `webui.call()`: Call Rust functions from JavaScript
- Dispatch custom events for Rust-to-JavaScript communication
- Handle response callbacks

#### logger.js
**Responsibilities**:
- Unified logging interface
- Console output with levels (debug, info, warn, error)
- Optional backend integration

#### di.js
**Responsibilities**:
- Dependency injection container
- Service registration and retrieval
- Singleton management

### frontend/src/use-cases/App.vue
**Purpose**: Root Vue component

**Responsibilities**:
- Main application layout
- Initialize communication with Rust backend
- Render child components
- Handle global state

### frontend/rsbuild.config.ts
**Purpose**: Production build configuration for Rsbuild

**Key Settings**:
- Output directory: `frontend/dist`
- Static file flattening
- Minification and optimization
- CSS/JS splitting

### frontend/rsbuild.config.dev.ts
**Purpose**: Development build configuration

**Key Settings**:
- Source map generation
- Hot module replacement (HMR)
- Development server

---

## Distribution Structure

### dist/app-1.0.0-linux-x64/

```
app-1.0.0-linux-x64/
├── app                       # ELF 64-bit executable (statically linked)
├── app.config.toml           # Configuration file
├── app.db                    # SQLite database (can be initialized at runtime)
├── static/                   # Frontend static files
│   ├── css/
│   │   └── index.85b27526.css
│   └── js/
│       ├── index.72bbc473.js
│       ├── lib-polyfill.c7fb3a2b.js
│       ├── lib-vue.1934806c.js
│       └── lib-vue.1934806c.js.LICENSE.txt
├── README.txt               # Quick start guide
└── start.sh                # Convenience startup script
```

**Self-Contained Verification**:
- Only depends on standard C library (libc.so.6, libm.so.6)
- All dependencies (SQLite, WebUI, Vue.js) are bundled
- No external runtime required

---

## Build System

### Build Pipeline Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                          BUILD PIPELINE                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  1. FRONTEND BUILD (Bun + Rsbuild)                                   │
│     ┌──────────────────┐                                             │
│     │  src/*.vue,      │                                             │
│     │  src/*.ts,       │ ──► rsbuild ──► frontend/dist/             │
│     │  src/*.js        │                                             │
│     └──────────────────┘                                             │
│           │                                                         │
│           ▼                                                         │
│     ┌──────────────────┐                                             │
│     │ build-frontend.js │ ──► static/ (flattened files)              │
│     └──────────────────┘                                             │
│                                                                      │
│  2. BACKEND BUILD (Rust + C Compiler)                                │
│     ┌──────────────────┐                                             │
│     │  src/*.rs        │ ──► cargo build ──► target/release/app      │
│     └──────────────────┘                                             │
│           │                                                         │
│           ▼                                                         │
│     ┌──────────────────┐                                             │
│     │   build.rs       │ ──► cc crate ──► libwebui-2-static.a        │
│     │  (thirdparty/)   │                                             │
│     └──────────────────┘                                             │
│                                                                      │
│  3. POST-BUILD                                                       │
│     ┌──────────────────┐                                             │
│     │ post-build.sh   │ ──► Rename executable                       │
│     └──────────────────┘                                             │
│                                                                      │
│  4. DISTRIBUTION                                                     │
│     ┌──────────────────┐                                             │
│     │ build-dist.sh   │ ──► dist/app-*/                             │
│     │                  │ ──► app-*.tar.gz                            │
│     └──────────────────┘                                             │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Build Dependencies

| Tool | Purpose | Installation |
|------|---------|--------------|
| Rust/Cargo | Backend compilation | rustup.rs |
| Bun | Frontend package manager/build | bun.sh |
| cc (GCC/Clang) | C compiler for WebUI | System package manager |
| WebKitGTK | WebView runtime (Linux) | System package manager |

---

## Configuration System

### Configuration Loading Order

1. **Default values**: Hardcoded in Rust
2. **TOML file**: `app.config.toml` (highest priority)
3. **Environment variables**: Override TOML (planned)

### Configuration Sections

```toml
[executable]           # Executable naming
name = "app"

[database]             # Database settings
path = "app.db"
create_sample_data = true

[logging]              # Logging configuration
level = "info"
file = "application.log"
```

### Runtime Behavior

- Configuration is loaded once at startup
- Changes require restart
- Database path is relative to working directory
- Logs are appended to log file

---

## Communication Flow

### JavaScript to Rust

```javascript
// JavaScript
webui.call('update_user:1:John:john@example.com:Admin:Active');
```

```rust
// Rust handler
window.bind("update_user", |event| {
    let element_name = /* parse event.element */;
    let parts: Vec<&str> = element_name.split(':').collect();
    // Extract id, name, email, role, status
    db.update_user(id, name_opt, email_opt, ...)?;
});
```

### Rust to JavaScript

```rust
// Rust
let response = serde_json::json!({
    "success": true,
    "message": "User updated"
});
send_response(window, "user_update_response", &response);
```

```javascript
// JavaScript
window.addEventListener('user_update_response', (e) => {
    const data = e.detail;
    console.log(data.message);
});
```

### Event Naming Convention

| Direction | Pattern | Example |
|-----------|---------|---------|
| JS → Rust | `action:param1:param2` | `create_user:John:john@email.com:User` |
| Rust → JS | `{action}_response` | `user_create_response` |

---

## Building and Running

### Prerequisites

```bash
# Linux (Arch)
sudo pacman -S rustup base-devel webkit2gtk

# Install Bun
curl -fsSL https://bun.sh/install | bash

# Install Rust
rustup init
rustup default stable
```

### Quick Start

```bash
# Build and run (development)
./run.sh

# Build release
./run.sh --release

# Build distribution package
./build-dist.sh build-release

# Verify distribution
./build-dist.sh verify
```

### Build Outputs

| Command | Output |
|---------|--------|
| `./run.sh` | `target/debug/app` |
| `./run.sh --release` | `target/release/app` |
| `./build-dist.sh build-release` | `dist/app-1.0.0-linux-x64.tar.gz` |

---

## Cross-Platform Distribution

### Supported Platforms

| Platform | Architecture | Archive Format | Run Command |
|----------|--------------|----------------|-------------|
| Linux | x64, arm64 | .tar.gz | `./app` |
| macOS | x64, arm64 | .tar.gz | `./app` |
| Windows | x64 | .zip | `app.exe` |

### CI/CD Pipeline

The `.github/workflows/cross-build.yml` orchestrates:

1. **Linux Build** (ubuntu-latest)
   - Builds with GCC
   - Creates .tar.gz archive

2. **macOS Build** (macos-latest)
   - Builds for x86_64-apple-darwin
   - Creates .tar.gz archive

3. **Windows Build** (windows-latest)
   - Builds with MinGW
   - Creates .zip archive

4. **Release Creation**
   - Downloads all artifacts
   - Attaches to GitHub release

### Creating a Release

```bash
# 1. Push changes to main branch
git add .
git commit -m "Release preparation"
git push

# 2. Create GitHub release (triggers CI/CD)
gh release create v1.0.0 \
  --title "Release v1.0.0" \
  --notes "See CHANGELOG.md"
```

### Distribution Verification

```bash
# Extract and run
tar -xzf app-1.0.0-linux-x64.tar.gz
cd app-1.0.0-linux-x64
./app

# Check dependencies (should show only libc, libm)
ldd app
```

---

## License

MIT License - See LICENSE file for details.

---

## Further Reading

- [WebUI Documentation](https://webui.dev/)
- [Vue.js Documentation](https://vuejs.org/)
- [Rust Programming Language](https://www.rust-lang.org/)
- [SQLite Documentation](https://www.sqlite.org/)
- [Bun Documentation](https://bun.sh/docs)
- [Rsbuild Documentation](https://rsbuild.dev/)
