<template>
  <div class="ws-status-panel" :class="statusClass">
    <div class="status-bar" :class="{ expanded: isExpanded }" @click="toggleExpand">
      <div class="status-left">
        <span class="status-dot"></span>
        <span class="status-text">{{ statusText }}</span>
      </div>
      <div class="status-right">
        <span v-if="stats.latency" class="latency">{{ stats.latency }}ms</span>
        <span v-if="stats.messagesSent > 0" class="counter">↑{{ stats.messagesSent }}</span>
        <span v-if="stats.messagesReceived > 0" class="counter">↓{{ stats.messagesReceived }}</span>
        <span v-if="stats.messagesQueued > 0" class="queued">{{ stats.messagesQueued }}▧</span>
        <span class="expand-icon">{{ isExpanded ? '▼' : '▲' }}</span>
      </div>
    </div>
    
    <div v-if="isExpanded" class="status-details">
      <div class="detail-row">
        <div class="detail-item">
          <span class="detail-label">Backend</span>
          <span class="detail-value" :class="{ ok: isBackendOk }">{{ isBackendOk ? 'Online' : 'Offline' }}</span>
        </div>
        <div class="detail-item">
          <span class="detail-label">Latency</span>
          <span class="detail-value">{{ stats.latency ? stats.latency + 'ms' : '-' }}</span>
        </div>
        <div class="detail-item">
          <span class="detail-label">Sent</span>
          <span class="detail-value">{{ stats.messagesSent }}</span>
        </div>
        <div class="detail-item">
          <span class="detail-label">Received</span>
          <span class="detail-value">{{ stats.messagesReceived }}</span>
        </div>
        <div class="detail-item">
          <span class="detail-label">Queued</span>
          <span class="detail-value">{{ stats.messagesQueued }}</span>
        </div>
        <div class="detail-item">
          <span class="detail-label">Since</span>
          <span class="detail-value">{{ formatTime(stats.lastConnected) }}</span>
        </div>
      </div>
      <div v-if="stats.lastError" class="error-row">
        <span class="error-label">Error:</span>
        <span class="error-text">{{ stats.lastError }}</span>
      </div>
      <div class="action-row">
        <button class="action-btn" @click.stop="testConnection" :disabled="isTesting">
          {{ isTesting ? 'Testing...' : 'Test' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue';
import {
  type ConnectionStats,
  type ConnectionStatus,
  wsManager,
} from '../../models/services/websocket.ts';

const isExpanded = ref(false);
const isTesting = ref(false);
const currentStatus = ref<ConnectionStatus>('initializing');
const stats = ref<ConnectionStats>(wsManager.getStats());

let unsubscribe: (() => void) | null = null;

const isBackendOk = computed(() => currentStatus.value === 'connected');

const statusText = computed(() => {
  switch (currentStatus.value) {
    case 'initializing':
      return 'Starting...';
    case 'connecting':
      return 'Connecting...';
    case 'connected':
      return 'Backend Connected';
    case 'disconnected':
      return 'Disconnected';
    case 'error':
      return 'Backend Error';
    default:
      return 'Unknown';
  }
});

const statusClass = computed(() => {
  switch (currentStatus.value) {
    case 'connected':
      return 'status-connected';
    case 'connecting':
      return 'status-connecting';
    case 'error':
      return 'status-error';
    default:
      return 'status-disconnected';
  }
});

const toggleExpand = () => {
  isExpanded.value = !isExpanded.value;
};

const testConnection = async () => {
  isTesting.value = true;
  try {
    await wsManager.testConnection();
  } finally {
    isTesting.value = false;
  }
};

const formatTime = (date: Date | null): string => {
  if (!date) return '-';
  return date.toLocaleTimeString();
};

onMounted(() => {
  unsubscribe = wsManager.onStatusChange((status, s) => {
    currentStatus.value = status;
    stats.value = s;
  });
});

onUnmounted(() => {
  if (unsubscribe) {
    unsubscribe();
  }
});
</script>

<style scoped>
.ws-status-panel {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 99999;
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  font-size: 12px;
}

.status-bar {
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  border-top: 2px solid transparent;
}

.status-bar:hover {
  filter: brightness(1.15);
}

.status-left, .status-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  transition: all 0.3s ease;
}

.status-text {
  font-weight: 600;
  letter-spacing: 0.5px;
}

.latency {
  color: inherit;
  opacity: 0.9;
}

.counter {
  opacity: 0.8;
}

.queued {
  color: #fbbf24;
}

.expand-icon {
  font-size: 10px;
  opacity: 0.6;
  margin-left: 8px;
}

.status-details {
  background: linear-gradient(180deg, #1a1a2e 0%, #16162a 100%);
  border-top: 1px solid #3a3a5a;
  padding: 12px 16px;
}

.detail-row {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  margin-bottom: 8px;
}

.detail-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 80px;
}

.detail-label {
  color: #6b7280;
  font-size: 10px;
  text-transform: uppercase;
}

.detail-value {
  color: #e5e7eb;
  font-weight: 500;
}

.detail-value.ok {
  color: #10b981;
}

.error-row {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: rgba(239, 68, 68, 0.15);
  border-radius: 4px;
  margin-bottom: 8px;
}

.error-label {
  color: #ef4444;
  font-weight: 600;
}

.error-text {
  color: #fca5a5;
  font-size: 11px;
}

.action-row {
  display: flex;
  gap: 8px;
}

.action-btn {
  background: rgba(99, 102, 241, 0.3);
  border: 1px solid rgba(99, 102, 241, 0.5);
  color: #a5b4fc;
  padding: 6px 16px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn:hover:not(:disabled) {
  background: rgba(99, 102, 241, 0.5);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Status colors */
.status-connected {
  background: linear-gradient(90deg, #064e3b 0%, #065f46 100%);
  color: #d1fae5;
  border-top-color: #34d399;
}
.status-connected .status-dot {
  background: #34d399;
  box-shadow: 0 0 8px #34d399;
}

.status-connecting {
  background: linear-gradient(90deg, #78350f 0%, #92400e 100%);
  color: #fef3c7;
  border-top-color: #fbbf24;
}
.status-connecting .status-dot {
  background: #fbbf24;
  animation: pulse 1s infinite;
}

.status-error {
  background: linear-gradient(90deg, #7f1d1d 0%, #991b1b 100%);
  color: #fee2e2;
  border-top-color: #ef4444;
}
.status-error .status-dot {
  background: #ef4444;
  box-shadow: 0 0 8px #ef4444;
  animation: blink 0.5s infinite;
}

.status-disconnected {
  background: linear-gradient(90deg, #1f2937 0%, #111827 100%);
  color: #9ca3af;
  border-top-color: #4b5563;
}
.status-disconnected .status-dot {
  background: #6b7280;
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(0.9); }
}

@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0.4; }
}
</style>
