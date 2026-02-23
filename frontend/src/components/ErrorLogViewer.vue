<script setup lang="ts">
import {
  clearErrorLogs,
  type ErrorLog,
  getErrorLogs,
  useErrorTracker,
} from '../../core/errorTracker';

const { errorLogs } = useErrorTracker();

function formatTimestamp(timestamp: string): string {
  return new Date(timestamp).toLocaleTimeString();
}

function getLevelClass(level: ErrorLog['level']): string {
  switch (level) {
    case 'error':
      return 'level-error';
    case 'warn':
      return 'level-warn';
    default:
      return 'level-info';
  }
}
</script>

<template>
  <div class="error-log-viewer">
    <div class="error-log-header">
      <h3>Error Logs ({{ errorLogs.length }})</h3>
      <button v-if="errorLogs.length > 0" @click="clearErrorLogs" class="clear-btn">
        Clear All
      </button>
    </div>

    <div v-if="errorLogs.length === 0" class="no-errors">
      No errors logged
    </div>

    <div v-else class="error-list">
      <div
        v-for="log in errorLogs"
        :key="log.id"
        :class="['error-item', getLevelClass(log.level)]"
      >
        <div class="error-header">
          <span class="error-level">{{ log.level.toUpperCase() }}</span>
          <span class="error-time">{{ formatTimestamp(log.timestamp) }}</span>
          <span class="error-context">{{ log.context }}</span>
        </div>
        <div class="error-message">{{ log.message }}</div>
        <div v-if="log.stack" class="error-stack">{{ log.stack }}</div>
        <div v-if="log.extra" class="error-extra">
          <pre>{{ JSON.stringify(log.extra, null, 2) }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.error-log-viewer {
  background: #1a1a2e;
  border-radius: 8px;
  padding: 1rem;
  max-height: 400px;
  overflow-y: auto;
}

.error-log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.error-log-header h3 {
  margin: 0;
  color: #fff;
  font-size: 1rem;
}

.clear-btn {
  background: #dc2626;
  color: white;
  border: none;
  padding: 0.25rem 0.75rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.75rem;
}

.clear-btn:hover {
  background: #b91c1c;
}

.no-errors {
  color: #9ca3af;
  text-align: center;
  padding: 2rem;
}

.error-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.error-item {
  background: #16213e;
  border-radius: 4px;
  padding: 0.75rem;
  border-left: 3px solid;
}

.level-error {
  border-left-color: #dc2626;
}

.level-warn {
  border-left-color: #f59e0b;
}

.level-info {
  border-left-color: #3b82f6;
}

.error-header {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  margin-bottom: 0.25rem;
  font-size: 0.75rem;
}

.error-level {
  font-weight: bold;
  padding: 0.125rem 0.375rem;
  border-radius: 2px;
}

.level-error .error-level {
  background: #dc2626;
  color: white;
}

.level-warn .error-level {
  background: #f59e0b;
  color: black;
}

.level-info .error-level {
  background: #3b82f6;
  color: white;
}

.error-time {
  color: #9ca3af;
}

.error-context {
  color: #a78bfa;
  font-weight: 500;
}

.error-message {
  color: #fff;
  font-family: monospace;
  font-size: 0.875rem;
  word-break: break-word;
}

.error-stack,
.error-extra {
  margin-top: 0.5rem;
  padding: 0.5rem;
  background: #0f0f23;
  border-radius: 4px;
  font-size: 0.75rem;
  color: #9ca3af;
  overflow-x: auto;
}

.error-stack {
  white-space: pre-wrap;
  word-break: break-all;
}

.error-extra pre {
  margin: 0;
  white-space: pre-wrap;
}
</style>
