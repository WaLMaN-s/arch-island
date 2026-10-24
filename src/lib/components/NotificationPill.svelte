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
    <Bell size={13} color="#38bdf8" />
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
    <X size={13} color="#e2e8f0" />
  </button>
</div>

<style>
  .notification-pill {
    display: flex;
    align-items: center;
    width: 100%;
    gap: 8px;
    padding: 2px 8px;
    background: linear-gradient(90deg, rgba(23, 147, 209, 0.22) 0%, rgba(14, 165, 233, 0.12) 100%);
    border: 1px solid rgba(56, 189, 248, 0.45);
    border-radius: 99px;
    box-shadow: 0 0 16px rgba(23, 147, 209, 0.35);
    box-sizing: border-box;
    animation: fadeIn 0.25s cubic-bezier(0.16, 1, 0.3, 1);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .notification-pill:hover {
    background: linear-gradient(90deg, rgba(23, 147, 209, 0.3) 0%, rgba(14, 165, 233, 0.2) 100%);
    border-color: rgba(56, 189, 248, 0.7);
  }

  .notif-icon-badge {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    background: rgba(56, 189, 248, 0.25);
    border: 1px solid rgba(56, 189, 248, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    box-shadow: 0 0 8px rgba(56, 189, 248, 0.4);
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
    font-size: 9.5px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: #38bdf8;
    background: rgba(56, 189, 248, 0.18);
    border: 1px solid rgba(56, 189, 248, 0.35);
    padding: 1px 5px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .summary {
    font-size: 11.5px;
    font-weight: 700;
    color: #ffffff;
    overflow: hidden;
    text-overflow: ellipsis;
    text-shadow: 0 1px 3px rgba(0, 0, 0, 0.6);
  }

  .body-text {
    font-size: 10.5px;
    font-weight: 500;
    color: #f1f5f9;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    opacity: 0.95;
  }

  .dismiss-btn {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.15);
    cursor: pointer;
    padding: 3px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    transition: all 0.15s ease;
  }

  .dismiss-btn:hover {
    background: rgba(239, 68, 68, 0.3);
    border-color: rgba(239, 68, 68, 0.5);
    color: #ef4444;
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
