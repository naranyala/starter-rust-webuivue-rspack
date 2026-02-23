<template>
  <div class="window-tracker-view">
    <div class="header">
      <h2>🪟 Window Tracking Monitor</h2>
      <div class="controls">
        <Button variant="secondary" @click="testConnection" :loading="isTesting">
          {{ isTesting ? 'Testing...' : 'Test Connection' }}
        </Button>
        <Button 
          :variant="isTrackingEnabled ? 'secondary' : 'primary'" 
          @click="isTrackingEnabled ? disableTracking() : enableTracking()"
        >
          {{ isTrackingEnabled ? 'Disable Tracking' : 'Enable Tracking' }}
        </Button>
      </div>
    </div>

    <div class="status-section">
      <div class="status-card" :class="{ active: isTrackingEnabled }">
        <div class="status-icon">{{ isTrackingEnabled ? '✅' : '⏸️' }}</div>
        <div class="status-text">
          <div class="status-label">Tracking Status</div>
          <div class="status-value">{{ isTrackingEnabled ? 'Active' : 'Paused' }}</div>
        </div>
      </div>
      
      <div class="status-card" :class="connectionStateClass">
        <div class="status-icon">{{ connectionIcon }}</div>
        <div class="status-text">
          <div class="status-label">Connection State</div>
          <div class="status-value">{{ connectionState }}</div>
        </div>
      </div>
      
      <div class="status-card" v-if="connInfo.reconnectAttempts > 0">
        <div class="status-icon">[Reconnecting]</div>
        <div class="status-text">
          <div class="status-label">Reconnect Attempts</div>
          <div class="status-value">{{ connInfo.reconnectAttempts }}/10</div>
        </div>
      </div>
      
      <div class="status-card">
        <div class="status-icon">🪟</div>
        <div class="status-text">
          <div class="status-label">Active Windows</div>
          <div class="status-value">{{ windowStatesList.length }}</div>
        </div>
      </div>
      
      <div class="status-card" v-if="connInfo.pendingMessages > 0">
        <div class="status-icon">[Pending]</div>
        <div class="status-text">
          <div class="status-label">Pending Messages</div>
          <div class="status-value">{{ connInfo.pendingMessages }}</div>
        </div>
      </div>
      
      <div class="status-card" v-if="connInfo.totalMessagesSent > 0">
        <div class="status-icon">📤</div>
        <div class="status-text">
          <div class="status-label">Messages Sent</div>
          <div class="status-value">{{ connInfo.totalMessagesSent }}</div>
        </div>
      </div>
    </div>

    <div class="connection-timeline" v-if="connInfo.lastConnectedAt || connInfo.lastDisconnectedAt">
      <h3>Connection Timeline</h3>
      <div class="timeline-item" v-if="connInfo.lastConnectedAt">
        <span class="timeline-icon">[Connected]</span>
        <span class="timeline-label">Last Connected:</span>
        <span class="timeline-value">{{ formatTime(connInfo.lastConnectedAt) }}</span>
      </div>
      <div class="timeline-item" v-if="connInfo.lastDisconnectedAt">
        <span class="timeline-icon">[Disconnected]</span>
        <span class="timeline-label">Last Disconnected:</span>
        <span class="timeline-value">{{ formatTime(connInfo.lastDisconnectedAt) }}</span>
      </div>
      <div class="timeline-item error" v-if="connInfo.lastErrorAt">
        <span class="timeline-icon">[Error]</span>
        <span class="timeline-label">Last Error:</span>
        <span class="timeline-value">{{ formatTime(connInfo.lastErrorAt) }}</span>
        <span class="timeline-error">{{ connInfo.lastError }}</span>
      </div>
    </div>

    <div class="windows-list" v-if="windowStatesList.length > 0">
      <h3>Current Window States</h3>
      <div class="window-item" v-for="win in windowStatesList" :key="win.windowId">
        <span class="window-id">#{{ win.windowId }}</span>
        <span class="window-state" :class="win.state">{{ win.state }}</span>
      </div>
    </div>

    <div class="info-section">
      <h3>ℹ️ About Window Tracking</h3>
      <p>This monitor displays real-time window state changes from WinBox.js windows.</p>
      <p>All state changes (opened, focused, minimized, restored, maximized, closed) are logged to the Rust backend.</p>
      <p>Check the backend console to see detailed logging output.</p>
      
      <div class="debug-info" v-if="debugMessages.length > 0">
        <h4>Debug Messages</h4>
        <div class="debug-log">
          <div v-for="(msg, index) in debugMessages" :key="index" class="debug-line">
            {{ msg }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, onMounted, onUnmounted, ref } from 'vue';
import Button from '../../components/common/Button.vue';
import { useWindowTracking } from '../../models/composables/useWindowTracking.ts';
import { connectionManager } from '../../models/services/index.ts';

const { isTrackingEnabled, windowStatesList, enableTracking, disableTracking } =
  useWindowTracking();

const isTesting = ref(false);
const debugMessages = ref([]);
const connectionState = ref('disconnected');
const connInfo = ref(connectionManager.getStateInfo());

const connectionStateClass = computed(() => {
  return {
    connected: connectionState.value === 'connected',
    connecting: connectionState.value === 'connecting',
    reconnecting: connectionState.value === 'reconnecting',
    error: connectionState.value === 'error',
    disconnected: connectionState.value === 'disconnected',
  };
});

const connectionIcon = computed(() => {
  switch (connectionState.value) {
    case 'connected':
      return '[Connected]';
    case 'connecting':
      return '[Connecting]';
    case 'reconnecting':
      return '[Reconnecting]';
    case 'error':
      return '[Error]';
    default:
      return '[Closed]';
  }
});

const testConnection = async () => {
  isTesting.value = true;
  const timestamp = new Date().toLocaleTimeString();
  debugMessages.value.push(`[${timestamp}] Testing connection...`);

  const result = await connectionManager.isConnected();

  debugMessages.value.push(
    `[${new Date().toLocaleTimeString()}] Connection test: ${result ? 'SUCCESS' : 'FAILED'}`
  );

  // Keep only last 20 messages
  if (debugMessages.value.length > 20) {
    debugMessages.value = debugMessages.value.slice(-20);
  }

  isTesting.value = false;
};

const formatTime = (date) => {
  if (!date) return 'N/A';
  return new Date(date).toLocaleString();
};

let unsubscribe;

onMounted(() => {
  debugMessages.value.push(`[${new Date().toLocaleTimeString()}] Window Tracker initialized`);

  unsubscribe = connectionManager.subscribe({
    onStateChange: (state, info) => {
      connectionState.value = state;
      connInfo.value = info;
      debugMessages.value.push(`[${new Date().toLocaleTimeString()}] State: ${state}`);
    },
    onError: (error, info) => {
      connInfo.value = info;
      debugMessages.value.push(`[${new Date().toLocaleTimeString()}] Error: ${error.message}`);
    },
    onMessageSent: (payload) => {
      debugMessages.value.push(`[${new Date().toLocaleTimeString()}] Sent: ${payload.new_state}`);
    },
    onMessageReceived: (data) => {
      debugMessages.value.push(
        `[${new Date().toLocaleTimeString()}] Received: ${JSON.stringify(data).slice(0, 50)}`
      );
    },
  });
});

onUnmounted(() => {
  if (unsubscribe) {
    unsubscribe();
  }
});
</script>

<style scoped>
.window-tracker-view {
  padding: 1.5rem;
  height: 100%;
  overflow-y: auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.header h2 {
  margin: 0;
  color: #1f2937;
}

.controls {
  display: flex;
  gap: 0.5rem;
}

.status-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.status-card {
  background: white;
  border-radius: 0.5rem;
  padding: 1rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  border: 2px solid transparent;
  transition: all 0.2s;
}

.status-card.active {
  border-color: #10b981;
  background: #f0fdf4;
}

.status-card.connected {
  border-color: #3b82f6;
  background: #eff6ff;
}

.status-card.connecting,
.status-card.reconnecting {
  border-color: #f59e0b;
  background: #fffbeb;
}

.status-card.error {
  border-color: #ef4444;
  background: #fef2f2;
}

.status-card.disconnected {
  border-color: #6b7280;
  background: #f9fafb;
}

.status-icon {
  font-size: 1.5rem;
}

.status-label {
  font-size: 0.75rem;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.status-value {
  font-size: 1.25rem;
  font-weight: 600;
  color: #1f2937;
}

.connection-timeline {
  background: white;
  border-radius: 0.5rem;
  padding: 1rem;
  margin-bottom: 1.5rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.connection-timeline h3 {
  margin: 0 0 1rem;
  color: #1f2937;
}

.timeline-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0;
  border-bottom: 1px solid #e5e7eb;
}

.timeline-item:last-child {
  border-bottom: none;
}

.timeline-item.error {
  color: #dc2626;
}

.timeline-icon {
  font-size: 1rem;
}

.timeline-label {
  font-weight: 600;
  color: #374151;
}

.timeline-value {
  color: #6b7280;
  font-family: monospace;
  font-size: 0.875rem;
}

.timeline-error {
  margin-left: auto;
  font-size: 0.75rem;
  color: #dc2626;
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.windows-list {
  background: white;
  border-radius: 0.5rem;
  padding: 1rem;
  margin-bottom: 1.5rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.windows-list h3 {
  margin: 0 0 1rem;
  color: #1f2937;
}

.window-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  border-bottom: 1px solid #e5e7eb;
}

.window-item:last-child {
  border-bottom: none;
}

.window-id {
  font-family: monospace;
  color: #6b7280;
}

.window-state {
  padding: 0.25rem 0.75rem;
  border-radius: 9999px;
  font-size: 0.75rem;
  font-weight: 500;
  text-transform: capitalize;
}

.window-state.opened {
  background: #dbeafe;
  color: #2563eb;
}

.window-state.focused {
  background: #dcfce7;
  color: #16a34a;
}

.window-state.minimized {
  background: #fef3c7;
  color: #d97706;
}

.window-state.restored,
.window-state.maximized {
  background: #e0e7ff;
  color: #4f46e5;
}

.window-state.closed {
  background: #fee2e2;
  color: #dc2626;
}

.info-section {
  background: #f9fafb;
  border-radius: 0.5rem;
  padding: 1.5rem;
}

.info-section h3 {
  margin: 0 0 1rem;
  color: #4f46e5;
}

.info-section p {
  margin: 0 0 0.75rem;
  line-height: 1.6;
  color: #4b5563;
}

.debug-info {
  margin-top: 1rem;
  padding: 1rem;
  background: #1f2937;
  border-radius: 0.5rem;
}

.debug-info h4 {
  margin: 0 0 0.5rem;
  color: #94a3b8;
  font-size: 0.875rem;
}

.debug-log {
  font-family: monospace;
  font-size: 0.75rem;
  color: #e2e8f0;
  max-height: 200px;
  overflow-y: auto;
}

.debug-line {
  padding: 0.25rem 0;
  border-bottom: 1px solid #334155;
}

.debug-line:last-child {
  border-bottom: none;
}
</style>
