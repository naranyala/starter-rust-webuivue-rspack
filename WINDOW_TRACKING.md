# Window Tracking Integration

This document explains how the window state tracking mechanism works between the Vue.js frontend (WinBox.js) and the Rust backend.

## Overview

The window tracking system automatically monitors and logs all window state changes from WinBox.js windows to the Rust backend console. This provides visibility into user interactions with the windowed interface.

## Architecture

### Frontend (Vue.js/TypeScript)

#### 1. Window Tracker Service (`src/models/services/window/windowTracker.ts`)

A singleton service that tracks window state changes and sends them to the backend:

```typescript
import { windowTracker, useWindowTracker } from '@/models/services';

// Using the singleton directly
windowTracker.trackWindowOpened(windowId, title, component);
windowTracker.trackWindowFocused(windowId, title, component);
windowTracker.trackWindowMinimized(windowId, title, component);
windowTracker.trackWindowRestored(windowId, title, component);
windowTracker.trackWindowMaximized(windowId, title, component);
windowTracker.trackWindowClosed(windowId, title, component);

// Using the composable in Vue components
const { 
  isTrackingEnabled, 
  windowStatesList,
  enableTracking,
  disableTracking 
} = useWindowTracker();
```

#### 2. Integration in App.vue

Window events are automatically tracked in `App.vue`:
- `onclose` → `trackWindowClosed`
- `onminimize` → `trackWindowMinimized`
- `onrestore` → `trackWindowRestored`
- `onmaximize` → `trackWindowMaximized`
- `openWindow()` → `trackWindowOpened`
- `focusWindow()` → `trackWindowFocused`

#### 3. Communication Protocol

The frontend sends window state changes to the backend using the `window.webui.call()` mechanism:

```typescript
const eventData = JSON.stringify(payload);
const elementName = `window_state_change:${encodeURIComponent(eventData)}`;
await window.webui.call(elementName);
```

### Backend (Rust)

#### 1. Event Definition (`src/infrastructure/event_bus.rs`)

New event type added to `AppEvent` enum:

```rust
pub enum AppEvent {
    // ... existing events
    WindowStateChanged { 
        window_id: i64, 
        window_title: String, 
        component: String, 
        previous_state: Option<String>, 
        new_state: String 
    },
}
```

#### 2. Window Handler (`src/presentation/handlers/window_handlers.rs`)

Handles incoming window state change events from frontend:

- Receives and parses JSON payload from frontend
- Stores event in history (last 100 events)
- Logs state change with visual indicators
- Emits to event bus for other components

**Visual indicators in logs:**
- [Opened] Opened
- [Focused] Focused
- [Minimized] Minimized
- [Restored] Restored
- [Maximized] Maximized
- [Closed] Closed

#### 3. Setup in Main

Window handlers are registered in `main.rs`:

```rust
presentation::handlers::window_handlers::setup_window_handlers(&mut my_window);
```

## Tracked Events

| Event | Description | Frontend Trigger | Backend Log |
|-------|-------------|------------------|-------------|
| `opened` | Window created | `openWindow()` | [Opened] Window opened |
| `focused` | Window focused | `focusWindow()`, clicking window | [Focused] Window focused |
| `minimized` | Window minimized | Minimize button, `minimizeAll()` | [Minimized] Window minimized |
| `restored` | Window restored | Restore button, focusing minimized | [Restored] Window restored |
| `maximized` | Window maximized | Maximize button | [Maximized] Window maximized |
| `closed` | Window closed | Close button | [Closed] Window closed |

## Example Backend Output

```
========================================
  [Window State Change]
========================================

  Window ID: 1
  Window Title: User Management
  Component: UserList
  Previous State: inactive
  New State: focused
  Timestamp: 2026-02-16T10:30:00.000Z

[2026-02-16 10:30:00.123 INF] src/presentation/handlers/window_handlers [Window] 1 - 'User Management' (UserList) changed from Some("inactive") to "focused"
```
============================================================
  [Window State Change]
============================================================

  Window ID: 1
  Window Title: User Management
  Component: UserList
  Previous State: inactive
  New State: focused
  Timestamp: 2026-02-16T10:30:00.000Z

[2026-02-16 10:30:00.123 INF] src/presentation/handlers/window_handlers [Window] 1 - 'User Management' (UserList) changed from Some("inactive") to "focused"
```

## Usage in Components

### Accessing Window State

```vue
<script setup>
import { useWindowTracking } from '@/models';

const { 
  isTrackingEnabled,
  windowStatesList,
  getWindowState,
  getAllWindowStates 
} = useWindowTracking();

// Check if tracking is enabled
console.log(isTrackingEnabled.value);

// Get state of specific window
const state = getWindowState(1);

// Get all window states
const allStates = getAllWindowStates();
</script>
```

### Window Tracker Monitor

A dedicated view is available at `WindowTrackerView.vue` to monitor window states in real-time.

## API Reference

### Frontend API

#### `WindowTracker` Class

- `enable()` - Enable tracking
- `disable()` - Disable tracking
- `trackWindowOpened(id, title, component)` - Track window creation
- `trackWindowFocused(id, title, component)` - Track window focus
- `trackWindowMinimized(id, title, component)` - Track minimize
- `trackWindowRestored(id, title, component)` - Track restore
- `trackWindowMaximized(id, title, component)` - Track maximize
- `trackWindowClosed(id, title, component)` - Track close
- `getWindowState(id)` - Get current state of window
- `getAllWindowStates()` - Get all window states

#### `useWindowTracker()` Composable

Returns:
- `isTrackingEnabled: Ref<boolean>` - Tracking status
- `lastEvent: Ref<WindowTrackingInfo \| null>` - Last tracked event
- `enableTracking()` - Enable tracking
- `disableTracking()` - Disable tracking
- `getWindowState(id)` - Get window state
- `getAllWindowStates()` - Get all states
- `windowStatesList: ComputedRef<Array>` - List of all window states
- `trackCustomEvent(...)` - Track custom event

### Backend API

#### Handler Bindings

- `window_state_change` - Receives state change events from frontend
- `get_window_state_history` - Returns history of state changes

#### Event Bus

Subscribe to window state changes:

```rust
use crate::infrastructure::event_bus::{EVENT_BUS, AppEvent};

EVENT_BUS.subscribe("window.state_changed", |envelope| {
    if let AppEvent::WindowStateChanged { window_id, new_state, .. } = envelope.event {
        println!("Window {} changed to {}", window_id, new_state);
    }
}).await;
```

## Data Structure

### Frontend → Backend Payload

```typescript
interface WindowStateChangePayload {
  window_id: number;        // Window ID
  window_title: string;     // Window title
  component: string;        // Component name
  previous_state: string | null;  // Previous state (if any)
  new_state: string;        // New state
  timestamp: string;        // ISO 8601 timestamp
}
```

### Backend Event

```rust
AppEvent::WindowStateChanged {
    window_id: i64,
    window_title: String,
    component: String,
    previous_state: Option<String>,
    new_state: String,
}
```

## Configuration

Tracking is enabled by default. To disable programmatically:

```typescript
import { windowTracker } from '@/models/services';

// Disable tracking
windowTracker.disable();

// Re-enable
windowTracker.enable();
```

## Troubleshooting

### Events not appearing in backend logs

1. Check that `window.webui` is available
2. Verify the handler is registered in `main.rs`
3. Check browser console for frontend errors
4. Ensure the event data is valid JSON

### URL encoding issues

The payload is URL-encoded before sending. The backend automatically decodes it using the `urldecode()` function in `window_handlers.rs`.

## Future Enhancements

- [ ] Persist window state history to database
- [ ] Add window position/size tracking
- [ ] Implement window state restore on app restart
- [ ] Add analytics/statistics for window usage
- [ ] Export window state history to file
