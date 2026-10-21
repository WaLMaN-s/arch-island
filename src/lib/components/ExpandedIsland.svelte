<script lang="ts">
  import { onMount } from 'svelte';
  import { islandState } from '$lib/stores/islandState.svelte';
  import { systemStore } from '$lib/stores/systemStore.svelte';
  import { tauriBridge } from '$lib/services/tauriBridge';
  import MusicCard from './MusicCard.svelte';
  import ControlCenter from './ControlCenter.svelte';
  import { Disc, Sliders, X } from 'lucide-svelte';

  let currentTab = $state<'music' | 'controls'>(
    systemStore.media?.status === 'Playing' ? 'music' : 'controls'
  );

  let isLyricsMode = $state(false);

  function getTargetDimensions(tab: 'music' | 'controls', lyrics: boolean) {
    if (tab === 'controls') {
      return { w: 430, h: 320 };
    } else {
      if (lyrics) {
        return { w: 465, h: 360 };
      } else {
        return { w: 385, h: 215 };
      }
    }
  }

  // Initialize dimensions matching the initial tab immediately (no mount jump)
  const initialDims = getTargetDimensions(
    systemStore.media?.status === 'Playing' ? 'music' : 'controls',
    false
  );
  let cardWidth = $state(initialDims.w);
  let cardHeight = $state(initialDims.h);

  function applyMorph(tab: 'music' | 'controls', lyrics: boolean) {
    const { w, h } = getTargetDimensions(tab, lyrics);
    cardWidth = w;
    cardHeight = h;
  }

  function handleSelectTab(tab: 'music' | 'controls') {
    if (currentTab === tab) {
      if (tab === 'music' && isLyricsMode) {
        handleLyricsToggle(false);
      }
      return;
    }
    currentTab = tab;
    applyMorph(tab, isLyricsMode);
  }

  function handleLyricsToggle(lyrics: boolean) {
    isLyricsMode = lyrics;
    if (currentTab === 'music') {
      applyMorph('music', lyrics);
    }
  }

  function handleClose(e: MouseEvent) {
    e.stopPropagation();
    islandState.collapse();
  }
</script>

<div
  class="expanded-card"
  style="width: {cardWidth}px; height: {cardHeight}px;"
>
  <!-- Header Bar -->
  <div class="card-header">
    <div class="tab-pill">
      <button
        class="tab-btn"
        class:active={currentTab === 'controls'}
        onclick={() => handleSelectTab('controls')}
      >
        <Sliders size={13} />
        <span>Control</span>
      </button>

      <button
        class="tab-btn"
        class:active={currentTab === 'music'}
        onclick={() => handleSelectTab('music')}
      >
        <Disc size={13} />
        <span>Music</span>
        {#if systemStore.media?.status === 'Playing'}
          <span class="active-dot"></span>
        {/if}
      </button>
    </div>

    <div class="header-right">
      <button class="close-btn" onclick={handleClose} title="Tutup Card">
        <X size={13} color="#94a3b8" />
      </button>
    </div>
  </div>

  <!-- Content Stack: Smooth crossfade transitions -->
  <div class="card-content-stack">
    <div
      class="tab-pane"
      class:active={currentTab === 'music'}
      class:inactive={currentTab !== 'music'}
    >
      <MusicCard onLyricsToggle={handleLyricsToggle} />
    </div>

    <div
      class="tab-pane"
      class:active={currentTab === 'controls'}
      class:inactive={currentTab !== 'controls'}
    >
      <ControlCenter />
    </div>
  </div>
</div>

<style>
  .expanded-card {
    box-sizing: border-box;
    user-select: none;
    background: rgba(16, 18, 27, 0.88);
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 20px;
    box-shadow: 0 20px 48px rgba(0, 0, 0, 0.6);
    display: flex;
    flex-direction: column;
    padding: 10px 14px;
    margin: 0 auto;
    overflow: hidden;
    /* Ultra-smooth Apple-style fluid easing with NO overshoot/vibration */
    transition: width 0.32s cubic-bezier(0.16, 1, 0.3, 1),
                height 0.32s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
    padding-bottom: 6px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    flex-shrink: 0;
  }

  .tab-pill {
    display: flex;
    align-items: center;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 99px;
    padding: 2px;
    gap: 2px;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 4px 10px;
    background: transparent;
    border: none;
    border-radius: 99px;
    font-size: 10.5px;
    font-weight: 700;
    color: #565f89;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .tab-btn:hover {
    color: #c0caf5;
  }

  .tab-btn.active {
    background: rgba(122, 162, 247, 0.18);
    color: #7aa2f7;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }

  .active-dot {
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: #73daca;
    box-shadow: 0 0 6px #73daca;
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .close-btn {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 50%;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .close-btn:hover {
    background: rgba(255, 255, 255, 0.12);
    transform: scale(1.08);
  }

  .card-content-stack {
    position: relative;
    flex: 1;
    min-height: 0;
    width: 100%;
    overflow: hidden;
  }

  .tab-pane {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    min-height: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    transition: opacity 0.25s cubic-bezier(0.16, 1, 0.3, 1),
                transform 0.28s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .tab-pane.active {
    opacity: 1;
    transform: translateY(0);
    pointer-events: auto;
    z-index: 2;
  }

  .tab-pane.inactive {
    opacity: 0;
    transform: translateY(6px);
    pointer-events: none;
    z-index: 1;
  }
</style>
