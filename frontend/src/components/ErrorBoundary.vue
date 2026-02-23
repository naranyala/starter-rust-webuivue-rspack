<script setup lang="ts">
import { onErrorCaptured, ref } from 'vue';
import { logError } from '../core/errorTracker';

const hasError = ref(false);
const errorMessage = ref('');
const errorInfo = ref('');

onErrorCaptured((err, instance, info) => {
  hasError.value = true;
  errorMessage.value = err instanceof Error ? err.message : String(err);
  errorInfo.value = info;

  logError('ErrorBoundary', err, {
    componentName: instance?.$options?.name || instance?.$options?.__name || 'Unknown',
    info,
  });

  return false;
});

function handleRetry() {
  hasError.value = false;
  errorMessage.value = '';
  errorInfo.value = '';
}

function handleReload() {
  window.location.reload();
}
</script>

<template>
  <div v-if="hasError" class="error-boundary">
    <div class="error-content">
      <div class="error-icon">⚠️</div>
      <h2>Something went wrong</h2>
      <p class="error-message">{{ errorMessage }}</p>
      <p class="error-info">{{ errorInfo }}</p>
      <div class="error-actions">
        <button @click="handleRetry" class="btn btn-secondary">Try Again</button>
        <button @click="handleReload" class="btn btn-primary">Reload Page</button>
      </div>
    </div>
  </div>
  <slot v-else></slot>
</template>

<style scoped>
.error-boundary {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.error-content {
  text-align: center;
  color: #fff;
  padding: 2rem;
  max-width: 500px;
}

.error-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.error-message {
  color: #f87171;
  font-size: 1.25rem;
  margin: 1rem 0;
}

.error-info {
  color: #9ca3af;
  font-size: 0.875rem;
  margin-bottom: 2rem;
  word-break: break-all;
}

.error-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.btn {
  padding: 0.75rem 1.5rem;
  border-radius: 0.5rem;
  font-weight: 500;
  cursor: pointer;
  border: none;
  transition: all 0.2s;
}

.btn-primary {
  background: #3b82f6;
  color: white;
}

.btn-primary:hover {
  background: #2563eb;
}

.btn-secondary {
  background: #4b5563;
  color: white;
}

.btn-secondary:hover {
  background: #374151;
}
</style>
