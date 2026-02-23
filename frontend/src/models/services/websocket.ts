// WebSocket communication layer - connects to Rust backend WebSocket server
// Uses WebSocket protocol to communicate with Rust backend

import type { WindowStateChangePayload } from '../types';

const DEBUG = false;
const log = (...args: unknown[]) => {
  if (DEBUG) console.log('[WebSocket]', ...args);
};

export type ConnectionStatus =
  | 'initializing'
  | 'connecting'
  | 'connected'
  | 'disconnected'
  | 'error';

interface ConnectionStats {
  status: ConnectionStatus;
  lastConnected: Date | null;
  lastError: string | null;
  lastErrorAt: Date | null;
  messagesSent: number;
  messagesReceived: number;
  messagesQueued: number;
  latency: number | null;
}

type StatusChangeCallback = (
  status: ConnectionStatus,
  stats: ConnectionStats,
  prevStatus: ConnectionStatus | null
) => void;
type ErrorCallback = (error: Error, context: string, stats: ConnectionStats) => void;
type MessageHandler = (data: any) => void;

class WebSocketManager {
  private status: ConnectionStatus = 'initializing';
  private prevStatus: ConnectionStatus | null = null;
  private messageQueue: WindowStateChangePayload[] = [];
  private statusListeners: Set<StatusChangeCallback> = new Set();
  private errorListeners: Set<ErrorCallback> = new Set();
  private messageHandlers: Set<MessageHandler> = new Set();
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 3000;
  private pingInterval: ReturnType<typeof setInterval> | null = null;
  private isBackendInitialized = false;

  private stats: ConnectionStats = {
    status: 'initializing',
    lastConnected: null,
    lastError: null,
    lastErrorAt: null,
    messagesSent: 0,
    messagesReceived: 0,
    messagesQueued: 0,
    latency: null,
  };

  private readonly MESSAGE_QUEUE_MAX = 50;
  private readonly PING_INTERVAL = 15000;

  constructor() {
    this.init();
  }

  private async init() {
    this.setStatus('initializing');

    try {
      // Get WebSocket port from backend
      const wsPort = await this.getWebSocketPort();
      if (wsPort) {
        this.connectToWebSocket(wsPort);
      } else {
        this.setStatus('error');
        this.stats.lastError = 'Could not get WebSocket port from backend';
        this.stats.lastErrorAt = new Date();
        log('Could not get WebSocket port from backend');
      }
    } catch (error) {
      this.setStatus('error');
      this.stats.lastError =
        error instanceof Error ? error.message : 'Failed to initialize WebSocket';
      this.stats.lastErrorAt = new Date();
      log('Failed to initialize WebSocket:', error);
    }
  }

  private async getWebSocketPort(): Promise<number | null> {
    // Try to get port from backend via WebUI call
    return new Promise((resolve) => {
      try {
        // Set up a callback to receive the port
        (window as any)._webui_port_callback = (port: number) => {
          log('Received WebSocket port from backend:', port);
          resolve(port);
        };

        // Check if WebUI is available to get the port
        if ((window as any).webui?.call || (window as any).__webui__?.call) {
          const webui = (window as any).webui || (window as any).__webui__;
          webui.call('get_port_info');

          // Set a timeout in case the callback doesn't come back
          setTimeout(() => {
            if ((window as any)._webui_port_callback) {
              log('Timeout getting WebSocket port, using fallback');
              resolve(9876); // fallback port
            }
          }, 3000);
        } else {
          log('WebUI not available, using fallback port');
          resolve(9876); // fallback port
        }
      } catch (error) {
        log('Error getting port via WebUI:', error);
        resolve(9876); // fallback port
      }
    });
  }

  private connectToWebSocket(port: number) {
    try {
      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      const wsUrl = `${protocol}//${window.location.hostname}:${port}`;

      log('Connecting to WebSocket:', wsUrl);
      this.setStatus('connecting');

      this.ws = new WebSocket(wsUrl);

      this.ws.onopen = () => {
        log('WebSocket connected');
        this.reconnectAttempts = 0;
        this.isBackendInitialized = true;
        this.setStatus('connected');
        this.stats.lastConnected = new Date();
        this.startPing();

        // Process queued messages
        this.processQueue();
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          log('Received message:', data);

          this.stats.messagesReceived++;
          this.stats.lastConnected = new Date();

          // Handle ping/pong
          if (data.type === 'ping') {
            this.send({ type: 'pong', timestamp: Date.now() });
          }

          // Handle response messages by dispatching custom events
          if (data.request_id && data.success !== undefined) {
            // This is a response to a request, dispatch a custom event
            let eventType = 'general_response';

            // Map request types to specific event types
            if (data.request_type) {
              if (data.request_type === 'get_system_info') {
                eventType = 'sysinfo_response';
              } else if (data.request_type === 'get_uptime') {
                eventType = 'uptime_response';
              } else {
                eventType = `${data.request_type.replace('get_', '')}_response`;
              }
            }

            const customEvent = new CustomEvent(eventType, { detail: data });
            window.dispatchEvent(customEvent);
          }

          // Notify message handlers
          this.messageHandlers.forEach((handler) => {
            try {
              handler(data);
            } catch (e) {
              console.error('Error in message handler:', e);
            }
          });

          if (this.status !== 'connected') {
            this.setStatus('connected');
          }
        } catch (error) {
          log('Error parsing message:', error);
        }
      };

      this.ws.onclose = (event) => {
        log('WebSocket closed:', event.code, event.reason);
        this.setStatus('disconnected');
        this.cleanup();

        // Attempt to reconnect
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
          setTimeout(() => {
            this.reconnectAttempts++;
            log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
            this.connectToWebSocket(port);
          }, this.reconnectDelay);
        } else {
          this.setStatus('error');
          this.stats.lastError = 'Max reconnection attempts reached';
          this.stats.lastErrorAt = new Date();
        }
      };

      this.ws.onerror = (error) => {
        log('WebSocket error:', error);
        this.stats.lastError = error.toString();
        this.stats.lastErrorAt = new Date();
        this.setStatus('error');
      };
    } catch (error) {
      log('Error creating WebSocket connection:', error);
      this.setStatus('error');
      this.stats.lastError =
        error instanceof Error ? error.message : 'Failed to create WebSocket connection';
      this.stats.lastErrorAt = new Date();
    }
  }

  private startPing() {
    if (this.pingInterval) {
      clearInterval(this.pingInterval);
    }

    this.pingInterval = setInterval(() => {
      if (this.status === 'connected' && this.ws?.readyState === WebSocket.OPEN) {
        try {
          const startTime = Date.now();
          this.ws.send(JSON.stringify({ type: 'ping', timestamp: startTime }));
          this.stats.latency = Date.now() - startTime;
        } catch (error) {
          log('Ping failed:', error);
          this.stats.lastError = error instanceof Error ? error.message : 'Ping failed';
          this.stats.lastErrorAt = new Date();
          this.setStatus('error');
        }
      }
    }, this.PING_INTERVAL);
  }

  private processQueue() {
    if (this.status === 'connected' && this.ws?.readyState === WebSocket.OPEN) {
      while (this.messageQueue.length > 0) {
        const payload = this.messageQueue.shift();
        if (payload) {
          this.send(payload);
        }
      }
      this.stats.messagesQueued = 0;
    }
  }

  private cleanup() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }

    if (this.pingInterval) {
      clearInterval(this.pingInterval);
      this.pingInterval = null;
    }
  }

  private setStatus(status: ConnectionStatus) {
    if (this.status === status) return;

    this.prevStatus = this.status;
    this.status = status;
    this.stats.status = status;

    log(`Status: ${this.prevStatus} -> ${status}`);
    this.notifyStatusChange();
  }

  private notifyStatusChange() {
    const currentStats = { ...this.stats };
    this.statusListeners.forEach((listener) => {
      try {
        listener(this.status, currentStats, this.prevStatus);
      } catch (e) {
        console.error('Error in status listener:', e);
      }
    });
  }

  async send(payload: WindowStateChangePayload): Promise<boolean> {
    if (this.status !== 'connected' || !this.ws || this.ws.readyState !== WebSocket.OPEN) {
      // Queue the message if not connected
      this.messageQueue.push(payload);
      this.stats.messagesQueued = this.messageQueue.length;
      if (this.status !== 'connecting') {
        this.setStatus('disconnected');
      }
      return false;
    }

    try {
      const jsonStr = JSON.stringify(payload);
      this.ws.send(jsonStr);
      this.stats.messagesSent++;
      this.notifyStatusChange();
      return true;
    } catch (error) {
      log('Send error:', error);
      this.stats.lastError = error instanceof Error ? error.message : 'Send failed';
      this.stats.lastErrorAt = new Date();
      this.messageQueue.push(payload);
      this.stats.messagesQueued = this.messageQueue.length;
      this.setStatus('error');
      return false;
    }
  }

  onStatusChange(callback: StatusChangeCallback): () => void {
    this.statusListeners.add(callback);
    callback(this.status, { ...this.stats }, this.prevStatus);

    return () => {
      this.statusListeners.delete(callback);
    };
  }

  onError(callback: ErrorCallback): () => void {
    this.errorListeners.add(callback);
    return () => {
      this.errorListeners.delete(callback);
    };
  }

  onMessage(callback: MessageHandler): () => void {
    this.messageHandlers.add(callback);
    return () => {
      this.messageHandlers.delete(callback);
    };
  }

  getStatus(): ConnectionStatus {
    return this.status;
  }

  getStats(): ConnectionStats {
    return { ...this.stats };
  }

  isConnected(): boolean {
    return this.status === 'connected';
  }

  async testConnection(): Promise<boolean> {
    if (this.status === 'connected' && this.ws?.readyState === WebSocket.OPEN) {
      try {
        const startTime = Date.now();
        this.ws.send(JSON.stringify({ type: 'ping', timestamp: startTime }));
        this.stats.latency = Date.now() - startTime;
        this.setStatus('connected');
        return true;
      } catch (error) {
        this.stats.lastError = error instanceof Error ? error.message : 'Ping failed';
        this.stats.lastErrorAt = new Date();
        this.setStatus('error');
        return false;
      }
    }

    // If not connected, try to get the port and reconnect
    try {
      const wsPort = await this.getWebSocketPort();
      if (wsPort) {
        this.cleanup();
        this.connectToWebSocket(wsPort);
        return true;
      }
    } catch (error) {
      this.stats.lastError = error instanceof Error ? error.message : 'Test connection failed';
      this.stats.lastErrorAt = new Date();
      this.setStatus('error');
    }

    return false;
  }

  dispose() {
    this.cleanup();
    this.statusListeners.clear();
    this.errorListeners.clear();
    this.messageHandlers.clear();
  }
}

export const wsManager = new WebSocketManager();

export async function sendWindowState(payload: WindowStateChangePayload): Promise<boolean> {
  return wsManager.send(payload);
}

export type { ConnectionStats };
