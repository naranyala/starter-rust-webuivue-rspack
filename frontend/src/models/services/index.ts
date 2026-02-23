export { useApi } from './api/apiClient.ts';
// Legacy exports
export {
  type ConnectionState,
  type ConnectionStateInfo,
  connectionManager,
} from './connection/connectionManager.ts';
export { useDatabase } from './database/databaseService.ts';
export { createEventBus, eventBus, useEventBus } from './eventBus/eventBusService.ts';
// WebUI connection
export {
  getConnectionStatus,
  getPortInfo,
  sendToBackend,
  testConnection,
  webuiConn,
} from './webuiConnection.ts';
// Window tracker
export { useWindowTracker, windowTracker } from './window/simpleTracker.ts';
