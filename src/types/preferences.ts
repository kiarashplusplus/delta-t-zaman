export interface WindowState {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  maximized?: boolean;
}

export interface UserPreferences {
  time_format: '12h' | '24h';
  locale: string;
  theme: 'light' | 'dark' | 'system';
  high_contrast: 'on' | 'off' | 'system';
  default_timezone: string;
  always_on_top: boolean;
  autostart: boolean;
  telemetry_opt_in?: boolean;
  auto_update: boolean;
  window_state?: WindowState;
  compact_mode: boolean;
  compact_window_state?: WindowState;
}

export interface SystemInfo {
  platform: string;
  arch: string;
  locale: string;
  timezone: string; // IANA
  app_version: string;
  is_mobile: boolean;
  supports_tray: boolean;
  os_version: string;
}

export interface PlatformCloseBehavior {
  platform: 'macos' | 'windows' | 'linux' | 'ios' | 'android';
  close_action: 'hide_to_tray' | 'quit' | 'suspend';
  tray_available: boolean;
  quit_shortcut: string | null;
}

// IPC request/response types can be defined here as well
export interface IPCResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}
