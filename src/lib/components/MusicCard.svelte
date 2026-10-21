<script lang="ts">
  import { systemStore } from '$lib/stores/systemStore.svelte';
  import { islandState } from '$lib/stores/islandState.svelte';
  import { tauriBridge } from '$lib/services/tauriBridge';
  import WaveformVisualizer from './WaveformVisualizer.svelte';
  import {
    Disc,
    Play,
    Pause,
    SkipBack,
    SkipForward,
    Mic2,
    Music2,
    ChevronDown,
    ListMusic,
    Maximize2,
  } from 'lucide-svelte';

  let media = $derived(systemStore.media);
  let isPlaying = $derived(media?.status === 'Playing');

  let { onLyricsToggle }: { onLyricsToggle?: (lyrics: boolean) => void } = $props();

  // View state: 'player' or 'lyrics'
  let viewMode = $state<'player' | 'lyrics'>('player');

  function openLyrics() {
    viewMode = 'lyrics';
    onLyricsToggle?.(true);
  }

  function closeLyrics() {
    viewMode = 'player';
    onLyricsToggle?.(false);
  }

  async function handleOpenSpotifyLyrics() {
    islandState.collapse();
    await tauriBridge.openSpotifyLyrics();
  }

  // Lyrics state
  interface LyricLine {
    time: number;
    text: string;
  }

  let lyricsData = $state<{ synced: LyricLine[]; plain?: string } | null>(null);
  let isLoadingLyrics = $state(false);
  let lastFetchedTrack = $state<string>('');
  let lyricsContainerEl = $state<HTMLDivElement | null>(null);

  function formatTime(seconds: number): string {
    if (!seconds || isNaN(seconds) || seconds < 0) return '0:00';
    const m = Math.floor(seconds / 60);
    const s = Math.floor(seconds % 60);
    return `${m}:${s < 10 ? '0' : ''}${s}`;
  }

  let progressPercent = $derived(() => {
    if (!media || !media.duration_sec || media.duration_sec <= 0) return 0;
    return Math.min(100, Math.max(0, (media.position_sec / media.duration_sec) * 100));
  });

  function handleSeek(e: MouseEvent) {
    if (!media || !media.duration_sec || media.duration_sec <= 0) return;
    const target = e.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const ratio = Math.max(0, Math.min(1, clickX / rect.width));
    const targetSec = ratio * media.duration_sec;
    systemStore.seek(targetSec);
  }

  function parseLrc(lrcText: string): LyricLine[] {
    const lines = lrcText.split('\n');
    const result: LyricLine[] = [];
    const regex = /\[(\d{2}):(\d{2})\.(\d{2,3})\](.*)/;

    for (const line of lines) {
      const match = line.match(regex);
      if (match) {
        const min = parseInt(match[1], 10);
        const sec = parseInt(match[2], 10);
        const ms = parseInt(match[3].padEnd(3, '0').slice(0, 3), 10);
        const time = min * 60 + sec + ms / 1000;
        const text = match[4].trim();
        if (text) {
          result.push({ time, text });
        }
      }
    }
    return result.sort((a, b) => a.time - b.time);
  }

  async function loadLyrics() {
    if (!media?.title || !media?.artist) {
      lyricsData = null;
      return;
    }

    const trackKey = `${media.title}-${media.artist}`;
    if (trackKey === lastFetchedTrack) return;

    lastFetchedTrack = trackKey;
    isLoadingLyrics = true;

    try {
      const data = await tauriBridge.fetchLyrics(media.title, media.artist);
      if (data?.syncedLyrics) {
        lyricsData = {
          synced: parseLrc(data.syncedLyrics),
          plain: data.plainLyrics,
        };
      } else if (data?.plainLyrics) {
        lyricsData = {
          synced: [],
          plain: data.plainLyrics,
        };
      } else {
        lyricsData = null;
      }
    } catch {
      lyricsData = null;
    } finally {
      isLoadingLyrics = false;
    }
  }

  // Reactively fetch lyrics when track title or viewMode changes
  $effect(() => {
    if (media?.title && media?.artist) {
      loadLyrics();
    }
  });

  // Calculate current active lyric line
  let activeLyricIndex = $derived(() => {
    if (!lyricsData || !lyricsData.synced.length || !media) return -1;
    const pos = media.position_sec;
    let idx = -1;
    for (let i = 0; i < lyricsData.synced.length; i++) {
      if (lyricsData.synced[i].time <= pos) {
        idx = i;
      } else {
        break;
      }
    }
    return idx;
  });

  // Auto-scroll lyrics container when active line updates
  $effect(() => {
    const idx = activeLyricIndex();
    if (idx >= 0 && viewMode === 'lyrics' && lyricsContainerEl) {
      const lineEl = document.getElementById(`lyric-line-${idx}`);
      if (lineEl) {
        lineEl.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }
    }
  });

  function jumpToLyric(timeSec: number) {
    systemStore.seek(timeSec);
  }
</script>

<div class="music-card">
  {#if media && media.title}
    {#if viewMode === 'player'}
      <!-- Standard Artwork & Playback View -->
      <div class="card-body">
        <!-- Left: Album Art (Clickable to switch to Lyrics) -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="artwork-container" onclick={openLyrics} title="Klik untuk lihat Lirik">
          {#if media.art_url && media.art_url.startsWith('http')}
            <img src={media.art_url} alt="Album Art" class="album-art" />
          {:else}
            <div class="art-placeholder" class:playing={isPlaying}>
              <Disc size={36} color="#7aa2f7" />
            </div>
          {/if}
          <div class="floating-visualizer">
            <WaveformVisualizer {isPlaying} barCount={5} />
          </div>
          <div class="art-hover-overlay">
            <Mic2 size={20} color="#ffffff" />
            <span>Lirik</span>
          </div>
        </div>

        <!-- Right: Track Info & Controls -->
        <div class="track-details">
          <div class="meta-row">
            <span class="player-pill">{media.player}</span>
            <div class="meta-actions">
              <button
                class="lyrics-badge-btn"
                onclick={openLyrics}
                title="Buka Lirik Lagu"
              >
                <Mic2 size={11} color="#7aa2f7" />
                <span>Lirik</span>
              </button>
              <span class="status-indicator" class:playing={isPlaying}>
                {media.status}
              </span>
            </div>
          </div>

          <h3 class="track-title" title={media.title}>{media.title}</h3>
          <p class="track-artist" title={media.artist}>{media.artist}</p>
          {#if media.album}
            <p class="track-album" title={media.album}>{media.album}</p>
          {/if}

          <!-- Progress Bar -->
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="progress-container" onclick={handleSeek}>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {progressPercent()}%;"></div>
            </div>
            <div class="time-labels">
              <span>{formatTime(media.position_sec)}</span>
              <span>{formatTime(media.duration_sec)}</span>
            </div>
          </div>

          <!-- Media Controls -->
          <div class="controls-row">
            <button class="ctrl-btn" onclick={() => systemStore.prevTrack()} title="Previous Track">
              <SkipBack size={18} color="#c0caf5" />
            </button>

            <button class="ctrl-btn play-btn" onclick={() => systemStore.togglePlayPause()} title={isPlaying ? 'Pause' : 'Play'}>
              {#if isPlaying}
                <Pause size={20} color="#000000" />
              {:else}
                <Play size={20} color="#000000" style="margin-left: 2px;" />
              {/if}
            </button>

            <button class="ctrl-btn" onclick={() => systemStore.nextTrack()} title="Next Track">
              <SkipForward size={18} color="#c0caf5" />
            </button>
          </div>
        </div>
      </div>

    {:else}
      <!-- Synced Lyrics View (Real-time LRC, no Spotify needed!) -->
      <div class="lyrics-view">
        <!-- Top Lyrics Header Bar -->
        <div class="lyrics-topbar">
          <button class="back-pill-btn" onclick={closeLyrics} title="Kembali ke Info Lagu">
            <ChevronDown size={14} />
            <span>Kembali</span>
          </button>

          <div class="now-playing-pill" title="{media.title} - {media.artist}">
            <span class="title-cut">{media.title}</span>
            <span class="dot-sep">•</span>
            <span class="artist-cut">{media.artist}</span>
          </div>

          <div class="lyrics-actions">
            {#if lyricsData?.synced.length}
              <span class="sync-indicator">Real-time</span>
            {/if}

            <button
              class="expand-spotify-btn"
              onclick={handleOpenSpotifyLyrics}
              title="Lebarkan ke Layar Penuh Spotify Lirik"
            >
              <Maximize2 size={11} />
              <span>Spotify Lirik</span>
            </button>
          </div>
        </div>

        <!-- Scrollable Lyrics Container -->
        <div class="lyrics-scroll-box" bind:this={lyricsContainerEl}>
          {#if isLoadingLyrics}
            <div class="lyrics-status-box">
              <div class="loading-spinner"></div>
              <span>Mencari lirik lagu...</span>
            </div>
          {:else if lyricsData && lyricsData.synced.length > 0}
            <div class="lyrics-lines-list">
              {#each lyricsData.synced as line, i}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div
                  id="lyric-line-{i}"
                  class="lyric-line"
                  class:active={activeLyricIndex() === i}
                  class:passed={activeLyricIndex() > i}
                  onclick={() => jumpToLyric(line.time)}
                  title="Klik untuk melompat ke {formatTime(line.time)}"
                >
                  <span class="line-text">{line.text}</span>
                </div>
              {/each}
            </div>
          {:else if lyricsData && lyricsData.plain}
            <div class="plain-lyrics-box">
              <pre class="plain-text">{lyricsData.plain}</pre>
            </div>
          {:else}
            <div class="lyrics-status-box">
              <Music2 size={24} color="#565f89" />
              <p class="status-main">Lirik tidak ditemukan untuk lagu ini</p>
              <p class="status-sub">Mungkin berupa audio instrumental atau lagu lokal</p>
            </div>
          {/if}
        </div>

        <!-- Compact Bottom Player Bar in Lyrics View -->
        <div class="lyrics-bottom-bar">
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="mini-progress-bar" onclick={handleSeek}>
            <div class="mini-fill" style="width: {progressPercent()}%;"></div>
          </div>

          <div class="mini-controls-row">
            <span class="mini-time">{formatTime(media.position_sec)} / {formatTime(media.duration_sec)}</span>
            <div class="mini-btns">
              <button class="mini-btn" onclick={() => systemStore.prevTrack()}>
                <SkipBack size={14} color="#c0caf5" />
              </button>
              <button class="mini-play" onclick={() => systemStore.togglePlayPause()}>
                {#if isPlaying}
                  <Pause size={14} color="#000000" />
                {:else}
                  <Play size={14} color="#000000" style="margin-left: 1px;" />
                {/if}
              </button>
              <button class="mini-btn" onclick={() => systemStore.nextTrack()}>
                <SkipForward size={14} color="#c0caf5" />
              </button>
            </div>
          </div>
        </div>
      </div>
    {/if}

  {:else}
    <div class="no-player-state">
      <div class="icon-ring">
        <Disc size={32} color="#565f89" />
      </div>
      <p class="no-player-title">No Media Playing</p>
      <p class="no-player-subtitle">Open Spotify, Firefox, MPV, or VLC to start playing audio</p>
    </div>
  {/if}
</div>

<style>
  .music-card {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    position: relative;
  }

  .card-body {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 2px 2px;
    animation: fadeIn 0.2s ease;
  }

  .artwork-container {
    position: relative;
    width: 110px;
    height: 110px;
    flex-shrink: 0;
    border-radius: 16px;
    overflow: hidden;
    cursor: pointer;
    box-shadow: 0 8px 22px rgba(0, 0, 0, 0.55), 0 0 1px 1px rgba(255, 255, 255, 0.12);
  }

  .artwork-container:hover .art-hover-overlay {
    opacity: 1;
  }

  .art-hover-overlay {
    position: absolute;
    inset: 0;
    background: rgba(10, 12, 18, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 4px;
    color: #ffffff;
    font-size: 11px;
    font-weight: 700;
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .album-art {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .art-placeholder {
    width: 100%;
    height: 100%;
    background: radial-gradient(circle at center, #1f2335 0%, #16161e 100%);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .art-placeholder.playing {
    animation: pulseGlow 2.5s ease-in-out infinite alternate;
  }

  .floating-visualizer {
    position: absolute;
    bottom: 5px;
    right: 5px;
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(8px);
    padding: 3px 6px;
    border-radius: 99px;
  }

  .track-details {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .meta-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 4px;
  }

  .meta-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .lyrics-badge-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    background: rgba(122, 162, 247, 0.12);
    border: 1px solid rgba(122, 162, 247, 0.25);
    padding: 2px 8px;
    border-radius: 99px;
    color: #7aa2f7;
    font-size: 10px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .lyrics-badge-btn:hover {
    background: rgba(122, 162, 247, 0.25);
    transform: scale(1.04);
  }

  .player-pill {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: #7aa2f7;
    background: rgba(122, 162, 247, 0.12);
    padding: 2px 8px;
    border-radius: 99px;
  }

  .status-indicator {
    font-size: 11px;
    font-weight: 600;
    color: #565f89;
  }

  .status-indicator.playing {
    color: #73daca;
  }

  .track-title {
    font-size: 16px;
    font-weight: 700;
    color: #c0caf5;
    margin: 0 0 2px 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    letter-spacing: -0.02em;
  }

  .track-artist {
    font-size: 13px;
    font-weight: 600;
    color: #bb9af7;
    margin: 0 0 2px 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .track-album {
    font-size: 11px;
    color: #565f89;
    margin: 0 0 6px 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Progress */
  .progress-container {
    cursor: pointer;
    margin-bottom: 8px;
  }

  .progress-bar {
    width: 100%;
    height: 5px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 99px;
    overflow: hidden;
    transition: height 0.15s ease;
  }

  .progress-container:hover .progress-bar {
    height: 7px;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #7aa2f7 0%, #bb9af7 100%);
    border-radius: 99px;
    transition: width 0.1s linear;
  }

  .time-labels {
    display: flex;
    justify-content: space-between;
    font-size: 10px;
    font-weight: 600;
    color: #565f89;
    margin-top: 3px;
    font-variant-numeric: tabular-nums;
  }

  /* Controls */
  .controls-row {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 16px;
    margin-top: 2px;
    padding-bottom: 4px;
  }

  .ctrl-btn {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 50%;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .ctrl-btn:hover {
    background: rgba(255, 255, 255, 0.12);
    transform: scale(1.08);
  }

  .ctrl-btn:active {
    transform: scale(0.95);
  }

  .play-btn {
    width: 38px;
    height: 38px;
    background: #c0caf5;
    border: none;
    box-shadow: 0 0 16px rgba(192, 202, 245, 0.3);
  }

  .play-btn:hover {
    background: #ffffff;
    box-shadow: 0 0 22px rgba(255, 255, 255, 0.5);
  }

  /* LYRICS VIEW */
  .lyrics-view {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    width: 100%;
    animation: fadeIn 0.2s ease;
  }

  .lyrics-topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 6px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    flex-shrink: 0;
  }

  .back-pill-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 99px;
    padding: 3px 8px;
    color: #c0caf5;
    font-size: 10px;
    font-weight: 700;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .back-pill-btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .now-playing-pill {
    max-width: 200px;
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 10.5px;
    color: #a9b1d6;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .title-cut {
    font-weight: 700;
    color: #ffffff;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .dot-sep {
    color: #565f89;
  }

  .artist-cut {
    color: #bb9af7;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .lyrics-actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .sync-indicator {
    font-size: 9px;
    font-weight: 700;
    color: #10b981;
    background: rgba(16, 185, 129, 0.12);
    padding: 2px 6px;
    border-radius: 99px;
  }

  .expand-spotify-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    background: rgba(30, 215, 96, 0.12);
    border: 1px solid rgba(30, 215, 96, 0.3);
    border-radius: 99px;
    padding: 2px 8px;
    color: #1ed760;
    font-size: 9.5px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .expand-spotify-btn:hover {
    background: rgba(30, 215, 96, 0.25);
    transform: scale(1.04);
  }

  .lyrics-scroll-box {
    flex: 1;
    overflow-y: auto;
    padding: 14px 6px;
    display: flex;
    flex-direction: column;
    scroll-behavior: smooth;
    mask-image: linear-gradient(to bottom, transparent 0%, black 15%, black 85%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 15%, black 85%, transparent 100%);
  }

  .lyrics-lines-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 20px 0;
  }

  .lyric-line {
    padding: 6px 12px;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.16, 1, 0.3, 1);
    opacity: 0.35;
    transform: scale(0.97);
    transform-origin: left center;
  }

  .lyric-line:hover {
    opacity: 0.7;
    background: rgba(255, 255, 255, 0.04);
  }

  .lyric-line.active {
    opacity: 1;
    transform: scale(1.05);
    background: rgba(122, 162, 247, 0.14);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
  }

  .lyric-line.active .line-text {
    font-size: 15px;
    font-weight: 800;
    color: #ffffff;
    text-shadow: 0 0 12px rgba(122, 162, 247, 0.5);
  }

  .line-text {
    font-size: 13.5px;
    font-weight: 600;
    color: #c0caf5;
    line-height: 1.4;
    transition: all 0.2s ease;
  }

  .plain-lyrics-box {
    padding: 10px;
  }

  .plain-text {
    font-family: inherit;
    font-size: 12px;
    line-height: 1.6;
    color: #c0caf5;
    white-space: pre-wrap;
    margin: 0;
  }

  .lyrics-status-box {
    margin: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    text-align: center;
    color: #565f89;
    padding: 20px;
  }

  .loading-spinner {
    width: 20px;
    height: 20px;
    border: 2px solid rgba(122, 162, 247, 0.2);
    border-top-color: #7aa2f7;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .status-main {
    font-size: 12.5px;
    font-weight: 600;
    color: #c0caf5;
    margin: 0;
  }

  .status-sub {
    font-size: 10.5px;
    margin: 0;
  }

  /* Compact Bottom Bar */
  .lyrics-bottom-bar {
    padding-top: 6px;
    flex-shrink: 0;
  }

  .mini-progress-bar {
    width: 100%;
    height: 3px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 99px;
    overflow: hidden;
    cursor: pointer;
    margin-bottom: 6px;
  }

  .mini-fill {
    height: 100%;
    background: #7aa2f7;
    border-radius: 99px;
  }

  .mini-controls-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .mini-time {
    font-size: 10px;
    font-weight: 600;
    color: #565f89;
    font-variant-numeric: tabular-nums;
  }

  .mini-btns {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .mini-btn {
    background: transparent;
    border: none;
    padding: 4px;
    cursor: pointer;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.15s ease;
  }

  .mini-btn:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .mini-play {
    width: 26px;
    height: 26px;
    background: #c0caf5;
    border: none;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .mini-play:hover {
    background: #ffffff;
    transform: scale(1.08);
  }

  /* Empty State */
  .no-player-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 20px;
  }

  .icon-ring {
    width: 58px;
    height: 58px;
    border-radius: 50%;
    background: #16161e;
    border: 1px dashed rgba(255, 255, 255, 0.15);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 12px;
  }

  .no-player-title {
    font-size: 15px;
    font-weight: 700;
    color: #c0caf5;
    margin: 0 0 4px 0;
  }

  .no-player-subtitle {
    font-size: 12px;
    color: #565f89;
    margin: 0;
  }

  @keyframes pulseGlow {
    0% {
      box-shadow: inset 0 0 10px rgba(122, 162, 247, 0.1);
    }
    100% {
      box-shadow: inset 0 0 25px rgba(187, 154, 247, 0.3);
    }
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
