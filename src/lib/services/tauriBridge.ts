import { invoke, isTauri } from '@tauri-apps/api/core';

import { listen } from '@tauri-apps/api/event';
import type {
  BatteryState,
  BluetoothState,
  BrightnessState,
  HardwareEventPayload,
  MediaState,
  NetworkState,
  NotificationPayload,
  SystemState,
  UsbDriveInfo,
  VolumeState,
  WifiNetworkInfo,
} from '$lib/types';


export const tauriBridge = {
  isAvailable(): boolean {
    return isTauri();
  },

  async resizeIsland(mode: 'tucked' | 'compact' | 'expanded', customWidth?: number, customHeight?: number): Promise<void> {
    if (!isTauri()) return;
    try {
      await invoke('resize_island', {
        mode,
        customWidth: customWidth ?? null,
        customHeight: customHeight ?? null,
      });
    } catch (e) {
      console.warn('resizeIsland error:', e);
    }
  },

  async fetchMediaState(): Promise<MediaState | null> {
    if (!isTauri()) return null;
    try {
      return await invoke<MediaState | null>('get_media_state');
    } catch {
      return null;
    }
  },

  async mediaPlayPause(): Promise<void> {
    if (!isTauri()) return;
    await invoke('media_play_pause');
  },

  async mediaNext(): Promise<void> {
    if (!isTauri()) return;
    await invoke('media_next');
  },

  async mediaPrevious(): Promise<void> {
    if (!isTauri()) return;
    await invoke('media_previous');
  },

  async mediaSeek(offsetSec: number): Promise<void> {
    if (!isTauri()) return;
    await invoke('media_seek', { offsetSec });
  },

  async openSpotifyLyrics(): Promise<void> {
    if (!isTauri()) return;
    await invoke('open_spotify_lyrics');
  },

  async fetchBatteryState(): Promise<BatteryState | null> {
    if (!isTauri()) return null;
    try {
      return await invoke<BatteryState | null>('get_battery_state');
    } catch {
      return null;
    }
  },

  async fetchNetworkState(): Promise<NetworkState> {
    if (!isTauri()) {
      return { connected: true, network_type: 'wifi', ssid: 'Arch-Linux', signal_percent: 85, ethernet_connected: false, wifi_enabled: true };
    }
    try {
      return await invoke<NetworkState>('get_network_state');
    } catch {
      return { connected: false, network_type: 'disconnected', ssid: null, signal_percent: 0, ethernet_connected: false, wifi_enabled: true };
    }
  },

  async toggleWifi(enabled: boolean): Promise<void> {
    if (!isTauri()) return;
    await invoke('toggle_wifi', { enabled });
  },

  async scanWifiNetworks(): Promise<WifiNetworkInfo[]> {
    if (!isTauri()) {
      return [
        { ssid: 'OpungToba', signal_percent: 80, security: 'WPA2', in_use: true },
        { ssid: 'Vania', signal_percent: 55, security: 'WPA2', in_use: false },
        { ssid: 'Cafe_Free_WiFi', signal_percent: 40, security: 'Open', in_use: false },
      ];
    }
    try {
      return await invoke<WifiNetworkInfo[]>('scan_wifi_networks');
    } catch {
      return [];
    }
  },

  async connectWifi(ssid: string, password?: string): Promise<void> {
    if (!isTauri()) return;
    await invoke('connect_wifi', { ssid, password: password || null });
  },

  async disconnectWifi(): Promise<void> {
    if (!isTauri()) return;
    await invoke('disconnect_wifi');
  },

  async dismissSystemNotifications(): Promise<void> {
    if (!isTauri()) return;
    try {
      await invoke('dismiss_system_notifications');
    } catch {}
  },

  async fetchVolumeState(): Promise<VolumeState> {
    if (!isTauri()) return { volume: 50, muted: false };
    try {
      return await invoke<VolumeState>('get_volume_state');
    } catch {
      return { volume: 50, muted: false };
    }
  },

  async setVolume(percent: number): Promise<void> {
    if (!isTauri()) return;
    await invoke('set_volume', { percent: Math.round(percent) });
  },

  async changeVolume(delta: number): Promise<void> {
    if (!isTauri()) return;
    await invoke('change_volume', { delta });
  },

  async toggleMute(): Promise<void> {
    if (!isTauri()) return;
    await invoke('toggle_mute');
  },

  async fetchMicState(): Promise<VolumeState> {
    if (!isTauri()) return { volume: 50, muted: false };
    try {
      return await invoke<VolumeState>('get_mic_state');
    } catch {
      return { volume: 50, muted: false };
    }
  },

  async setMicVolume(percent: number): Promise<void> {
    if (!isTauri()) return;
    await invoke('set_mic_volume', { percent: Math.round(percent) });
  },

  async toggleMicMute(): Promise<void> {
    if (!isTauri()) return;
    await invoke('toggle_mic_mute');
  },

  async fetchBrightnessState(): Promise<BrightnessState> {
    if (!isTauri()) return { brightness: 50 };
    try {
      return await invoke<BrightnessState>('get_brightness_state');
    } catch {
      return { brightness: 50 };
    }
  },

  async setBrightness(percent: number): Promise<void> {
    if (!isTauri()) return;
    await invoke('set_brightness', { percent: Math.round(percent) });
  },

  async changeBrightness(delta: number): Promise<void> {
    if (!isTauri()) return;
    await invoke('change_brightness', { delta });
  },

  async fetchSystemState(): Promise<SystemState> {
    if (!isTauri()) return { cpu_percent: 2, ram_used_mb: 1840, ram_total_mb: 8192, ram_percent: 22, uptime_secs: 3600 };
    try {
      return await invoke<SystemState>('get_system_state');
    } catch {
      return { cpu_percent: 0, ram_used_mb: 0, ram_total_mb: 0, ram_percent: 0, uptime_secs: 0 };
    }
  },

  async listenNotification(callback: (payload: NotificationPayload) => void): Promise<() => void> {
    if (!isTauri()) return () => {};
    try {
      const unlisten = await listen<NotificationPayload>('notification_received', (event) => {
        callback(event.payload);
      });
      return unlisten;
    } catch {
      return () => {};
    }
  },

  async fetchBluetoothState(): Promise<BluetoothState> {
    if (!isTauri()) return { enabled: true, connected_device: null, devices: [] };
    try {
      return await invoke<BluetoothState>('get_bluetooth_state');
    } catch {
      return { enabled: false, connected_device: null, devices: [] };
    }
  },

  async toggleBluetooth(enabled: boolean): Promise<void> {
    if (!isTauri()) return;
    await invoke('toggle_bluetooth', { enabled });
  },

  async connectBluetooth(mac: string): Promise<void> {
    if (!isTauri()) return;
    await invoke('connect_bluetooth', { mac });
  },

  async disconnectBluetooth(mac: string): Promise<void> {
    if (!isTauri()) return;
    await invoke('disconnect_bluetooth', { mac });
  },

  async fetchAirplaneMode(): Promise<boolean> {
    if (!isTauri()) return false;
    try {
      return await invoke<boolean>('get_airplane_mode');
    } catch {
      return false;
    }
  },

  async toggleAirplaneMode(enabled: boolean): Promise<void> {
    if (!isTauri()) return;
    await invoke('toggle_airplane_mode', { enabled });
  },

  async fetchUsbDrives(): Promise<UsbDriveInfo[]> {
    if (!isTauri()) return [];
    try {
      return await invoke<UsbDriveInfo[]>('get_usb_drives');
    } catch {
      return [];
    }
  },

  async openFileManager(targetPath?: string): Promise<void> {
    if (!isTauri()) return;
    await invoke('open_file_manager', { targetPath: targetPath || null });
  },

  async fetchLyrics(title: string, artist: string): Promise<{ syncedLyrics?: string; plainLyrics?: string } | null> {
    try {
      // Clean title and artist of extra tags like (Remastered), - Single, etc.
      const cleanTitle = title.replace(/\([^)]*\)/g, '').replace(/\[[^\]]*\]/g, '').trim();
      const cleanArtist = artist.split(',')[0].trim();
      const url = `https://lrclib.net/api/get?track_name=${encodeURIComponent(cleanTitle)}&artist_name=${encodeURIComponent(cleanArtist)}`;
      
      const res = await fetch(url);
      if (!res.ok) return null;
      const data = await res.json();
      return {
        syncedLyrics: data.syncedLyrics || undefined,
        plainLyrics: data.plainLyrics || undefined,
      };
    } catch {
      return null;
    }
  },

  async listenHardwareEvent(callback: (payload: HardwareEventPayload) => void): Promise<() => void> {
    if (!isTauri()) return () => {};
    try {
      const unlisten = await listen<HardwareEventPayload>('hardware_event', (event) => {
        callback(event.payload);
      });
      return unlisten;
    } catch {
      return () => {};
    }
  },
};

