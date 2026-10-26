<script lang="ts">
  import { islandState } from '$lib/stores/islandState.svelte';
  import { systemStore } from '$lib/stores/systemStore.svelte';
  import WaveformVisualizer from './WaveformVisualizer.svelte';
  import NotificationPill from './NotificationPill.svelte';
  import ArchLogo from './ArchLogo.svelte';
  import {
    Battery,
    BatteryLow,
    BatteryMedium,
    Headphones,
    Monitor,
    Sun,
    Usb,
    Volume2,
    VolumeX,
    Wifi,
    WifiOff,
    Zap,
  } from 'lucide-svelte';

  function handleClick() {
    window.focus();
    islandState.toggle();
  }

  let pct = $derived(systemStore.battery?.percentage ?? 70);
  let isCharging = $derived(systemStore.battery?.is_charging ?? false);
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="notch-island" onclick={handleClick}>
  <!-- Inverted ear wings connecting flush to top edge (Nusa style) -->
  <div class="ear-wing left"></div>
  <div class="ear-wing right"></div>

  <!-- Camera sensor dot at top center -->
  <div class="camera-lens"></div>

  {#if islandState.activeHardwareEvent}
    <!-- Transient Hardware Notification: Pops down only for 2.8s when plugged/unplugged -->
    <div class="hardware-banner">
      <div class="hw-icon-circle" class:charging={islandState.activeHardwareEvent.kind === 'charger'}>
        {#if islandState.activeHardwareEvent.kind === 'charger'}
          <Zap size={11} color="#10b981" fill="#10b981" />
        {:else if islandState.activeHardwareEvent.kind === 'usb'}
          <Usb size={11} color="#7aa2f7" />
        {:else if islandState.activeHardwareEvent.kind === 'display'}
          <Monitor size={11} color="#38bdf8" />
        {:else}
          <Headphones size={11} color="#bb9af7" />
        {/if}
      </div>

      <div class="hw-text-group">
        <span class="hw-title">{islandState.activeHardwareEvent.title}</span>
        <span class="hw-subtitle">{islandState.activeHardwareEvent.subtitle}</span>
      </div>

      {#if islandState.activeHardwareEvent.percentage !== undefined && islandState.activeHardwareEvent.percentage !== null}
        <span class="big-percent">{islandState.activeHardwareEvent.percentage}%</span>
      {/if}
    </div>

  {:else if islandState.activeNotification}
    <NotificationPill notification={islandState.activeNotification} />

  {:else if islandState.hudType === 'volume'}
    <div class="hud-pill">
      {#if systemStore.volume.muted}
        <VolumeX size={11} color="#f7768e" />
      {:else}
        <Volume2 size={11} color="#7aa2f7" />
      {/if}
      <div class="hud-bar">
        <div class="hud-fill" style="width: {systemStore.volume.volume}%;"></div>
      </div>
      <span class="hud-val">{systemStore.volume.muted ? 'MUTED' : `${systemStore.volume.volume}%`}</span>
    </div>

  {:else if islandState.hudType === 'brightness'}
    <div class="hud-pill">
      <Sun size={11} color="#e0af68" />
      <div class="hud-bar">
        <div class="hud-fill brightness" style="width: {systemStore.brightness.brightness}%;"></div>
      </div>
      <span class="hud-val">{systemStore.brightness.brightness}%</span>
    </div>

  {:else}
    <!-- ARCH DYNAMIC ISLAND: Left (Wave/Arch Logo), Center (CLOCK), Right (Battery Icon + %) -->
    <div class="default-view">
      <!-- Left: Mini Waveform if music playing, else Arch logo when connected, else WifiOff -->
      <div class="left-cluster">
        {#if systemStore.media && systemStore.media.status === 'Playing'}
          <div class="mini-wave" title="{systemStore.media.title} - {systemStore.media.artist}">
            <WaveformVisualizer isPlaying={true} barCount={3} />
          </div>
        {:else if systemStore.network.connected}
          <div class="arch-chip" title="Arch Linux • {systemStore.network.ssid || 'Connected'}">
            <ArchLogo size={13} color="#1793d1" />
          </div>
        {:else}
          <div class="wifi-chip" title="Offline">
            <WifiOff size={11} color="#565f89" />
          </div>
        {/if}
      </div>

      <!-- Center: Tanggal & Jam (Date + Clock) -->
      <div class="center-cluster">
        <span class="date-chip">{systemStore.currentDate}</span>
        <span class="dot-divider">•</span>
        <span class="clock-display">{systemStore.currentTime}</span>
      </div>

      <!-- Right: Battery Icon + Percentage (and Bluetooth battery if connected) -->
      <div class="right-cluster">
        {#if systemStore.bluetooth.connected_device && systemStore.bluetooth.battery_percent !== null && systemStore.bluetooth.battery_percent !== undefined}
          <div class="bt-compact-pill" title="{systemStore.bluetooth.connected_device}: {systemStore.bluetooth.battery_percent}%">
            <Headphones size={11} color="#7aa2f7" />
            <span class="bt-compact-val">{systemStore.bluetooth.battery_percent}%</span>
          </div>
        {/if}

        <div class="battery-pill">
          {#if isCharging}
            <Zap size={10} color="#10b981" fill="#10b981" />
          {:else if pct <= 20}
            <BatteryLow size={13} color="#f7768e" />
          {:else if pct <= 50}
            <BatteryMedium size={13} color="#e0af68" />
          {:else}
            <Battery size={13} color="#73daca" />
          {/if}
          <span class="big-percent" class:charging={isCharging}>{pct}%</span>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .notch-island {
    position: relative;
    width: 330px;
    max-width: 100%;
    height: 32px;
    /* Elegant translucent dark glassmorphism (not pitch black) */
    background: rgba(16, 18, 27, 0.85);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-top: none;
    border-radius: 0 0 16px 16px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.45);
    display: flex;
    align-items: center;
    padding: 0 14px;
    cursor: pointer;
    user-select: none;
    box-sizing: border-box;
    transition: transform 0.15s cubic-bezier(0.16, 1, 0.3, 1),
                background 0.15s ease,
                border-color 0.15s ease;
  }

  .notch-island:hover {
    background: rgba(22, 25, 38, 0.92);
    border-color: rgba(122, 162, 247, 0.35);
  }

  /* Concave ear wings connecting smoothly to top screen bezel (Nusa formula) */
  .ear-wing {
    position: absolute;
    top: 0;
    width: 9px;
    height: 9px;
    pointer-events: none;
  }

  .ear-wing.left {
    left: -9px;
    background: radial-gradient(circle at 0% 100%, transparent 9px, rgba(16, 18, 27, 0.85) 9.5px);
  }

  .ear-wing.right {
    right: -9px;
    background: radial-gradient(circle at 100% 100%, transparent 9px, rgba(16, 18, 27, 0.85) 9.5px);
  }

  /* Camera dot */
  .camera-lens {
    position: absolute;
    top: 3px;
    left: 50%;
    transform: translateX(-50%);
    width: 3.5px;
    height: 3.5px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.18);
    pointer-events: none;
  }

  /* DEFAULT VIEW (Clock, Waveform/WiFi, Battery) */
  .default-view {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
  }

  .left-cluster {
    display: flex;
    align-items: center;
  }

  .clock-display {
    font-size: 13px;
    font-weight: 700;
    color: #ffffff;
    letter-spacing: -0.01em;
    font-variant-numeric: tabular-nums;
  }

  .center-cluster {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
  }

  .date-chip {
    font-size: 11px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.7);
    letter-spacing: -0.01em;
  }

  .dot-divider {
    font-size: 8px;
    color: rgba(255, 255, 255, 0.35);
  }

  .mini-wave {
    display: flex;
    align-items: center;
  }

  .wifi-chip {
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.8;
  }

  .arch-chip {
    display: flex;
    align-items: center;
    justify-content: center;
    filter: drop-shadow(0 0 6px rgba(23, 147, 209, 0.45));
    transition: transform 0.2s ease;
  }

  .arch-chip:hover {
    transform: scale(1.1);
  }

  .right-cluster {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .bt-compact-pill {
    display: flex;
    align-items: center;
    gap: 3px;
    background: rgba(122, 162, 247, 0.12);
    border: 1px solid rgba(122, 162, 247, 0.2);
    padding: 1px 5px;
    border-radius: 99px;
  }

  .bt-compact-val {
    font-size: 10.5px;
    font-weight: 700;
    color: #7aa2f7;
    font-variant-numeric: tabular-nums;
  }

  .battery-pill {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .big-percent {
    font-size: 13px;
    font-weight: 700;
    color: #ffffff;
    font-variant-numeric: tabular-nums;
  }

  .big-percent.charging {
    color: #10b981;
  }

  /* Hardware Notification Banner */
  .hardware-banner {
    display: flex;
    align-items: center;
    width: 100%;
    gap: 8px;
    animation: fadeIn 0.15s ease;
  }

  .hw-icon-circle {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.08);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .hw-icon-circle.charging {
    background: rgba(16, 185, 129, 0.16);
  }

  .hw-text-group {
    flex: 1;
    display: flex;
    flex-direction: column;
    line-height: 1.1;
  }

  .hw-title {
    font-size: 10.5px;
    font-weight: 600;
    color: #ffffff;
  }

  .hw-subtitle {
    font-size: 8.5px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.6);
  }

  /* HUD */
  .hud-pill {
    display: flex;
    align-items: center;
    width: 100%;
    gap: 8px;
  }

  .hud-bar {
    flex: 1;
    height: 4px;
    background: rgba(255, 255, 255, 0.12);
    border-radius: 99px;
    overflow: hidden;
  }

  .hud-fill {
    height: 100%;
    background: #7aa2f7;
    border-radius: 99px;
    transition: width 0.1s ease;
  }

  .hud-fill.brightness {
    background: #e0af68;
  }

  .hud-val {
    font-size: 10px;
    font-weight: 700;
    color: #c0caf5;
    min-width: 28px;
    text-align: right;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-2px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
