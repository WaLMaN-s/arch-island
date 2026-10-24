<script lang="ts">
  import { onDestroy } from 'svelte';
  import { systemStore } from '$lib/stores/systemStore.svelte';
  import { islandState } from '$lib/stores/islandState.svelte';
  import { tauriBridge } from '$lib/services/tauriBridge';
  import type { WifiNetworkInfo } from '$lib/types';
  import {
    Activity,
    Bell,
    Check,
    Cpu,
    HardDrive,
    Headphones,
    Lock,
    Mic,
    MicOff,
    RefreshCw,
    Sun,
    Volume2,
    VolumeX,
    Wifi,
    WifiOff,
    Zap,
    BatteryMedium,
    Bluetooth,
    BluetoothOff,
    Plane,
    Usb,
    FolderOpen,
    X,
  } from 'lucide-svelte';
  import ArchLogo from './ArchLogo.svelte';

  let showWifiModal = $state(false);
  let wifiNetworks = $state<WifiNetworkInfo[]>([]);
  let isScanning = $state(false);
  let selectedSsid = $state<string | null>(null);
  let wifiPassword = $state('');
  let isConnecting = $state(false);
  let wifiError = $state<string | null>(null);

  // Bluetooth Modal State
  let showBluetoothModal = $state(false);
  let isConnectingBt = $state(false);
  let btError = $state<string | null>(null);

  $effect(() => {
    islandState.hasModalOpen = showWifiModal || showBluetoothModal;
  });

  onDestroy(() => {
    islandState.hasModalOpen = false;
  });

  async function openBluetoothModal() {
    showBluetoothModal = true;
    btError = null;
    await systemStore.refreshBluetooth();
  }

  async function connectBtDevice(mac: string) {
    isConnectingBt = true;
    btError = null;
    try {
      await systemStore.connectBluetooth(mac);
    } catch (e: any) {
      btError = e?.toString() || 'Gagal menyambungkan Bluetooth';
    } finally {
      isConnectingBt = false;
    }
  }

  async function disconnectBtDevice(mac: string) {
    isConnectingBt = true;
    btError = null;
    try {
      await systemStore.disconnectBluetooth(mac);
    } catch (e: any) {
      btError = e?.toString() || 'Gagal memutuskan Bluetooth';
    } finally {
      isConnectingBt = false;
    }
  }

  function handleVolumeChange(e: Event) {
    const input = e.target as HTMLInputElement;
    systemStore.setVol(Number(input.value));
  }

  function handleMicChange(e: Event) {
    const input = e.target as HTMLInputElement;
    systemStore.setMicVol(Number(input.value));
  }

  function handleBrightnessChange(e: Event) {
    const input = e.target as HTMLInputElement;
    systemStore.setBright(Number(input.value));
  }

  async function toggleWifi(e: MouseEvent) {
    e.stopPropagation();
    const targetState = !systemStore.network.wifi_enabled;
    await tauriBridge.toggleWifi(targetState);
    await systemStore.fetchSystemData();
  }

  async function openWifiModal() {
    showWifiModal = true;
    await scanNetworks();
  }

  async function scanNetworks() {
    isScanning = true;
    wifiError = null;
    try {
      wifiNetworks = await tauriBridge.scanWifiNetworks();
    } catch (e: any) {
      wifiError = e?.toString() || 'Scan failed';
    } finally {
      isScanning = false;
    }
  }

  function handleSelectNetwork(net: WifiNetworkInfo) {
    selectedSsid = net.ssid;
    wifiPassword = '';
    wifiError = null;
  }

  async function connectSelected() {
    if (!selectedSsid) return;
    isConnecting = true;
    wifiError = null;
    try {
      await tauriBridge.connectWifi(selectedSsid, wifiPassword || undefined);
      await systemStore.fetchSystemData();
      await scanNetworks();
      selectedSsid = null;
      wifiPassword = '';
    } catch (e: any) {
      wifiError = e?.toString() || 'Connection failed';
    } finally {
      isConnecting = false;
    }
  }

  async function disconnectCurrent() {
    isConnecting = true;
    try {
      await tauriBridge.disconnectWifi();
      await systemStore.fetchSystemData();
      await scanNetworks();
    } finally {
      isConnecting = false;
    }
  }
</script>

<div
  class="control-grid"
  onwheel={(e) => {
    e.currentTarget.scrollTop += e.deltaY;
  }}
>
  <!-- Top Row: Clock & Quick Stats -->
  <div class="row-clock">
    <div class="time-block">
      <span class="big-time">{systemStore.currentTime}</span>
      <span class="full-date">{systemStore.currentDate}</span>
    </div>

    <div class="system-stats">
      <div class="stat-badge" title="CPU Usage">
        <Cpu size={14} color="#7aa2f7" />
        <span class="stat-name">CPU</span>
        <span class="stat-value">{systemStore.system.cpu_percent}%</span>
      </div>

      <div class="stat-badge" title="RAM Usage">
        <HardDrive size={14} color="#bb9af7" />
        <span class="stat-name">RAM</span>
        <span class="stat-value">{systemStore.system.ram_used_mb} MB ({systemStore.system.ram_percent}%)</span>
      </div>
    </div>
  </div>

  <!-- Middle Row: Network, Bluetooth, Airplane Mode & Battery Cards -->
  <div class="metrics-row">
    <!-- Network & Connectivity Column -->
    <div class="connectivity-col">
      <!-- Network Card with WiFi ON/OFF toggle and Network Selector -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="metric-card interactive" onclick={openWifiModal}>
        <div class="card-header">
          <div class="header-left">
            <div class="icon-circle" class:active={systemStore.network.connected}>
              {#if systemStore.network.connected}
                <ArchLogo size={14} color="#1793d1" />
              {:else}
                <WifiOff size={14} color="#565f89" />
              {/if}
            </div>
            <span class="card-label">{systemStore.network.connected ? 'Arch Network' : 'Wi-Fi'}</span>
          </div>

          <!-- Toggle Switch ON/OFF -->
          <button
            class="switch-btn"
            class:checked={systemStore.network.wifi_enabled}
            onclick={toggleWifi}
            title={systemStore.network.wifi_enabled ? 'Turn Wi-Fi Off' : 'Turn Wi-Fi On'}
          >
            <div class="switch-knob"></div>
          </button>
        </div>

        <div class="card-content">
          <span class="primary-text" title={systemStore.network.ssid || 'Not connected'}>
            {systemStore.network.ssid || (systemStore.network.connected ? 'Ethernet' : 'Disconnected')}
          </span>
          <span class="secondary-text">
            {systemStore.network.wifi_enabled
              ? (systemStore.network.connected ? `${systemStore.network.signal_percent}% Signal • Ubah` : 'Aktif • Klik untuk sambung')
              : 'Wi-Fi dimatikan'}
          </span>
        </div>
      </div>

      <!-- Quick Toggles: Bluetooth & Mode Pesawat -->
      <div class="quick-toggles-row">
        <!-- Bluetooth Toggle Card with battery percentage & device name -->
        <button
          class="quick-toggle-btn"
          class:active={systemStore.bluetooth.enabled}
          onclick={(e) => { e.stopPropagation(); openBluetoothModal(); }}
          title={systemStore.bluetooth.enabled ? 'Kelola Perangkat Bluetooth' : 'Nyalakan Bluetooth'}
        >
          <div class="toggle-icon-wrap" class:active={systemStore.bluetooth.enabled}>
            {#if systemStore.bluetooth.connected_device}
              <Headphones size={13} color="#7aa2f7" />
            {:else if systemStore.bluetooth.enabled}
              <Bluetooth size={13} color="#7aa2f7" />
            {:else}
              <BluetoothOff size={13} color="#565f89" />
            {/if}
          </div>
          <div class="toggle-text-wrap">
            <div class="toggle-title-row">
              <span class="toggle-title">Bluetooth</span>
              {#if systemStore.bluetooth.battery_percent !== null && systemStore.bluetooth.battery_percent !== undefined}
                <span class="bt-bat-badge">{systemStore.bluetooth.battery_percent}%</span>
              {/if}
            </div>
            <span class="toggle-sub">{systemStore.bluetooth.connected_device || (systemStore.bluetooth.enabled ? 'Aktif' : 'Mati')}</span>
          </div>
        </button>

        <!-- Mode Pesawat Toggle -->
        <button
          class="quick-toggle-btn"
          class:active={systemStore.airplaneMode}
          onclick={(e) => { e.stopPropagation(); systemStore.toggleAirplaneMode(); }}
          title={systemStore.airplaneMode ? 'Matikan Mode Pesawat' : 'Nyalakan Mode Pesawat'}
        >
          <div class="toggle-icon-wrap" class:active={systemStore.airplaneMode}>
            <Plane size={13} color={systemStore.airplaneMode ? '#f7768e' : '#565f89'} />
          </div>
          <div class="toggle-text-wrap">
            <span class="toggle-title">Pesawat</span>
            <span class="toggle-sub">{systemStore.airplaneMode ? 'Aktif' : 'Mati'}</span>
          </div>
        </button>
      </div>
    </div>

    <!-- Battery & USB Devices Column -->
    <div class="battery-col">
      <!-- Battery Card -->
      <div class="metric-card">
        <div class="card-header">
          <div class="header-left">
            <div class="icon-circle" class:active={systemStore.battery?.is_charging}>
              {#if systemStore.battery?.is_charging}
                <Zap size={14} color="#10b981" />
              {:else}
                <BatteryMedium size={14} color="#9ece6a" />
              {/if}
            </div>
            <span class="card-label">Battery</span>
          </div>
          <span class="status-badge" class:charging={systemStore.battery?.is_charging}>
            {systemStore.battery?.state || 'Normal'}
          </span>
        </div>

        <div class="card-content">
          <div class="battery-metric-row">
            <span class="primary-text">{systemStore.battery?.percentage || 0}%</span>
            {#if systemStore.battery?.power_watts}
              <span class="watt-text">{systemStore.battery.power_watts.toFixed(1)}W</span>
            {/if}
          </div>
          <span class="secondary-text">
            {systemStore.battery?.time_remaining ? `~${systemStore.battery.time_remaining}` : (systemStore.battery?.is_charging ? 'Charging' : 'On Battery')}
          </span>
        </div>
      </div>

      <!-- USB Drives Section (Shows when flashdisk is connected) -->
      {#if systemStore.usbDrives.length > 0}
        <div class="usb-drives-list">
          {#each systemStore.usbDrives as drive}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
              class="usb-card"
              onclick={() => systemStore.openFileManager(drive.mountpoint || undefined)}
              title="Buka {drive.label || drive.name} di File Manager (Thunar)"
            >
              <div class="usb-left">
                <Usb size={13} color="#7aa2f7" />
                <div class="usb-meta">
                  <span class="usb-name">{drive.label || drive.name}</span>
                  <span class="usb-size">{drive.size}</span>
                </div>
              </div>
              <div class="usb-badge">
                <FolderOpen size={11} color="#c0caf5" />
                <span>Buka</span>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  <!-- Bottom Row: Sliders (Voice Audio Mic, Speaker Volume, & Brightness) -->
  <div class="sliders-row">
    <!-- Voice Audio / Mic Slider (Diatas Suara) -->
    <div class="slider-container">
      <button class="slider-icon-btn" onclick={() => systemStore.toggleMicMute()} title={systemStore.mic.muted ? 'Unmute Mic' : 'Mute Mic'}>
        {#if systemStore.mic.muted}
          <MicOff size={16} color="#f7768e" />
        {:else}
          <Mic size={16} color="#bb9af7" />
        {/if}
      </button>

      <div class="range-wrapper">
        <input
          type="range"
          min="0"
          max="100"
          value={systemStore.mic.volume}
          oninput={handleMicChange}
          class="slider-input mic"
        />
        <div class="slider-fill mic" style="width: {systemStore.mic.volume}%;"></div>
      </div>

      <span class="slider-val">{systemStore.mic.muted ? 'MUTED' : `${systemStore.mic.volume}%`}</span>
    </div>

    <!-- Volume Slider -->
    <div class="slider-container">
      <button class="slider-icon-btn" onclick={() => systemStore.toggleMute()} title={systemStore.volume.muted ? 'Unmute' : 'Mute'}>
        {#if systemStore.volume.muted}
          <VolumeX size={16} color="#f7768e" />
        {:else}
          <Volume2 size={16} color="#7aa2f7" />
        {/if}
      </button>

      <div class="range-wrapper">
        <input
          type="range"
          min="0"
          max="100"
          value={systemStore.volume.volume}
          oninput={handleVolumeChange}
          class="slider-input volume"
        />
        <div class="slider-fill volume" style="width: {systemStore.volume.volume}%;"></div>
      </div>

      <span class="slider-val">{systemStore.volume.muted ? 'MUTED' : `${systemStore.volume.volume}%`}</span>
    </div>

    <!-- Brightness Slider -->
    <div class="slider-container">
      <div class="slider-icon-btn static">
        <Sun size={16} color="#e0af68" />
      </div>

      <div class="range-wrapper">
        <input
          type="range"
          min="1"
          max="100"
          value={systemStore.brightness.brightness}
          oninput={handleBrightnessChange}
          class="slider-input brightness"
        />
        <div class="slider-fill brightness" style="width: {systemStore.brightness.brightness}%;"></div>
      </div>

      <span class="slider-val">{systemStore.brightness.brightness}%</span>
    </div>
  </div>

  <!-- Notifications Section (Smartphone style notification drawer below brightness) -->
  <div class="notifications-section">
    <div class="notif-section-header">
      <div class="notif-header-left">
        <Bell size={12} color="#7aa2f7" />
        <span class="notif-section-title">Pemberitahuan</span>
        {#if systemStore.notifications.length > 0}
          <span class="notif-count-pill">{systemStore.notifications.length}</span>
        {/if}
      </div>

      {#if systemStore.notifications.length > 0}
        <button class="clear-all-btn" onclick={() => systemStore.clearAllNotifications()} title="Hapus Semua Pemberitahuan">
          <span>Hapus Semua</span>
        </button>
      {/if}
    </div>

    {#if systemStore.notifications.length === 0}
      <div class="empty-notif-box">
        <span class="empty-notif-text">Tidak ada pemberitahuan baru</span>
      </div>
    {:else}
      <div class="notif-scroll-list">
        {#each systemStore.notifications as notif (notif.id)}
          <div
            class="notif-card"
            class:whatsapp={notif.app_name.toLowerCase().includes('whatsapp') || notif.summary.toLowerCase().includes('whatsapp')}
            class:firefox={notif.app_name.toLowerCase().includes('firefox')}
          >
            <div class="notif-card-header">
              <div class="notif-card-app">
                <span class="app-dot"></span>
                <span class="notif-app">{notif.app_name}</span>
                <span class="notif-time">{notif.time}</span>
              </div>
              <button class="notif-dismiss-btn" onclick={() => systemStore.dismissNotification(notif.id)} title="Hapus">
                <X size={11} />
              </button>
            </div>

            <div class="notif-card-content">
              <span class="notif-summary">{notif.summary}</span>
              {#if notif.body}
                <span class="notif-body">{notif.body}</span>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Wi-Fi Network Selector Modal / Drawer -->
  {#if showWifiModal}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="wifi-overlay" onclick={() => (showWifiModal = false)}>
      <div class="wifi-dialog" onclick={(e) => e.stopPropagation()}>
        <div class="dialog-header">
          <div class="header-title">
            <Wifi size={16} color="#7aa2f7" />
            <span>Wi-Fi Networks</span>
          </div>

          <div class="dialog-actions">
            <button class="icon-action-btn" onclick={scanNetworks} disabled={isScanning} title="Scan for networks">
              <RefreshCw size={14} class={isScanning ? 'spin' : ''} />
            </button>
            <button class="icon-action-btn" onclick={() => (showWifiModal = false)} title="Close">
              <X size={15} />
            </button>
          </div>
        </div>

        {#if wifiError}
          <div class="error-banner">{wifiError}</div>
        {/if}

        <div class="network-list">
          {#if isScanning && wifiNetworks.length === 0}
            <div class="empty-state">Scanning available networks...</div>
          {:else if wifiNetworks.length === 0}
            <div class="empty-state">No Wi-Fi networks found. Make sure Wi-Fi is turned on.</div>
          {:else}
            {#each wifiNetworks as net}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div
                class="network-item"
                class:selected={selectedSsid === net.ssid}
                class:active={net.in_use}
                onclick={() => handleSelectNetwork(net)}
              >
                <div class="net-left">
                  <Wifi size={14} color={net.in_use ? '#10b981' : '#7aa2f7'} />
                  <span class="net-ssid">{net.ssid}</span>
                  {#if net.security && net.security !== '--'}
                    <Lock size={11} color="#565f89" />
                  {/if}
                </div>

                <div class="net-right">
                  <span class="net-signal">{net.signal_percent}%</span>
                  {#if net.in_use}
                    <span class="connected-badge">Connected</span>
                  {/if}
                </div>
              </div>

              {#if selectedSsid === net.ssid}
                <div class="connect-box">
                  {#if net.in_use}
                    <button class="btn-disconnect" onclick={disconnectCurrent} disabled={isConnecting}>
                      {isConnecting ? 'Disconnecting...' : 'Disconnect'}
                    </button>
                  {:else}
                    {#if net.security && net.security !== '--'}
                      <input
                        type="password"
                        placeholder="Network password"
                        bind:value={wifiPassword}
                        class="pwd-input"
                        onkeydown={(e) => e.key === 'Enter' && connectSelected()}
                      />
                    {/if}
                    <button class="btn-connect" onclick={connectSelected} disabled={isConnecting}>
                      {isConnecting ? 'Connecting...' : 'Connect'}
                    </button>
                  {/if}
                </div>
              {/if}
            {/each}
          {/if}
        </div>
      </div>
    </div>
  {/if}

  <!-- Bluetooth Device Selector Modal / Drawer -->
  {#if showBluetoothModal}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="wifi-overlay" onclick={() => (showBluetoothModal = false)}>
      <div class="wifi-dialog" onclick={(e) => e.stopPropagation()}>
        <div class="dialog-header">
          <div class="header-title">
            <Bluetooth size={16} color="#7aa2f7" />
            <span>Perangkat Bluetooth</span>
          </div>

          <div class="dialog-actions">
            <button
              class="icon-action-btn"
              onclick={() => systemStore.toggleBluetooth()}
              title={systemStore.bluetooth.enabled ? 'Matikan Bluetooth' : 'Nyalakan Bluetooth'}
            >
              {#if systemStore.bluetooth.enabled}
                <Zap size={14} color="#10b981" />
              {:else}
                <BluetoothOff size={14} color="#565f89" />
              {/if}
            </button>
            <button class="icon-action-btn" onclick={() => systemStore.refreshBluetooth()} disabled={isConnectingBt} title="Segarkan">
              <RefreshCw size={14} class={isConnectingBt ? 'spin' : ''} />
            </button>
            <button class="icon-action-btn" onclick={() => (showBluetoothModal = false)} title="Tutup">
              <X size={15} />
            </button>
          </div>
        </div>

        {#if btError}
          <div class="error-banner">{btError}</div>
        {/if}

        <div class="network-list">
          {#if !systemStore.bluetooth.enabled}
            <div class="empty-state">Bluetooth sedang dimatikan. Klik tombol petir untuk menyalakan.</div>
          {:else if systemStore.bluetooth.devices.length === 0}
            <div class="empty-state">Tidak ada perangkat Bluetooth ditemukan atau dipasangkan.</div>
          {:else}
            {#each systemStore.bluetooth.devices as dev}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div class="network-item" class:active={dev.connected}>
                <div class="net-left">
                  {#if dev.icon === 'audio-headset' || dev.icon === 'audio-card'}
                    <Headphones size={15} color={dev.connected ? '#10b981' : '#7aa2f7'} />
                  {:else}
                    <Bluetooth size={15} color={dev.connected ? '#10b981' : '#7aa2f7'} />
                  {/if}
                  <div class="bt-dev-meta">
                    <span class="net-ssid">{dev.name || dev.mac}</span>
                    <span class="bt-mac">{dev.mac}</span>
                  </div>
                </div>

                <div class="net-right">
                  {#if dev.connected}
                    {#if dev.battery_percent !== null && dev.battery_percent !== undefined}
                      <span class="bt-bat-pill">
                        <BatteryMedium size={12} color="#10b981" />
                        <span>{dev.battery_percent}%</span>
                      </span>
                    {/if}
                    <span class="connected-badge">Tersambung</span>
                    <button class="btn-disconnect-sm" onclick={() => disconnectBtDevice(dev.mac)} disabled={isConnectingBt}>
                      Putuskan
                    </button>
                  {:else}
                    <button class="btn-connect-sm" onclick={() => connectBtDevice(dev.mac)} disabled={isConnectingBt}>
                      Sambungkan
                    </button>
                  {/if}
                </div>
              </div>
            {/each}
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .control-grid {
    flex: 1;
    min-height: 0;
    height: 100%;
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 7px;
    width: 100%;
    padding: 0 4px 6px 0;
    overflow-y: auto;
    overflow-x: hidden;
    scroll-behavior: smooth;
    scrollbar-width: thin;
    scrollbar-color: rgba(255, 255, 255, 0.22) transparent;
  }

  .control-grid::-webkit-scrollbar {
    width: 4px;
  }

  .control-grid::-webkit-scrollbar-track {
    background: transparent;
  }

  .control-grid::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.22);
    border-radius: 99px;
  }

  .control-grid::-webkit-scrollbar-thumb:hover {
    background: rgba(122, 162, 247, 0.5);
  }

  .control-grid > *,
  .row-clock,
  .metrics-row,
  .sliders-row,
  .notifications-section {
    flex-shrink: 0;
  }

  /* Clock & Stats */
  .row-clock {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .time-block {
    display: flex;
    align-items: baseline;
    gap: 5px;
  }

  .big-time {
    font-size: 17px;
    font-weight: 700;
    color: #ffffff;
    letter-spacing: -0.02em;
    font-variant-numeric: tabular-nums;
  }

  .full-date {
    font-size: 11px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.6);
  }

  .system-stats {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .stat-badge {
    display: flex;
    align-items: center;
    gap: 4px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.06);
    padding: 2px 8px;
    border-radius: 99px;
  }

  .stat-name {
    font-size: 9.5px;
    font-weight: 700;
    color: #565f89;
  }

  .stat-value {
    font-size: 10px;
    font-weight: 600;
    color: #c0caf5;
    font-variant-numeric: tabular-nums;
  }

  /* Metric Cards */
  .metrics-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px;
  }

  .connectivity-col,
  .battery-col {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  /* Quick Toggles (Bluetooth & Airplane) */
  .quick-toggles-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 5px;
  }

  .quick-toggle-btn {
    background: rgba(255, 255, 255, 0.035);
    border: 1px solid rgba(255, 255, 255, 0.07);
    border-radius: 10px;
    padding: 4px 7px;
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    text-align: left;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .quick-toggle-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(122, 162, 247, 0.25);
  }

  .quick-toggle-btn.active {
    background: rgba(122, 162, 247, 0.12);
    border-color: rgba(122, 162, 247, 0.3);
  }

  .toggle-icon-wrap {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.05);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .toggle-icon-wrap.active {
    background: rgba(122, 162, 247, 0.2);
  }

  .toggle-text-wrap {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .toggle-title {
    font-size: 10px;
    font-weight: 700;
    color: #c0caf5;
    line-height: 1.1;
  }

  .toggle-sub {
    font-size: 8.5px;
    font-weight: 500;
    color: #565f89;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* USB Drives Card */
  .usb-drives-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .usb-card {
    background: rgba(122, 162, 247, 0.07);
    border: 1px solid rgba(122, 162, 247, 0.2);
    border-radius: 10px;
    padding: 6px 8px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .usb-card:hover {
    background: rgba(122, 162, 247, 0.14);
    transform: translateY(-1px);
  }

  .usb-left {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
  }

  .usb-meta {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .usb-name {
    font-size: 10.5px;
    font-weight: 700;
    color: #ffffff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .usb-size {
    font-size: 8.5px;
    color: #7aa2f7;
    font-weight: 600;
  }

  .usb-badge {
    display: flex;
    align-items: center;
    gap: 3px;
    font-size: 9px;
    font-weight: 700;
    color: #c0caf5;
    background: rgba(255, 255, 255, 0.08);
    padding: 2px 6px;
    border-radius: 99px;
  }

  .metric-card {
    background: rgba(255, 255, 255, 0.035);
    border: 1px solid rgba(255, 255, 255, 0.07);
    border-radius: 12px;
    padding: 6px 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    transition: all 0.2s ease;
  }

  .metric-card.interactive {
    cursor: pointer;
  }

  .metric-card.interactive:hover {
    background: rgba(255, 255, 255, 0.07);
    border-color: rgba(122, 162, 247, 0.3);
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .icon-circle {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.05);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-circle.active {
    background: rgba(122, 162, 247, 0.15);
  }

  .card-label {
    font-size: 10px;
    font-weight: 700;
    color: #565f89;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  /* Switch Toggle Button */
  .switch-btn {
    width: 28px;
    height: 15px;
    background: rgba(255, 255, 255, 0.12);
    border: none;
    border-radius: 99px;
    padding: 2px;
    cursor: pointer;
    transition: background 0.2s ease;
    display: flex;
    align-items: center;
  }

  .switch-btn.checked {
    background: #10b981;
  }

  .switch-knob {
    width: 11px;
    height: 11px;
    background: #ffffff;
    border-radius: 50%;
    transition: transform 0.2s ease;
  }

  .switch-btn.checked .switch-knob {
    transform: translateX(13px);
  }

  .status-badge {
    font-size: 9px;
    font-weight: 700;
    padding: 1px 5px;
    border-radius: 99px;
    background: rgba(255, 255, 255, 0.05);
    color: #565f89;
  }

  .status-badge.charging {
    color: #10b981;
    background: rgba(16, 185, 129, 0.12);
  }

  .card-content {
    display: flex;
    flex-direction: column;
  }

  .battery-metric-row {
    display: flex;
    align-items: baseline;
    gap: 5px;
  }

  .watt-text {
    font-size: 10px;
    font-weight: 600;
    color: #10b981;
  }

  .primary-text {
    font-size: 13px;
    font-weight: 600;
    color: #ffffff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .secondary-text {
    font-size: 10px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.55);
    margin-top: 1px;
  }

  /* Sliders */
  .sliders-row {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .slider-container {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(255, 255, 255, 0.035);
    border: 1px solid rgba(255, 255, 255, 0.07);
    border-radius: 10px;
    padding: 3px 8px;
  }


  .slider-icon-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border-radius: 6px;
    transition: background 0.2s ease;
  }

  .slider-icon-btn:hover:not(.static) {
    background: rgba(255, 255, 255, 0.1);
  }

  .slider-icon-btn.static {
    cursor: default;
  }

  .range-wrapper {
    position: relative;
    flex: 1;
    height: 8px;
    background: rgba(255, 255, 255, 0.08);
    border-radius: 99px;
    overflow: hidden;
    display: flex;
    align-items: center;
  }

  .slider-fill {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    background: #7aa2f7;
    border-radius: 99px;
    pointer-events: none;
    transition: width 0.05s linear;
  }

  .slider-fill.brightness {
    background: #e0af68;
  }

  .slider-fill.mic {
    background: #bb9af7;
  }

  .slider-input {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
    margin: 0;
  }

  .slider-val {
    font-size: 12px;
    font-weight: 700;
    color: #c0caf5;
    min-width: 44px;
    text-align: right;
    font-variant-numeric: tabular-nums;
  }

  /* Wi-Fi Dialog / Overlay */
  .wifi-overlay {
    position: absolute;
    inset: -10px;
    background: rgba(10, 12, 18, 0.85);
    backdrop-filter: blur(16px);
    border-radius: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    animation: fadeIn 0.2s ease;
  }

  .wifi-dialog {
    width: 90%;
    max-height: 85%;
    background: #161824;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 20px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.6);
  }

  .dialog-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .header-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 700;
    color: #c0caf5;
  }

  .dialog-actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .icon-action-btn {
    background: rgba(255, 255, 255, 0.06);
    border: none;
    border-radius: 50%;
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #a9b1d6;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .icon-action-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    color: #ffffff;
  }

  .spin {
    animation: spin 1s linear infinite;
  }

  .error-banner {
    padding: 6px 10px;
    background: rgba(247, 118, 142, 0.15);
    border: 1px solid rgba(247, 118, 142, 0.3);
    border-radius: 8px;
    font-size: 11px;
    color: #f7768e;
  }

  .network-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    overflow-y: auto;
    max-height: 220px;
  }

  .network-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 12px;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .network-item:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  .network-item.active {
    background: rgba(16, 185, 129, 0.12);
    border: 1px solid rgba(16, 185, 129, 0.25);
  }

  .net-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .net-ssid {
    font-size: 12px;
    font-weight: 600;
    color: #c0caf5;
  }

  .net-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .net-signal {
    font-size: 11px;
    font-weight: 500;
    color: #565f89;
  }

  .connected-badge {
    font-size: 10px;
    font-weight: 700;
    color: #10b981;
    background: rgba(16, 185, 129, 0.15);
    padding: 2px 6px;
    border-radius: 99px;
  }

  .connect-box {
    display: flex;
    gap: 8px;
    padding: 6px 12px 10px 12px;
    animation: fadeIn 0.2s ease;
  }

  .pwd-input {
    flex: 1;
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    padding: 6px 10px;
    font-size: 11px;
    color: #ffffff;
    outline: none;
  }

  .pwd-input:focus {
    border-color: #7aa2f7;
  }

  .btn-connect {
    background: #7aa2f7;
    color: #0f111a;
    font-weight: 700;
    font-size: 11px;
    border: none;
    border-radius: 8px;
    padding: 6px 14px;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .btn-disconnect {
    background: rgba(247, 118, 142, 0.18);
    color: #f7768e;
    font-weight: 700;
    font-size: 11px;
    border: 1px solid rgba(247, 118, 142, 0.3);
    border-radius: 8px;
    padding: 6px 14px;
    cursor: pointer;
    margin-left: auto;
  }

  .btn-connect:hover, .btn-disconnect:hover {
    opacity: 0.85;
  }

  .empty-state {
    font-size: 11px;
    color: #565f89;
    text-align: center;
    padding: 16px 0;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: scale(0.98); }
    to { opacity: 1; transform: scale(1); }
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  /* Bluetooth Battery Badge in toggle */
  .toggle-title-row {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .bt-bat-badge {
    font-size: 8px;
    font-weight: 700;
    color: #10b981;
    background: rgba(16, 185, 129, 0.15);
    padding: 1px 4px;
    border-radius: 99px;
  }

  /* Bluetooth Modal Items */
  .bt-dev-meta {
    display: flex;
    flex-direction: column;
  }

  .bt-mac {
    font-size: 8.5px;
    color: #565f89;
    font-family: monospace;
  }

  .bt-bat-pill {
    display: flex;
    align-items: center;
    gap: 3px;
    font-size: 9.5px;
    font-weight: 700;
    color: #10b981;
    background: rgba(16, 185, 129, 0.12);
    padding: 2px 6px;
    border-radius: 99px;
  }

  .btn-connect-sm {
    background: #7aa2f7;
    color: #0f111a;
    font-weight: 700;
    font-size: 10px;
    border: none;
    border-radius: 6px;
    padding: 4px 10px;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .btn-disconnect-sm {
    background: rgba(247, 118, 142, 0.15);
    color: #f7768e;
    font-weight: 700;
    font-size: 10px;
    border: 1px solid rgba(247, 118, 142, 0.3);
    border-radius: 6px;
    padding: 4px 10px;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .btn-connect-sm:hover, .btn-disconnect-sm:hover {
    opacity: 0.85;
  }

  /* NOTIFICATIONS SECTION */
  .notifications-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-top: 2px;
  }

  .notif-section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1px 2px;
  }

  .notif-header-left {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .notif-section-title {
    font-size: 10px;
    font-weight: 700;
    color: #565f89;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .notif-count-pill {
    font-size: 8.5px;
    font-weight: 800;
    color: #7aa2f7;
    background: rgba(122, 162, 247, 0.18);
    padding: 1px 5px;
    border-radius: 99px;
  }

  .clear-all-btn {
    background: transparent;
    border: none;
    font-size: 9px;
    font-weight: 600;
    color: #565f89;
    cursor: pointer;
    transition: color 0.15s ease;
    padding: 0;
  }

  .clear-all-btn:hover {
    color: #f7768e;
  }

  .empty-notif-box {
    padding: 6px 10px;
    background: rgba(255, 255, 255, 0.02);
    border: 1px dashed rgba(255, 255, 255, 0.06);
    border-radius: 10px;
    text-align: center;
  }

  .empty-notif-text {
    font-size: 10px;
    color: #565f89;
    font-weight: 500;
  }

  .notif-scroll-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-height: 110px;
    overflow-y: auto;
    padding-right: 2px;
  }

  .notif-card {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-left: 3px solid #38bdf8;
    border-radius: 10px;
    padding: 6px 9px;
    display: flex;
    flex-direction: column;
    gap: 3px;
    transition: all 0.2s ease;
  }

  .notif-card:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(56, 189, 248, 0.4);
  }

  .notif-card.whatsapp {
    border-left: 3px solid #22c55e;
  }

  .notif-card.firefox {
    border-left: 3px solid #f97316;
  }

  .notif-card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .notif-card-app {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .app-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #38bdf8;
    box-shadow: 0 0 6px #38bdf8;
  }

  .notif-card.whatsapp .app-dot {
    background: #22c55e;
    box-shadow: 0 0 6px #22c55e;
  }

  .notif-card.firefox .app-dot {
    background: #f97316;
    box-shadow: 0 0 6px #f97316;
  }

  .notif-app {
    font-size: 10px;
    font-weight: 800;
    color: #38bdf8;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .notif-time {
    font-size: 8.5px;
    color: #94a3b8;
  }

  .notif-dismiss-btn {
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    border-radius: 4px;
    transition: color 0.15s ease;
  }

  .notif-dismiss-btn:hover {
    color: #ef4444;
  }

  .notif-card-content {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .notif-summary {
    font-size: 11.5px;
    font-weight: 700;
    color: #ffffff;
    line-height: 1.25;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .notif-body {
    font-size: 10.5px;
    color: #cbd5e1;
    line-height: 1.3;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
