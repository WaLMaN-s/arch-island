<script lang="ts">
  import type { BatteryState } from '$lib/types';
  import { Zap } from 'lucide-svelte';

  let { battery }: { battery: BatteryState | null } = $props();

  let percentage = $derived(battery ? battery.percentage : 100);
  let isCharging = $derived(battery ? battery.is_charging : false);
  let isLow = $derived(percentage <= 20 && !isCharging);
</script>

<div class="battery-pill" class:charging={isCharging} class:low={isLow} title="{percentage}% - {isCharging ? 'Charging' : 'On battery'}">
  <div class="shell">
    <div class="level" style="width: {percentage}%;"></div>
    {#if isCharging}
      <div class="bolt-icon">
        <Zap size={10} color="#73daca" />
      </div>
    {/if}
  </div>
  <span class="pct-text">{percentage}%</span>
</div>

<style>
  .battery-pill {
    display: inline-flex;
    align-items: center;
    gap: 5px;
  }

  .shell {
    position: relative;
    width: 22px;
    height: 12px;
    border: 1.5px solid rgba(192, 202, 245, 0.4);
    border-radius: 4px;
    padding: 1px;
    display: flex;
    align-items: center;
    box-sizing: border-box;
  }

  .shell::after {
    content: '';
    position: absolute;
    right: -3px;
    top: 3px;
    width: 2px;
    height: 4px;
    background: rgba(192, 202, 245, 0.4);
    border-radius: 0 1px 1px 0;
  }

  .level {
    height: 100%;
    background: #9ece6a;
    border-radius: 1.5px;
    transition: width 0.4s ease, background-color 0.3s ease;
  }

  .battery-pill.charging .level {
    background: #73daca;
    box-shadow: 0 0 6px #73daca;
  }

  .battery-pill.low .level {
    background: #f7768e;
    box-shadow: 0 0 6px #f7768e;
  }

  .bolt-icon {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: pulse 1.2s ease-in-out infinite alternate;
  }

  .pct-text {
    font-size: 11px;
    font-weight: 600;
    color: #c0caf5;
    letter-spacing: -0.02em;
    font-variant-numeric: tabular-nums;
  }

  @keyframes pulse {
    0% {
      opacity: 0.6;
      transform: scale(0.9);
    }
    100% {
      opacity: 1;
      transform: scale(1.1);
    }
  }
</style>
