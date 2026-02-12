// lib/index.js
// Infrastructure and shared utilities barrel export

export { default as Logger } from './logger.js';
export { default as WebUIBridge } from './webui-bridge.js';
export { container, singleton, transient, inject, Container } from './di.js';

// Initialize DI container with core services
import { container } from './di.js';
import Logger from './logger.js';
import WebUIBridge from './webui-bridge.js';

// Register singletons
container().registerInstance('logger', Logger);
container().registerInstance('webuiBridge', WebUIBridge);