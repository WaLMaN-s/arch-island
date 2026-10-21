export interface MediaState {
  player: string;
  status: 'Playing' | 'Paused' | 'Stopped' | string;
  title: string;
  artist: string;
  album: string;
  art_url: string;
  position_sec: number;
  duration_sec: number;
}

export interface BatteryState {
  percentage: number;
  state: string;
  is_charging: boolean;
  time_remaining: string | null;
  power_watts: number;
  warning_level: 'none' | 'low' | 'critical' | string;
}

export interface NetworkState {
  connected: boolean;
  network_type: 'wifi' | 'ethernet' | 'disconnected' | string;
  ssid: string | null;
  signal_percent: number;
  ethernet_connected: boolean;
  wifi_enabled: boolean;
}

export interface WifiNetworkInfo {
  ssid: string;
  signal_percent: number;
  security: string;
  in_use: boolean;
}

export interface VolumeState {
  volume: number;
  muted: boolean;
}

export interface BrightnessState {
  brightness: number;
}

export interface SystemState {
  cpu_percent: number;
  ram_used_mb: number;
  ram_total_mb: number;
  ram_percent: number;
  uptime_secs: number;
}

export interface BluetoothDevice {
  mac: string;
  name: string;
  connected: boolean;
  paired: boolean;
  icon?: string | null;
  battery_percent?: number | null;
}

export interface BluetoothState {
  enabled: boolean;
  connected_device: string | null;
  connected_mac?: string | null;
  battery_percent?: number | null;
  devices: BluetoothDevice[];
}

export interface UsbDriveInfo {
  name: string;
  size: string;
  label: string | null;
  model: string | null;
  mountpoint: string | null;
}

export interface NotificationPayload {
  id: number;
  app_name: string;
  summary: string;
  body: string;
  icon: string;
}

export interface StoredNotification extends NotificationPayload {
  time: string;
  timestamp: number;
}

export interface HardwareEventPayload {
  kind: 'charger' | 'usb' | 'audio' | string;
  status: 'connected' | 'disconnected' | string;
  title: string;
  subtitle: string;
  icon: string;
  percentage?: number;
}

export type IslandMode = 'tucked' | 'compact' | 'expanded';
export type IslandSubView = 'auto' | 'music' | 'controls' | 'battery' | 'notification' | 'hud-volume' | 'hud-brightness';
