import type { WindowStateChangePayload } from '../types';
import { windowCommBridge } from './windowCommBridge.ts';

const DEBUG = true;
function log(...args: unknown[]) {
  if (DEBUG) {
    console.log('[WindowTracker]', ...args);
  }
}

export class WindowTracker {
  private static instance: WindowTracker;
  private windowStates: Map<number, string> = new Map();
  private isEnabled: boolean = true;

  private constructor() {
    log('WindowTracker initialized');
  }

  static getInstance(): WindowTracker {
    if (!WindowTracker.instance) {
      WindowTracker.instance = new WindowTracker();
    }
    return WindowTracker.instance;
  }

  enable(): void {
    this.isEnabled = true;
    log('Window tracking enabled');
  }

  disable(): void {
    this.isEnabled = false;
    log('Window tracking disabled');
  }

  private trackStateChange(
    windowId: number,
    windowTitle: string,
    component: string,
    newState: string
  ): void {
    if (!this.isEnabled) return;

    const timestamp = new Date().toISOString();
    const previousState = this.windowStates.get(windowId) || null;

    this.windowStates.set(windowId, newState);

    const payload: WindowStateChangePayload = {
      window_id: windowId,
      window_title: windowTitle,
      component: component,
      previous_state: previousState,
      new_state: newState,
      timestamp: timestamp,
    };

    // Send to backend via communication bridge
    windowCommBridge.sendWindowStateChange(payload);

    // Also log locally
    log(`[${newState.toUpperCase()}] ${windowTitle} (${component})`);
  }

  trackWindowOpened(windowId: number, windowTitle: string, component: string): void {
    this.trackStateChange(windowId, windowTitle, component, 'opened');
  }

  trackWindowFocused(windowId: number, windowTitle: string, component: string): void {
    this.trackStateChange(windowId, windowTitle, component, 'focused');
  }

  trackWindowMinimized(windowId: number, windowTitle: string, component: string): void {
    this.trackStateChange(windowId, windowTitle, component, 'minimized');
  }

  trackWindowRestored(windowId: number, windowTitle: string, component: string): void {
    this.trackStateChange(windowId, windowTitle, component, 'restored');
  }

  trackWindowMaximized(windowId: number, windowTitle: string, component: string): void {
    this.trackStateChange(windowId, windowTitle, component, 'maximized');
  }

  trackWindowClosed(windowId: number, windowTitle: string, component: string): void {
    this.trackStateChange(windowId, windowTitle, component, 'closed');
    this.windowStates.delete(windowId);
  }

  getWindowState(windowId: number): string | undefined {
    return this.windowStates.get(windowId);
  }

  getAllWindowStates(): Map<number, string> {
    return new Map(this.windowStates);
  }

  isBackendConnected(): boolean {
    return windowCommBridge.isBackendConnected();
  }

  getPendingCount(): number {
    return windowCommBridge.getPendingCount();
  }
}

export const windowTracker = WindowTracker.getInstance();

export function useWindowTracker() {
  return {
    trackOpened: windowTracker.trackWindowOpened.bind(windowTracker),
    trackFocused: windowTracker.trackWindowFocused.bind(windowTracker),
    trackMinimized: windowTracker.trackWindowMinimized.bind(windowTracker),
    trackRestored: windowTracker.trackWindowRestored.bind(windowTracker),
    trackMaximized: windowTracker.trackWindowMaximized.bind(windowTracker),
    trackClosed: windowTracker.trackWindowClosed.bind(windowTracker),
    enable: windowTracker.enable.bind(windowTracker),
    disable: windowTracker.disable.bind(windowTracker),
    getWindowState: windowTracker.getWindowState.bind(windowTracker),
    getAllWindowStates: windowTracker.getAllWindowStates.bind(windowTracker),
    isBackendConnected: windowTracker.isBackendConnected.bind(windowTracker),
    getPendingCount: windowTracker.getPendingCount.bind(windowTracker),
  };
}
