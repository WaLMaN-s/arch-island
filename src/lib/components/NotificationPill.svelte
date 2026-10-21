<script lang="ts">
  import type { NotificationPayload } from '$lib/types';
  import { islandState } from '$lib/stores/islandState.svelte';
  import { Bell, X } from 'lucide-svelte';

  let { notification }: { notification: NotificationPayload } = $props();

  function dismiss(e: MouseEvent) {
    e.stopPropagation();
    islandState.dismissNotification();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="notification-pill" onclick={dismiss}>
  <div class="notif-icon-badge">
    <Bell size={13} color="#bb9af7" />
  </div>

  <div class="notif-content">
    <div class="notif-header">
      <span class="app-name">{notification.app_name}</span>
      <span class="summary">{notification.summary}</span>
    </div>
    {#if notification.body}
      <p class="body-text">{notification.body}</p>
    {/if}
  </div>

  <button class="dismiss-btn" onclick={dismiss} title="Dismiss">
    <X size={12} color="#565f89" />
  </button>
</div>

<style>
  .notification-pill {
    display: flex;
    align-items: center;
    width: 100%;
    gap: 8px;
    padding: 0 4px;
    animation: fadeIn 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .notif-icon-badge {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: rgba(187, 154, 247, 0.15);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .notif-content {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .notif-header {
    display: flex;
    align-items: center;
    gap: 6px;
    overflow: hidden;
    white-space: nowrap;
  }

  .app-name {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: #bb9af7;
  }

  .summary {
    font-size: 11px;
    font-weight: 600;
    color: #c0caf5;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .body-text {
    font-size: 10px;
    color: #a9b1d6;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    opacity: 0.85;
  }

  .dismiss-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    transition: background 0.2s ease;
  }

  .dismiss-btn:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-4px) scale(0.96);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
