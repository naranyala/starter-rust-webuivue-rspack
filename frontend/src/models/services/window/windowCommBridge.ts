// Bidirectional communication bridge - uses ConnectionManager for robust handling

import { connectionManager } from '../connection/connectionManager.ts';
import type { WindowStateChangePayload } from '../types';

const DEBUG = true;
function log(...args: unknown[]) {
  if (DEBUG) {
    console.log('[WindowComm]', ...args);
  }
}

export class WindowCommunicationBridge {
  private static instance: WindowCommunicationBridge;

  private constructor() {
    // Listen to connection state changes
    connectionManager.subscribe({
      onStateChange: (state, info) => {
        log('Connection state changed:', state, info);
      },
      onError: (error, info) => {
        console.error('[WindowComm] Connection error:', error, info);
      },
      onMessageSent: (payload) => {
        log('Message sent:', payload.new_state);
      },
      onMessageReceived: (data) => {
        log('Message received:', data);
      },
    });
  }

  static getInstance(): WindowCommunicationBridge {
    if (!WindowCommunicationBridge.instance) {
      WindowCommunicationBridge.instance = new WindowCommunicationBridge();
    }
    return WindowCommunicationBridge.instance;
  }

  public sendWindowStateChange(payload: WindowStateChangePayload): void {
    log('Sending state change:', payload.new_state);
    connectionManager.send(payload, true);
  }

  public isBackendConnected(): boolean {
    return connectionManager.isConnected();
  }

  public getPendingCount(): number {
    const info = connectionManager.getStateInfo();
    return info.pendingMessages;
  }

  public async testConnection(): Promise<boolean> {
    try {
      const webui = (window as any).webui;
      if (!webui) return false;
      await webui.call('ping_backend');
      return true;
    } catch (error) {
      return false;
    }
  }

  public forceReconnect(): void {
    connectionManager.forceReconnect();
  }
}

// Export singleton
export const windowCommBridge = WindowCommunicationBridge.getInstance();

// Helper function to send window state
export function sendWindowState(payload: WindowStateChangePayload): void {
  windowCommBridge.sendWindowStateChange(payload);
}

// Test function
export async function testBackendConnection(): Promise<boolean> {
  return windowCommBridge.testConnection();
}
