import type { BatteryState, BluetoothState, BrightnessState, MediaState, NetworkState, StoredNotification, SystemState, UsbDriveInfo, VolumeState } from '$lib/types';
import { tauriBridge } from '$lib/services/tauriBridge';
import { islandState } from './islandState.svelte';

class SystemStore {
  media = $state<MediaState | null>(null);
  battery = $state<BatteryState | null>(null);
  network = $state<NetworkState>({
    connected: false,
    network_type: 'disconnected',
    ssid: null,
    signal_percent: 0,
    ethernet_connected: false,
    wifi_enabled: true,
  });
  bluetooth = $state<BluetoothState>({ enabled: false, connected_device: null, devices: [] });
  airplaneMode = $state<boolean>(false);
  usbDrives = $state<UsbDriveInfo[]>([]);
  notifications = $state<StoredNotification[]>([]);

  volume = $state<VolumeState>({ volume: 50, muted: false });
  mic = $state<VolumeState>({ volume: 50, muted: false });
  brightness = $state<BrightnessState>({ brightness: 50 });
  system = $state<SystemState>({
    cpu_percent: 0,
    ram_used_mb: 0,
    ram_total_mb: 0,
    ram_percent: 0,
    uptime_secs: 0,
  });

  currentTime = $state('');
  currentDate = $state('');

  private pollInterval: number | null = null;
  private timeInterval: number | null = null;
  private unlistenNotification: (() => void) | null = null;

  init() {
    this.updateClock();
    this.timeInterval = window.setInterval(() => this.updateClock(), 1000);

    // Initial fetch
    this.refreshAll();

    // Start in low-power idle polling (every 5s) to keep CPU at ~0.0% on laptop
    this.pollInterval = window.setInterval(() => this.refreshAll(), 5000);

    // Listen for incoming system notifications
    tauriBridge.listenNotification((payload) => {
      islandState.triggerNotification(payload);

      const now = new Date();
      const timeStr = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false });
      const stored: StoredNotification = {
        ...payload,
        time: timeStr,
        timestamp: Date.now(),
      };
      // Prepend to notifications list, max 20 items
      this.notifications = [stored, ...this.notifications.filter(n => n.id !== payload.id)].slice(0, 20);
    }).then((unlisten) => {
      this.unlistenNotification = unlisten;
    });
  }

  setPollingRate(ms: number) {
    if (this.pollInterval) clearInterval(this.pollInterval);
    this.pollInterval = window.setInterval(() => this.refreshAll(), ms);
  }

  destroy() {
    if (this.pollInterval) clearInterval(this.pollInterval);
    if (this.timeInterval) clearInterval(this.timeInterval);
    if (this.unlistenNotification) this.unlistenNotification();
  }

  private updateClock() {
    const now = new Date();
    this.currentTime = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false });
    this.currentDate = now.toLocaleDateString([], { weekday: 'short', month: 'short', day: 'numeric' });
  }

  async refreshAll() {
    const [media, battery, network, volume, mic, brightness, system, bluetooth, airplaneMode, usbDrives] = await Promise.all([
      tauriBridge.fetchMediaState(),
      tauriBridge.fetchBatteryState(),
      tauriBridge.fetchNetworkState(),
      tauriBridge.fetchVolumeState(),
      tauriBridge.fetchMicState(),
      tauriBridge.fetchBrightnessState(),
      tauriBridge.fetchSystemState(),
      tauriBridge.fetchBluetoothState(),
      tauriBridge.fetchAirplaneMode(),
      tauriBridge.fetchUsbDrives(),
    ]);

    this.media = media;
    this.battery = battery;
    this.network = network;
    this.volume = volume;
    this.mic = mic;
    this.brightness = brightness;
    this.system = system;
    this.bluetooth = bluetooth;
    this.airplaneMode = airplaneMode;
    this.usbDrives = usbDrives;
  }

  async fetchSystemData() {
    await this.refreshAll();
  }

  // User Actions
  async togglePlayPause() {
    await tauriBridge.mediaPlayPause();
    setTimeout(() => this.refreshMedia(), 150);
  }

  async nextTrack() {
    await tauriBridge.mediaNext();
    setTimeout(() => this.refreshMedia(), 250);
  }

  async prevTrack() {
    await tauriBridge.mediaPrevious();
    setTimeout(() => this.refreshMedia(), 250);
  }

  async seek(seconds: number) {
    await tauriBridge.mediaSeek(seconds);
    setTimeout(() => this.refreshMedia(), 100);
  }

  async setVol(val: number) {
    this.volume.volume = val;
    await tauriBridge.setVolume(val);
    islandState.showHud('volume', val);
  }

  async toggleMute() {
    this.volume.muted = !this.volume.muted;
    await tauriBridge.toggleMute();
    islandState.showHud('volume', this.volume.volume);
  }

  async setMicVol(val: number) {
    this.mic.volume = val;
    await tauriBridge.setMicVolume(val);
  }

  async toggleMicMute() {
    this.mic.muted = !this.mic.muted;
    await tauriBridge.toggleMicMute();
  }

  async setBright(val: number) {
    this.brightness.brightness = val;
    await tauriBridge.setBrightness(val);
    islandState.showHud('brightness', val);
  }

  async toggleBluetooth() {
    const next = !this.bluetooth.enabled;
    this.bluetooth.enabled = next;
    await tauriBridge.toggleBluetooth(next);
    setTimeout(() => this.refreshBluetooth(), 400);
  }

  async refreshBluetooth() {
    this.bluetooth = await tauriBridge.fetchBluetoothState();
  }

  async connectBluetooth(mac: string) {
    await tauriBridge.connectBluetooth(mac);
    await this.refreshBluetooth();
  }

  async disconnectBluetooth(mac: string) {
    await tauriBridge.disconnectBluetooth(mac);
    await this.refreshBluetooth();
  }

  dismissNotification(id: number) {
    this.notifications = this.notifications.filter(n => n.id !== id);
  }

  clearAllNotifications() {
    this.notifications = [];
  }

  async toggleAirplaneMode() {
    const next = !this.airplaneMode;
    this.airplaneMode = next;
    await tauriBridge.toggleAirplaneMode(next);
    setTimeout(() => this.refreshAll(), 400);
  }

  async openFileManager(path?: string) {
    await tauriBridge.openFileManager(path);
  }

  private async refreshMedia() {
    this.media = await tauriBridge.fetchMediaState();
  }
}

export const systemStore = new SystemStore();

