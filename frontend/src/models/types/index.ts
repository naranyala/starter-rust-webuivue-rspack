export interface User {
  id: number;
  name: string;
  email: string;
  role: string;
  status: string;
  department?: string;
  lastLogin?: string;
}

export interface Product {
  id: number;
  name: string;
  description: string;
  price: number;
  category: string;
  stock: number;
  sku?: string;
  weight?: number;
}

export interface SystemInfo {
  platform?: string;
  arch?: string;
  family?: string;
  os_detail?: string;
  hostname?: string;
  kernel_version?: string;
  boot_time?: number;
}

export interface ApiResponse<T = unknown> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface EventData {
  event: string;
  data: unknown;
  timestamp: string;
  source: 'frontend' | 'backend';
  id?: string;
}

export interface WindowState {
  id: number;
  title: string;
  icon: string;
  component: string;
  minimized: boolean;
  maximized: boolean;
}

export interface CardItem {
  id: string;
  title: string;
  description: string;
  icon: string;
  component: string;
}

export interface WindowTrackingEvent {
  windowId: number;
  windowTitle: string;
  component: string;
  eventType: 'opened' | 'focused' | 'minimized' | 'restored' | 'maximized' | 'closed';
  timestamp: string;
}

export interface WindowStateChangePayload {
  window_id: number;
  window_title: string;
  component: string;
  previous_state: string | null;
  new_state: string;
  timestamp: string;
}
