<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { islandState } from '$lib/stores/islandState.svelte';
  import { systemStore } from '$lib/stores/systemStore.svelte';
  import CompactIsland from '$lib/components/CompactIsland.svelte';
  import ExpandedIsland from '$lib/components/ExpandedIsland.svelte';
  import { listen } from '@tauri-apps/api/event';

  onMount(() => {
    systemStore.init();
    islandState.init();

    let unlistenClose: (() => void) | undefined;
    listen('close-requested', () => {
      if (islandState.mode === 'expanded') {
        islandState.collapse();
      }
    }).then((fn) => {
      unlistenClose = fn;
    });

    const handleKeydown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && islandState.mode === 'expanded') {
        islandState.collapse();
      }
    };

    const handleWindowLeave = () => {
      systemStore.setPollingRate(5000);
      islandState.onMouseLeave();
    };

    window.addEventListener('keydown', handleKeydown);
    window.addEventListener('mouseleave', handleWindowLeave);
    document.addEventListener('mouseleave', handleWindowLeave);

    return () => {
      unlistenClose?.();
      window.removeEventListener('keydown', handleKeydown);
      window.removeEventListener('mouseleave', handleWindowLeave);
      document.removeEventListener('mouseleave', handleWindowLeave);
    };
  });

  onDestroy(() => {
    systemStore.destroy();
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<main
  class="island-root"
  class:mode-tucked={islandState.mode === 'tucked'}
  class:mode-compact={islandState.mode === 'compact'}
  onmouseenter={() => {
    systemStore.setPollingRate(1500);
    islandState.onMouseEnter();
  }}
  onmouseleave={() => {
    systemStore.setPollingRate(5000);
    islandState.onMouseLeave();
  }}
  onclick={(e) => {
    if (e.target === e.currentTarget && islandState.mode === 'expanded' && !islandState.isLyricsOpen) {
      islandState.collapse();
    }
  }}
>
  {#if islandState.mode === 'tucked'}
    <!-- Ultra-slim notch lip at screen top edge (doesn't obstruct screen clicks) -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="tuck-indicator" onclick={() => islandState.onMouseEnter()}></div>
  {:else if islandState.mode === 'compact'}
    <div class="notch-spring-wrapper" class:retracting={islandState.isRetracting}>
      <CompactIsland />
    </div>
  {:else}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="expanded-wrapper"
      class:retracting={islandState.isRetracting}
      onclick={(e) => {
        if (e.target === e.currentTarget && islandState.mode === 'expanded' && !islandState.isLyricsOpen) {
          islandState.collapse();
        }
      }}
    >
      <ExpandedIsland />
    </div>
  {/if}
</main>

<style>
  :global(body, html) {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    background: transparent !important;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display", "Inter", sans-serif;
  }

  .island-root {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    background: transparent;
    user-select: none;
  }

  /* Tucked: completely clear trigger zone at top edge, no blur, no obstruction */
  .tuck-indicator {
    width: 100%;
    height: 100%;
    background: transparent;
    cursor: pointer;
  }

  /* Smooth gentle drop-down and retraction like Nusa (slow & nyaman, tidak kaget tiba-tiba) */
  .notch-spring-wrapper {
    animation: dropDownSmooth 0.5s cubic-bezier(0.34, 1.32, 0.5, 1) forwards;
    transform-origin: top center;
  }

  .notch-spring-wrapper.retracting {
    animation: slideUpSmooth 0.44s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  }

  @keyframes dropDownSmooth {
    0% {
      transform: translateY(-100%);
      opacity: 0.1;
    }
    65% {
      transform: translateY(2px) scaleY(1.02);
      opacity: 1;
    }
    85% {
      transform: translateY(-0.5px);
    }
    100% {
      transform: translateY(0);
      opacity: 1;
    }
  }

  @keyframes slideUpSmooth {
    0% {
      transform: translateY(0);
      opacity: 1;
    }
    100% {
      transform: translateY(-100%);
      opacity: 0;
    }
  }



  .expanded-wrapper {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 6px;
    box-sizing: border-box;
    animation: expandFade 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards;
  }

  .expanded-wrapper.retracting {
    animation: collapseFade 0.28s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  }

  @keyframes expandFade {
    0% {
      opacity: 0.6;
      transform: scale(0.96) translateY(-10px);
    }
    100% {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }

  @keyframes collapseFade {
    0% {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
    100% {
      opacity: 0;
      transform: scale(0.96) translateY(-12px);
    }
  }
</style>
