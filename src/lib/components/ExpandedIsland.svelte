<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { islandState } from '$lib/stores/islandState.svelte';
  import { systemStore } from '$lib/stores/systemStore.svelte';
  import { tauriBridge } from '$lib/services/tauriBridge';
  import MusicCard from './MusicCard.svelte';
  import ControlCenter from './ControlCenter.svelte';
  import TodoList from './TodoList.svelte';
  import CollegeSchedule from './CollegeSchedule.svelte';
  import { Disc, Sliders, ListTodo, GraduationCap, Power, Lock, Moon, RotateCw, X } from 'lucide-svelte';

  let currentTab = $state<'music' | 'controls' | 'tasks' | 'kuliah'>(
    systemStore.media?.status === 'Playing' ? 'music' : 'controls'
  );

  let isLyricsMode = $state(false);
  let showPowerMenu = $state(false);
  let pendingPowerAction = $state<'lock' | 'suspend' | 'reboot' | 'shutdown' | null>(null);

  function getTargetDimensions(tab: 'music' | 'controls' | 'tasks' | 'kuliah', lyrics: boolean) {
    if (tab === 'controls') {
      return { w: 430, h: 320 };
    } else if (tab === 'tasks') {
      return { w: 400, h: 330 };
    } else if (tab === 'kuliah') {
      return { w: 440, h: 350 };
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

  function applyMorph(tab: 'music' | 'controls' | 'tasks' | 'kuliah', lyrics: boolean) {
    const { w, h } = getTargetDimensions(tab, lyrics);
    cardWidth = w;
    cardHeight = h;
  }

  function handleSelectTab(tab: 'music' | 'controls' | 'tasks' | 'kuliah') {
    if (currentTab === tab) {
      if (tab === 'music' && isLyricsMode) {
        handleLyricsToggle(false);
      }
      return;
    }
    currentTab = tab;
    if (isLyricsMode) {
      handleLyricsToggle(false);
    }
    applyMorph(tab, false);
  }

  function handleLyricsToggle(lyrics: boolean) {
    isLyricsMode = lyrics;
    islandState.isLyricsOpen = lyrics;
    if (currentTab === 'music') {
      applyMorph('music', lyrics);
    }
  }

  function handleClose(e: MouseEvent) {
    e.stopPropagation();
    handleLyricsToggle(false);
    showPowerMenu = false;
    islandState.collapse();
  }

  async function triggerPower(action: 'lock' | 'suspend' | 'reboot' | 'shutdown') {
    if (pendingPowerAction !== action) {
      pendingPowerAction = action;
      return;
    }
    // Confirmed second click
    await tauriBridge.executePowerAction(action);
    showPowerMenu = false;
    pendingPowerAction = null;
    islandState.collapse();
  }

  let controlTabBtn = $state<HTMLButtonElement | null>(null);
  let cardElement = $state<HTMLElement | null>(null);

  onMount(() => {
    window.focus();
    requestAnimationFrame(() => {
      controlTabBtn?.focus();
    });
    setTimeout(() => {
      controlTabBtn?.focus();
    }, 50);
  });

  onDestroy(() => {
    islandState.isLyricsOpen = false;
    showPowerMenu = false;
  });

  const TABS: ('controls' | 'music' | 'tasks' | 'kuliah')[] = ['controls', 'music', 'tasks', 'kuliah'];
  const POWER_ACTIONS: ('lock' | 'suspend' | 'reboot' | 'shutdown')[] = ['lock', 'suspend', 'reboot', 'shutdown'];
  let powerFocusIndex = $state<number>(0);

  function handleKeyDown(e: KeyboardEvent) {
    if (islandState.mode !== 'expanded') return;

    const target = e.target as HTMLElement | null;
    // Don't hijack if typing in text/search/password inputs or textarea
    if (
      (target instanceof HTMLInputElement && (target.type === 'text' || target.type === 'password' || target.type === 'search')) ||
      target instanceof HTMLTextAreaElement
    ) {
      return;
    }

    // Toggle Power menu with 'p' key
    if ((e.key === 'p' || e.key === 'P') && !showPowerMenu) {
      e.preventDefault();
      showPowerMenu = true;
      powerFocusIndex = 0;
      pendingPowerAction = null;
      return;
    }

    // Power Menu Keyboard Navigation: ArrowUp / ArrowDown / Enter / Escape
    if (showPowerMenu) {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        powerFocusIndex = (powerFocusIndex + 1) % POWER_ACTIONS.length;
        pendingPowerAction = null;
        return;
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        powerFocusIndex = (powerFocusIndex - 1 + POWER_ACTIONS.length) % POWER_ACTIONS.length;
        pendingPowerAction = null;
        return;
      } else if (e.key === 'Enter') {
        e.preventDefault();
        triggerPower(POWER_ACTIONS[powerFocusIndex]);
        return;
      } else if (e.key === 'Escape') {
        e.preventDefault();
        showPowerMenu = false;
        pendingPowerAction = null;
        controlTabBtn?.focus();
        return;
      } else if (e.key === 'ArrowLeft') {
        e.preventDefault();
        showPowerMenu = false;
        pendingPowerAction = null;
        handleSelectTab('kuliah');
        return;
      } else if (e.key === 'ArrowRight') {
        e.preventDefault();
        showPowerMenu = false;
        pendingPowerAction = null;
        handleSelectTab('controls');
        return;
      }
    }

    // Left and Right arrows: cycle tabs including Power menu
    if (e.key === 'ArrowRight') {
      e.preventDefault();
      const idx = TABS.indexOf(currentTab);
      if (idx === TABS.length - 1) {
        // Dari tab Kuliah, panah kanan lanjut membuka menu Power
        showPowerMenu = true;
        powerFocusIndex = 0;
        pendingPowerAction = null;
      } else {
        handleSelectTab(TABS[idx + 1]);
      }
    } else if (e.key === 'ArrowLeft') {
      e.preventDefault();
      const idx = TABS.indexOf(currentTab);
      if (idx === 0) {
        // Dari tab Control, panah kiri mundur membuka menu Power
        showPowerMenu = true;
        powerFocusIndex = 0;
        pendingPowerAction = null;
      } else {
        handleSelectTab(TABS[idx - 1]);
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  bind:this={cardElement}
  tabindex="-1"
  class="expanded-card"
  style="width: {cardWidth}px; height: {cardHeight}px;"
  onmouseenter={() => islandState.onMouseEnter()}
  onmouseleave={() => islandState.onMouseLeave()}
  onclick={(e) => e.stopPropagation()}
>
  <!-- Header Bar -->
  <div class="card-header">
    <div class="tab-pill">
      <button
        bind:this={controlTabBtn}
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

      <button
        class="tab-btn"
        class:active={currentTab === 'tasks'}
        onclick={() => handleSelectTab('tasks')}
      >
        <ListTodo size={13} />
        <span>Tasks</span>
      </button>

      <button
        class="tab-btn"
        class:active={currentTab === 'kuliah'}
        onclick={() => handleSelectTab('kuliah')}
      >
        <GraduationCap size={13} />
        <span>Kuliah</span>
      </button>
    </div>

    <div class="header-right">
      <div class="power-menu-wrapper">
        <button
          class="power-btn"
          class:active={showPowerMenu}
          onclick={(e) => {
            e.stopPropagation();
            showPowerMenu = !showPowerMenu;
            powerFocusIndex = 0;
            pendingPowerAction = null;
          }}
          title="Menu Daya (Power)"
        >
          <Power size={12} />
        </button>

        {#if showPowerMenu}
          <div class="power-dropdown" onclick={(e) => e.stopPropagation()}>
            <button
              class="power-item"
              class:focused={powerFocusIndex === 0}
              class:confirm={pendingPowerAction === 'lock'}
              onclick={() => triggerPower('lock')}
              onmouseenter={() => (powerFocusIndex = 0)}
            >
              <Lock size={12} />
              <span>{pendingPowerAction === 'lock' ? 'Yakin Kunci?' : 'Lock'}</span>
            </button>
            <button
              class="power-item"
              class:focused={powerFocusIndex === 1}
              class:confirm={pendingPowerAction === 'suspend'}
              onclick={() => triggerPower('suspend')}
              onmouseenter={() => (powerFocusIndex = 1)}
            >
              <Moon size={12} />
              <span>{pendingPowerAction === 'suspend' ? 'Yakin Sleep?' : 'Sleep'}</span>
            </button>
            <button
              class="power-item"
              class:focused={powerFocusIndex === 2}
              class:confirm={pendingPowerAction === 'reboot'}
              onclick={() => triggerPower('reboot')}
              onmouseenter={() => (powerFocusIndex = 2)}
            >
              <RotateCw size={12} />
              <span>{pendingPowerAction === 'reboot' ? 'Yakin Reboot?' : 'Reboot'}</span>
            </button>
            <button
              class="power-item danger"
              class:focused={powerFocusIndex === 3}
              class:confirm={pendingPowerAction === 'shutdown'}
              onclick={() => triggerPower('shutdown')}
              onmouseenter={() => (powerFocusIndex = 3)}
            >
              <Power size={12} />
              <span>{pendingPowerAction === 'shutdown' ? 'Yakin Matikan?' : 'Power Off'}</span>
            </button>
          </div>
        {/if}
      </div>

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

    <div
      class="tab-pane"
      class:active={currentTab === 'tasks'}
      class:inactive={currentTab !== 'tasks'}
    >
      <TodoList />
    </div>

    <div
      class="tab-pane"
      class:active={currentTab === 'kuliah'}
      class:inactive={currentTab !== 'kuliah'}
    >
      <CollegeSchedule />
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

  .power-menu-wrapper {
    position: relative;
  }

  .power-btn {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 50%;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: #94a3b8;
    transition: all 0.2s ease;
  }

  .power-btn:hover,
  .power-btn.active {
    background: rgba(239, 68, 68, 0.15);
    color: #f87171;
    border-color: rgba(239, 68, 68, 0.3);
  }

  .power-dropdown {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    width: 130px;
    background: rgba(18, 20, 29, 0.96);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 12px;
    padding: 4px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    box-shadow: 0 12px 28px rgba(0, 0, 0, 0.6);
    z-index: 100;
    backdrop-filter: blur(16px);
  }

  .power-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    background: transparent;
    border: none;
    border-radius: 8px;
    color: #cbd5e1;
    font-size: 11px;
    cursor: pointer;
    text-align: left;
    transition: background 0.15s, color 0.15s;
  }

  .power-item:hover,
  .power-item.focused {
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
    outline: 1px solid rgba(56, 189, 248, 0.45);
  }

  .power-item.danger:hover,
  .power-item.danger.focused {
    background: rgba(239, 68, 68, 0.2);
    color: #ef4444;
    outline: 1px solid rgba(239, 68, 68, 0.5);
  }

  .power-item.confirm {
    background: rgba(239, 68, 68, 0.25);
    color: #fca5a5;
    font-weight: 600;
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
