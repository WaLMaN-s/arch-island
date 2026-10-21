<script lang="ts">
  let { isPlaying = false, barCount = 4 }: { isPlaying?: boolean; barCount?: number } = $props();
</script>

<div class="waveform-container" class:playing={isPlaying} aria-label="Audio Visualizer">
  {#each Array(barCount) as _, i}
    <div
      class="bar"
      style="animation-delay: {i * 0.15}s; animation-duration: {0.6 + (i % 3) * 0.2}s;"
    ></div>
  {/each}
</div>

<style>
  .waveform-container {
    display: flex;
    align-items: center;
    gap: 3px;
    height: 16px;
  }

  .bar {
    width: 3px;
    height: 6px;
    background: linear-gradient(180deg, #7aa2f7 0%, #bb9af7 100%);
    border-radius: 99px;
    transition: height 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .waveform-container.playing .bar {
    animation: bounce ease-in-out infinite alternate;
  }

  @keyframes bounce {
    0% {
      height: 4px;
    }
    100% {
      height: 16px;
    }
  }
</style>
