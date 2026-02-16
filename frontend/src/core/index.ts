export { pluginManager, type Plugin, type PluginRegistry } from './plugin';
export { connectionManager, type ConnectionStatus, type ConnectionStats } from './connection';
export {
  AppError,
  type Result,
  ok,
  err,
  isOk,
  isErr,
  map,
  mapErr,
  tryCatch,
} from './error';
