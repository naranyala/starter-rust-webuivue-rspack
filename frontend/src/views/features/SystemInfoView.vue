<template>
  <div class="system-info">
    <div class="system-header">
      <h2>[System] System Dashboard</h2>
      <Button variant="primary" @click="refreshInfo">Refresh</Button>
    </div>

    <div v-if="systemStore.loading" class="loading">Loading...</div>
    <div v-else-if="systemStore.error" class="error">{{ systemStore.error }}</div>
    <div v-else class="system-content">
      <div class="article-content">
        <h3>About System Information</h3>
        <p>System information provides critical insights into the health and performance of computing infrastructure.</p>
      </div>
      
      <div class="overview-section">
        <div class="info-grid">
          <div class="info-card">
            <div class="info-icon">[Platform]</div>
            <h3>Platform</h3>
            <p>{{ systemStore.systemInfo?.platform || 'N/A' }}</p>
          </div>
          <div class="info-card">
            <div class="info-icon">[Arch]</div>
            <h3>Architecture</h3>
            <p>{{ systemStore.systemInfo?.arch || 'N/A' }}</p>
          </div>
          <div class="info-card">
            <div class="info-icon">[OS]</div>
            <h3>OS Family</h3>
            <p>{{ systemStore.systemInfo?.family || 'N/A' }}</p>
          </div>
          <div class="info-card">
            <div class="info-icon">📋</div>
            <h3>OS Detail</h3>
            <p>{{ systemStore.systemInfo?.os_detail || 'N/A' }}</p>
          </div>
        </div>

        <div class="uptime-card">
          <div class="uptime-info">
            <div class="uptime-icon">⏱️</div>
            <div>
              <h3>System Uptime</h3>
              <p class="uptime-value">{{ formatUptime(systemStore.uptime) }}</p>
            </div>
          </div>
        </div>
      </div>

      <div class="resource-section">
        <h3>📊 Resource Usage</h3>
        <div class="resource-grid">
          <div class="resource-card">
            <div class="resource-header"><span>CPU</span><span>42%</span></div>
            <div class="progress-bar"><div class="progress-fill cpu" style="width: 42%"></div></div>
          </div>
          <div class="resource-card">
            <div class="resource-header"><span>Memory</span><span>65%</span></div>
            <div class="progress-bar"><div class="progress-fill memory" style="width: 65%"></div></div>
          </div>
          <div class="resource-card">
            <div class="resource-header"><span>Disk</span><span>30%</span></div>
            <div class="progress-bar"><div class="progress-fill disk" style="width: 30%"></div></div>
          </div>
        </div>
      </div>

      <div class="additional-section">
        <h3>📋 Additional Information</h3>
        <div class="additional-grid">
          <div class="additional-card"><h4>Hostname</h4><p>{{ systemStore.systemInfo?.hostname || 'N/A' }}</p></div>
          <div class="additional-card"><h4>Kernel</h4><p>{{ systemStore.systemInfo?.kernel_version || 'N/A' }}</p></div>
          <div class="additional-card"><h4>Boot Time</h4><p>{{ formatTimestamp(systemStore.systemInfo?.boot_time) }}</p></div>
          <div class="additional-card"><h4>System Time</h4><p>{{ currentTime }}</p></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import Button from '../../components/common/Button.vue';
import { useSystemInfoViewModel } from '../../viewmodels/useSystemInfoViewModel.ts';

const { systemStore, currentTime, refreshInfo, formatUptime, formatTimestamp } =
  useSystemInfoViewModel();
</script>

<style scoped>
.system-info { padding: 1rem; height: 100%; display: flex; flex-direction: column; overflow-y: auto; }
.system-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem; }
.system-content { flex: 1; display: flex; flex-direction: column; gap: 1.5rem; }
.overview-section { display: flex; gap: 1rem; flex-wrap: wrap; }
.info-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; flex: 1; }
.info-card { background: white; border-radius: 0.5rem; padding: 1rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.info-icon { font-size: 1.5rem; }
.info-card h3 { font-size: 0.875rem; color: #6b7280; margin: 0.5rem 0 0; }
.info-card p { font-size: 1.1rem; font-weight: 600; color: #1f2937; margin: 0; }
.uptime-card { background: white; border-radius: 0.5rem; padding: 1rem; min-width: 250px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.uptime-info { display: flex; align-items: center; gap: 1rem; }
.uptime-icon { font-size: 2rem; }
.uptime-value { font-size: 1.5rem; font-weight: 700; color: #1f2937; margin: 0; }
.resource-section, .additional-section { background: white; border-radius: 0.5rem; padding: 1rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.resource-section h3, .additional-section h3 { margin: 0 0 1rem; color: #1f2937; }
.resource-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; }
.resource-card { background: #f9fafb; border-radius: 0.5rem; padding: 1rem; }
.resource-header { display: flex; justify-content: space-between; margin-bottom: 0.75rem; font-weight: 600; }
.progress-bar { width: 100%; height: 8px; background: #e5e7eb; border-radius: 4px; overflow: hidden; }
.progress-fill { height: 100%; border-radius: 4px; }
.progress-fill.cpu { background: linear-gradient(90deg, #ef4444, #f87171); }
.progress-fill.memory { background: linear-gradient(90deg, #3b82f6, #60a5fa); }
.progress-fill.disk { background: linear-gradient(90deg, #10b981, #34d399); }
.additional-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; }
.additional-card { background: #f9fafb; border-radius: 0.5rem; padding: 1rem; }
.additional-card h4 { margin: 0 0 0.5rem; font-size: 0.875rem; color: #6b7280; }
.additional-card p { margin: 0; font-weight: 500; color: #1f2937; }
.loading, .error { padding: 1rem; text-align: center; }
.error { color: #dc2626; background: #fee2e2; border-radius: 0.5rem; }
.article-content { background: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.article-content h3 { color: #4f46e5; margin: 0 0 1rem; }
.article-content p { line-height: 1.6; color: #4b5563; }
</style>
