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

export interface WindowState {
  id: number;
  title: string;
  component: string;
  minimized: boolean;
  maximized: boolean;
}
