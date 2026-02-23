<template>
  <div class="devtools-panel" :class="{ collapsed: !isOpen }">
    <div class="devtools-header" @click="togglePanel">
      <span class="devtools-title">🔧 DevTools</span>
      <div class="devtools-tabs">
        <button 
          v-for="tab in tabs" 
          :key="tab.id"
          :class="['tab-btn', { active: activeTab === tab.id }]"
          @click.stop="activeTab = tab.id"
        >
          {{ tab.icon }} {{ tab.label }}
          <span v-if="tab.count" class="tab-count">{{ tab.count }}</span>
        </button>
      </div>
      <span class="toggle-icon">{{ isOpen ? '▼' : '▲' }}</span>
    </div>
    
    <div v-if="isOpen" class="devtools-content">
      <!-- Console Tab -->
      <div v-show="activeTab === 'console'" class="tab-content">
        <div class="toolbar">
          <button @click="clearLogs">Clear</button>
          <button @click="filterLog = ''" :class="{ active: filterLog === '' }">All</button>
          <button @click="filterLog = 'info'" :class="{ active: filterLog === 'info' }">Info</button>
          <button @click="filterLog = 'warn'" :class="{ active: filterLog === 'warn' }">Warn</button>
          <button @click="filterLog = 'error'" :class="{ active: filterLog === 'error' }">Error</button>
          <input v-model="filterText" placeholder="Filter..." class="filter-input" />
        </div>
        <div class="log-container" ref="logContainer">
          <div 
            v-for="(log, index) in filteredLogs" 
            :key="index"
            :class="['log-entry', log.level]"
          >
            <span class="log-time">{{ formatTime(log.time) }}</span>
            <span :class="['log-level', log.level]">{{ log.level.toUpperCase() }}</span>
            <span class="log-source">[{{ log.source }}]</span>
            <span class="log-message">{{ log.message }}</span>
          </div>
          <div v-if="filteredLogs.length === 0" class="empty-state">No logs</div>
        </div>
      </div>

      <!-- Errors Tab -->
      <div v-show="activeTab === 'errors'" class="tab-content">
        <div class="toolbar">
          <button @click="clearErrors">Clear Errors</button>
          <span class="error-count">{{ errors.length }} error(s)</span>
        </div>
        <div class="log-container">
          <div 
            v-for="(error, index) in errors" 
            :key="index"
            class="log-entry error"
          >
            <span class="log-time">{{ formatTime(error.time) }}</span>
            <span class="log-message">
              <strong>{{ error.source }}</strong>: {{ error.message }}
            </span>
            <pre v-if="error.stack" class="error-stack">{{ error.stack }}</pre>
          </div>
          <div v-if="errors.length === 0" class="empty-state">No errors</div>
        </div>
      </div>

      <!-- Network Tab -->
      <div v-show="activeTab === 'network'" class="tab-content">
        <div class="toolbar">
          <button @click="clearNetwork">Clear</button>
          <span class="network-count">{{ networkRequests.length }} request(s)</span>
        </div>
        <div class="log-container">
          <div 
            v-for="(req, index) in networkRequests" 
            :key="index"
            :class="['log-entry', req.status >= 400 ? 'error' : req.status >= 300 ? 'warn' : 'info']"
          >
            <span class="log-time">{{ formatTime(req.time) }}</span>
            <span class="log-method">{{ req.method }}</span>
            <span class="log-url">{{ req.url }}</span>
            <span :class="['log-status', req.status >= 400 ? 'error' : '']">{{ req.status || 'pending' }}</span>
          </div>
          <div v-if="networkRequests.length === 0" class="empty-state">No network requests</div>
        </div>
      </div>

      <!-- System Tab -->
      <div v-show="activeTab === 'system'" class="tab-content system-info">
        <div class="info-grid">
          <div class="info-card">
            <h4>Frontend</h4>
            <p><strong>Version:</strong> {{ systemInfo.frontendVersion }}</p>
            <p><strong>Mode:</strong> {{ systemInfo.mode }}</p>
            <p><strong>User Agent:</strong> {{ systemInfo.userAgent }}</p>
            <p><strong>Language:</strong> {{ systemInfo.language }}</p>
          </div>
          <div class="info-card">
            <h4>Runtime</h4>
            <p><strong>Memory:</strong> {{ systemInfo.memory }}</p>
            <p><strong>CPU Cores:</strong> {{ systemInfo.cores }}</p>
            <p><strong>Screen:</strong> {{ systemInfo.screen }}</p>
            <p><strong>Online:</strong> {{ systemInfo.online ? 'Yes' : 'No' }}</p>
          </div>
          <div class="info-card">
            <h4>Performance</h4>
            <p><strong>Load Time:</strong> {{ systemInfo.loadTime }}ms</p>
            <p><strong>Uptime:</strong> {{ systemInfo.uptime }}</p>
            <p><strong>FPS:</strong> <span id="fps-display">--</span></p>
          </div>
          <div class="info-card" v-if="backendInfo.platform">
            <h4>Backend (Rust)</h4>
            <p><strong>Platform:</strong> {{ backendInfo.platform }}</p>
            <p><strong>Architecture:</strong> {{ backendInfo.arch }}</p>
            <p><strong>Hostname:</strong> {{ backendInfo.hostname }}</p>
            <p><strong>Kernel:</strong> {{ backendInfo.kernel_version }}</p>
          </div>
        </div>
        <button @click="fetchBackendInfo" class="refresh-btn">🔄 Refresh Backend Info</button>
      </div>

      <!-- State Tab -->
      <div v-show="activeTab === 'state'" class="tab-content">
        <div class="toolbar">
          <button @click="refreshState">🔄 Refresh</button>
        </div>
        <div class="state-container">
          <div class="state-section">
            <h4>Pinia Stores</h4>
            <pre>{{ JSON.stringify(storesState, null, 2) }}</pre>
          </div>
        </div>
      </div>

      <!-- Actions Tab -->
      <div v-show="activeTab === 'actions'" class="tab-content">
        <div class="actions-grid">
          <button class="action-btn" @click="triggerError">⚠️ Trigger Test Error</button>
          <button class="action-btn" @click="logTest">📝 Log Test Message</button>
          <button class="action-btn" @click="warnTest">⚡ Log Warning</button>
          <button class="action-btn" @click="callBackend">🔙 Call Backend</button>
          <button class="action-btn" @click="clearAll">🗑️ Clear All</button>
          <button class="action-btn" @click="exportLogs">📤 Export Logs</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useErrorTracker } from '../core/errorTracker';
import { webui } from '../services/webui';

interface LogEntry {
  time: number;
  level: 'info' | 'warn' | 'error' | 'debug';
  source: string;
  message: string;
}

interface ErrorEntry {
  time: number;
  source: string;
  message: string;
  stack?: string;
}

interface NetworkRequest {
  time: number;
  method: string;
  url: string;
  status?: number;
}

const isOpen = ref(false);
const activeTab = ref('console');
const filterLog = ref('');
const filterText = ref('');
const logs = ref<LogEntry[]>([]);
const errors = ref<ErrorEntry[]>([]);
const networkRequests = ref<NetworkRequest[]>([]);
const storesState = ref({});
const backendInfo = ref<Record<string, string>>({});
const logContainer = ref<HTMLElement | null>(null);

const systemInfo = ref({
  frontendVersion: '1.0.0',
  mode: (import.meta as any).env?.MODE || 'production',
  userAgent: navigator.userAgent,
  language: navigator.language,
  memory: 'N/A',
  cores: navigator.hardwareConcurrency || 'N/A',
  screen: `${window.screen.width}x${window.screen.height}`,
  online: navigator.onLine,
  loadTime: (performance as any).timing?.loadEventEnd || 0,
  uptime: 'N/A',
});

const tabs = computed(() => [
  { id: 'console', label: 'Console', icon: '📋', count: logs.value.length },
  { id: 'errors', label: 'Errors', icon: '❌', count: errors.value.length },
  { id: 'network', label: 'Network', icon: '🌐', count: networkRequests.value.length },
  { id: 'system', label: 'System', icon: '💻', count: 0 },
  { id: 'state', label: 'State', icon: '📦', count: 0 },
  { id: 'actions', label: 'Actions', icon: '⚡', count: 0 },
]);

const filteredLogs = computed(() => {
  return logs.value.filter(log => {
    if (filterLog.value && log.level !== filterLog.value) return false;
    if (filterText.value && !log.message.toLowerCase().includes(filterText.value.toLowerCase())) return false;
    return true;
  });
});

function togglePanel() {
  isOpen.value = !isOpen.value;
}

function addLog(source: string, message: string, level: LogEntry['level'] = 'info') {
  logs.value.push({ time: Date.now(), level, source, message });
  scrollToBottom();
}

function addError(source: string, message: string, stack?: string) {
  errors.value.push({ time: Date.now(), source, message, stack });
  addLog(source, message, 'error');
}

function clearLogs() {
  logs.value = [];
}

function clearErrors() {
  errors.value = [];
}

function clearNetwork() {
  networkRequests.value = [];
}

function clearAll() {
  clearLogs();
  clearErrors();
  clearNetwork();
}

function formatTime(timestamp: number): string {
  const d = new Date(timestamp);
  return d.toLocaleTimeString('en-US', { hour12: false }) + '.' + String(d.getMilliseconds()).padStart(3, '0');
}

function scrollToBottom() {
  if (logContainer.value) {
    logContainer.value.scrollTop = logContainer.value.scrollHeight;
  }
}

function updateSystemInfo() {
  if (performance.memory) {
    systemInfo.value.memory = Math.round((performance as any).memory.usedJSHeapSize / 1048576) + ' MB';
  }
  systemInfo.value.uptime = Math.floor(performance.now() / 1000) + 's';
  
  if (window.__webui__) {
    systemInfo.value.mode = 'production (WebUI)';
  }
}

async function fetchBackendInfo() {
  try {
    const response = await webui.call<{ success: boolean; data?: Record<string, string> }>('get_system_info');
    if (response.success && response.data) {
      backendInfo.value = response.data;
      addLog('System', 'Backend info fetched', 'info');
    }
  } catch (e) {
    addError('System', `Failed to fetch backend info: ${e}`);
  }
}

function refreshState() {
  try {
    const state: Record<string, unknown> = {};
    
    if ((window as any).useUserStore) {
      const userStore = (window as any).useUserStore();
      state.userStore = userStore.$state;
    }
    if ((window as any).useSystemStore) {
      const systemStore = (window as any).useSystemStore();
      state.systemStore = systemStore.$state;
    }
    
    storesState.value = state;
    addLog('State', 'State refreshed', 'info');
  } catch (e) {
    addError('State', `Failed to refresh state: ${e}`);
  }
}

// Test actions
function triggerError() {
  throw new Error('Test error from DevTools');
}

function logTest() {
  addLog('DevTools', 'This is a test log message', 'info');
}

function warnTest() {
  addLog('DevTools', 'This is a test warning', 'warn');
}

async function callBackend() {
  try {
    const response = await webui.call('ping_backend');
    addLog('Backend', `Response: ${JSON.stringify(response)}`, 'info');
  } catch (e) {
    addError('Backend', `Call failed: ${e}`);
  }
}

function exportLogs() {
  const data = {
    logs: logs.value,
    errors: errors.value,
    network: networkRequests.value,
    exportedAt: new Date().toISOString(),
  };
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `devtools-logs-${Date.now()}.json`;
  a.click();
  URL.revokeObjectURL(url);
  addLog('DevTools', 'Logs exported', 'info');
}

// FPS counter
let frameCount = 0;
let lastFpsUpdate = 0;
function updateFPS() {
  frameCount++;
  const now = performance.now();
  if (now - lastFpsUpdate >= 1000) {
    const fps = frameCount;
    frameCount = 0;
    lastFpsUpdate = now;
    const fpsEl = document.getElementById('fps-display');
    if (fpsEl) fpsEl.textContent = String(fps);
  }
  requestAnimationFrame(updateFPS);
}

// Intercept console
function setupConsoleIntercept() {
  const originalLog = console.log;
  const originalWarn = console.warn;
  const originalError = console.error;

  console.log = (...args) => {
    originalLog.apply(console, args);
    addLog('console', args.map(a => typeof a === 'object' ? JSON.stringify(a) : String(a)).join(' '), 'info');
  };
  console.warn = (...args) => {
    originalWarn.apply(console, args);
    addLog('console', args.map(a => typeof a === 'object' ? JSON.stringify(a) : String(a)).join(' '), 'warn');
  };
  console.error = (...args) => {
    originalError.apply(console, args);
    addLog('console', args.map(a => typeof a === 'object' ? JSON.stringify(a) : String(a)).join(' '), 'error');
  };

  window.addEventListener('error', (e) => {
    addError('window', e.message, e.error?.stack);
  });

  window.addEventListener('unhandledrejection', (e) => {
    addError('Promise', String(e.reason));
  });
}

// Network intercept
function setupNetworkIntercept() {
  const originalFetch = window.fetch;
  window.fetch = async (...args) => {
    const url = typeof args[0] === 'string' ? args[0] : (args[0] as Request).url;
    const method = args[1]?.method || 'GET';
    const reqTime = Date.now();
    
    networkRequests.value.push({ time: reqTime, method, url });
    
    try {
      const response = await originalFetch.apply(this, args);
      const idx = networkRequests.value.findIndex(r => r.time === reqTime && r.url === url);
      if (idx >= 0) {
        networkRequests.value[idx].status = response.status;
      }
      return response;
    } catch (e) {
      const idx = networkRequests.value.findIndex(r => r.time === reqTime && r.url === url);
      if (idx >= 0) {
        networkRequests.value[idx].status = 0;
      }
      throw e;
    }
  };
}

let fpsAnimId: number;
onMounted(() => {
  addLog('DevTools', 'Panel initialized', 'info');
  setupConsoleIntercept();
  setupNetworkIntercept();
  updateSystemInfo();
  refreshState();
  
  setInterval(updateSystemInfo, 5000);
  fpsAnimId = requestAnimationFrame(updateFPS);
  
  systemInfo.value.loadTime = Math.round(performance.now());
});

onUnmounted(() => {
  cancelAnimationFrame(fpsAnimId);
});

defineExpose({ addLog, addError, isOpen });
</script>

<style scoped>
.devtools-panel {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background: #1e1e1e;
  color: #d4d4d4;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  z-index: 99999;
  box-shadow: 0 -2px 10px rgba(0,0,0,0.5);
  max-height: 50vh;
  display: flex;
  flex-direction: column;
}

.devtools-panel.collapsed {
  max-height: 32px;
}

.devtools-header {
  display: flex;
  align-items: center;
  padding: 6px 12px;
  background: #2d2d2d;
  cursor: pointer;
  user-select: none;
}

.devtools-title {
  font-weight: bold;
  margin-right: 20px;
}

.devtools-tabs {
  display: flex;
  gap: 4px;
  flex: 1;
}

.tab-btn {
  background: #3c3c3c;
  border: none;
  color: #d4d4d4;
  padding: 4px 10px;
  cursor: pointer;
  font-size: 11px;
  border-radius: 3px;
}

.tab-btn:hover {
  background: #505050;
}

.tab-btn.active {
  background: #007acc;
}

.tab-count {
  background: #d4d4d4;
  color: #1e1e1e;
  border-radius: 8px;
  padding: 1px 5px;
  font-size: 10px;
  margin-left: 4px;
}

.toggle-icon {
  color: #808080;
}

.devtools-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.tab-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: 8px;
}

.toolbar {
  display: flex;
  gap: 8px;
  margin-bottom: 8px;
  align-items: center;
}

.toolbar button {
  background: #3c3c3c;
  border: 1px solid #555;
  color: #d4d4d4;
  padding: 3px 8px;
  cursor: pointer;
  font-size: 11px;
}

.toolbar button:hover {
  background: #505050;
}

.toolbar button.active {
  background: #007acc;
}

.filter-input {
  background: #2d2d2d;
  border: 1px solid #555;
  color: #d4d4d4;
  padding: 3px 8px;
  font-size: 11px;
  margin-left: auto;
}

.log-container {
  flex: 1;
  overflow-y: auto;
  background: #1a1a1a;
  padding: 8px;
  border: 1px solid #333;
}

.log-entry {
  display: flex;
  gap: 8px;
  padding: 2px 0;
  border-bottom: 1px solid #2a2a2a;
}

.log-entry.info .log-level { color: #4ec9b0; }
.log-entry.warn .log-level { color: #dcdcaa; }
.log-entry.error .log-level { color: #f48771; }
.log-entry.debug .log-level { color: #808080; }

.log-time {
  color: #6a9955;
  flex-shrink: 0;
}

.log-level {
  flex-shrink: 0;
  width: 50px;
}

.log-source {
  color: #569cd6;
  flex-shrink: 0;
}

.log-message {
  color: #d4d4d4;
  word-break: break-all;
}

.error-stack {
  color: #f48771;
  font-size: 10px;
  margin: 4px 0 0 60px;
  white-space: pre-wrap;
  background: #2a1a1a;
  padding: 4px;
}

.empty-state {
  color: #808080;
  text-align: center;
  padding: 20px;
}

.error-count, .network-count {
  color: #f48771;
  margin-left: auto;
}

/* System Info */
.system-info {
  overflow-y: auto;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 12px;
  margin-bottom: 12px;
}

.info-card {
  background: #2a2a2a;
  padding: 12px;
  border-radius: 4px;
}

.info-card h4 {
  margin: 0 0 8px 0;
  color: #4ec9b0;
}

.info-card p {
  margin: 4px 0;
  color: #d4d4d4;
}

.refresh-btn {
  background: #3c3c3c;
  border: none;
  color: #d4d4d4;
  padding: 8px 16px;
  cursor: pointer;
}

/* State */
.state-container {
  flex: 1;
  overflow-y: auto;
}

.state-section {
  margin-bottom: 12px;
}

.state-section h4 {
  margin: 0 0 8px 0;
  color: #4ec9b0;
}

.state-section pre {
  background: #1a1a1a;
  padding: 8px;
  overflow-x: auto;
}

/* Actions */
.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 8px;
}

.action-btn {
  background: #3c3c3c;
  border: none;
  color: #d4d4d4;
  padding: 12px;
  cursor: pointer;
  font-size: 12px;
  border-radius: 4px;
}

.action-btn:hover {
  background: #505050;
}
</style>
