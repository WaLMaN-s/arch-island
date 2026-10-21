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

  // Dynamic card dimensions for smooth, organic morphing animation
  let cardWidth = $state(430);
  let cardHeight = $state(320);
  let currentWindowH = 335;
  const WINDOW_W = 480;

  let shrinkTimer: any = null;

  function getTargetDimensions(tab: 'music' | 'controls', lyrics: boolean) {
    if (tab === 'controls') {
      return { w: 430, h: 320, winH: 335 };
    } else {
      if (lyrics) {
        return { w: 465, h: 360, winH: 375 };
      } else {
        return { w: 385, h: 215, winH: 230 };
      }
    }
  }

  function applyMorph(tab: 'music' | 'controls', lyrics: boolean, immediate = false) {
    const { w, h, winH } = getTargetDimensions(tab, lyrics);

    if (shrinkTimer) {
      clearTimeout(shrinkTimer);
      shrinkTimer = null;
    }

    if (immediate) {
      cardWidth = w;
      cardHeight = h;
      currentWindowH = winH;
      tauriBridge.resizeIsland('expanded', WINDOW_W, winH);
      return;
    }

    const isGrowing = winH > currentWindowH || w > cardWidth;

    if (isGrowing) {
      // 1. Expand X11/Tauri window first so animation isn't clipped
      currentWindowH = winH;
      tauriBridge.resizeIsland('expanded', WINDOW_W, winH);

      // 2. Animate CSS card outward smoothly
      requestAnimationFrame(() => {
        cardWidth = w;
        cardHeight = h;
      });
    } else {
      // 1. Animate CSS card inward smoothly (Tauri window remains large enough)
      cardWidth = w;
      cardHeight = h;

      // 2. Shrink X11/Tauri window AFTER visual transition completes (520ms)
      shrinkTimer = setTimeout(() => {
        currentWindowH = winH;
        tauriBridge.resizeIsland('expanded', WINDOW_W, winH);
      }, 520);
    }
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

  onMount(() => {
    // Initial size based on active tab
    applyMorph(currentTab, isLyricsMode, true);
  });

  function handleClose(e: MouseEvent) {
    e.stopPropagation();
    if (shrinkTimer) clearTimeout(shrinkTimer);
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
      <button class="close-btn" onclick={handleClose} title="Collapse (Esc)">
        <X size={15} color="#565f89" />
      </button>
    </div>
  </div>

  <!-- Content Body Stack: Keeps state intact and provides silky-smooth cross-fade & morphing -->
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
    transition: width 0.52s cubic-bezier(0.34, 1.25, 0.4, 1),
                height 0.52s cubic-bezier(0.34, 1.25, 0.4, 1);
    animation: expandSpring 0.35s cubic-bezier(0.34, 1.32, 0.5, 1);
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
    transition: opacity 0.38s cubic-bezier(0.16, 1, 0.3, 1),
                transform 0.48s cubic-bezier(0.34, 1.25, 0.4, 1),
                filter 0.38s ease;
  }

  .tab-pane.active {
    opacity: 1;
    transform: translateY(0) scale(1);
    filter: blur(0px);
    pointer-events: auto;
    z-index: 2;
  }

  .tab-pane.inactive {
    opacity: 0;
    transform: translateY(8px) scale(0.96);
    filter: blur(4px);
    pointer-events: none;
    z-index: 1;
  }

  @keyframes expandSpring {
    0% {
      opacity: 0.6;
      transform: scale(0.96) translateY(-8px);
    }
    100% {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }
</style>
