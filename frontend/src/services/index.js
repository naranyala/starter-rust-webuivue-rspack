// lib/index.js
// Infrastructure and shared utilities barrel export

export { Container, container, inject, singleton, transient } from './di.js';
export { default as Logger } from './logger.js';
export { default as WebUIBridge } from './webui-bridge.js';

// Initialize DI container with core services
import { container } from './di.js';
import Logger from './logger.js';
import WebUIBridge from './webui-bridge.js';

// Register singletons
container().registerInstance('logger', Logger);
container().registerInstance('webuiBridge', WebUIBridge);
