import type { HardwareEventPayload, IslandMode, IslandSubView, NotificationPayload } from '$lib/types';
import { tauriBridge } from '$lib/services/tauriBridge';

class IslandStateStore {
  mode = $state<IslandMode>('tucked');
  subView = $state<IslandSubView>('auto');
  activeNotification = $state<NotificationPayload | null>(null);
  activeHardwareEvent = $state<HardwareEventPayload | null>(null);
  hudType = $state<'volume' | 'brightness' | null>(null);
  hudValue = $state<number>(0);
  isHovered = $state(false);
  isRetracting = $state(false);
  isLyricsOpen = $state(false);
  hasModalOpen = $state(false);

  private notifTimer: number | null = null;
  private hwTimer: number | null = null;
  private hudTimer: number | null = null;
  private leaveDebounceTimer: number | null = null;

  async init() {
    // Listen for system notifications
    tauriBridge.listenNotification((payload) => {
      this.triggerNotification(payload);
    });

    // Listen for hardware plug/unplug events (Charger, USB, Headphones)
    tauriBridge.listenHardwareEvent((payload) => {
      this.triggerHardwareEvent(payload);
    });
  }

  // Hover triggered by mouse reaching island area
  async onMouseEnter() {
    if (this.leaveDebounceTimer) {
      clearTimeout(this.leaveDebounceTimer);
      this.leaveDebounceTimer = null;
    }
    this.isRetracting = false;
    this.isHovered = true;

    if (this.mode === 'tucked') {
      this.mode = 'compact';
      await tauriBridge.resizeIsland('compact');
    }
  }

  // Mouse left the island area
  async onMouseLeave() {
    this.isHovered = false;

    if (this.leaveDebounceTimer) {
      clearTimeout(this.leaveDebounceTimer);
      this.leaveDebounceTimer = null;
    }

    // In expanded mode (Control Center or Music Card):
    // Automatically close back to compact/tucked when cursor leaves!
    // EXCEPTION: In Lyrics mode (isLyricsOpen), DO NOT auto-close per user requirement!
    // EXCEPTION: While a modal is open (hasModalOpen), DO NOT auto-close!
    if (this.mode === 'expanded') {
      if (this.isLyricsOpen || this.hasModalOpen) {
        return;
      }

      this.leaveDebounceTimer = window.setTimeout(async () => {
        if (!this.isHovered && this.mode === 'expanded' && !this.isLyricsOpen && !this.hasModalOpen) {
          await this.collapse();
        }
      }, 280);
      return;
    }

    // In compact mode: smoothly slide back up to tucked notch
    if (this.mode === 'compact' && !this.activeNotification && !this.activeHardwareEvent && !this.hudType) {
      this.leaveDebounceTimer = window.setTimeout(async () => {
        if (!this.isHovered && this.mode === 'compact') {
          this.isRetracting = true;
          // Smooth slide-up animation duration
          await new Promise((resolve) => setTimeout(resolve, 380));
          if (!this.isHovered && this.mode === 'compact') {
            this.mode = 'tucked';
            this.isRetracting = false;
            await tauriBridge.resizeIsland('tucked');
          }
        }
      }, 350);
    }
  }

  async expand(subView: IslandSubView = 'auto') {
    if (this.leaveDebounceTimer) {
      clearTimeout(this.leaveDebounceTimer);
      this.leaveDebounceTimer = null;
    }
    this.mode = 'expanded';
    this.isRetracting = false;
    this.subView = subView;
    await tauriBridge.resizeIsland('expanded');
  }

  async collapse() {
    if (this.leaveDebounceTimer) {
      clearTimeout(this.leaveDebounceTimer);
      this.leaveDebounceTimer = null;
    }
    this.subView = 'auto';
    this.hudType = null;
    this.isLyricsOpen = false;
    this.hasModalOpen = false;
    this.mode = 'compact';
    this.isRetracting = false;
    await tauriBridge.resizeIsland('compact');

    if (!this.isHovered) {
      this.leaveDebounceTimer = window.setTimeout(async () => {
        if (!this.isHovered && this.mode === 'compact') {
          this.isRetracting = true;
          await new Promise((resolve) => setTimeout(resolve, 380));
          if (!this.isHovered && this.mode === 'compact') {
            this.mode = 'tucked';
            this.isRetracting = false;
            await tauriBridge.resizeIsland('tucked');
          }
        }
      }, 350);
    }
  }

  async toggle() {
    if (this.mode === 'expanded') {
      await this.collapse();
    } else {
      await this.expand();
    }
  }

  async triggerNotification(payload: NotificationPayload) {
    if (this.notifTimer) clearTimeout(this.notifTimer);
    this.activeHardwareEvent = null;
    this.activeNotification = payload;

    tauriBridge.dismissSystemNotifications();

    if (this.mode === 'tucked') {
      this.mode = 'compact';
      await tauriBridge.resizeIsland('compact', 440, 58);
    } else if (this.mode === 'compact') {
      await tauriBridge.resizeIsland('compact', 440, 58);
    }

    this.notifTimer = window.setTimeout(async () => {
      this.activeNotification = null;
      if (!this.isHovered && this.mode === 'compact') {
        this.mode = 'tucked';
        await tauriBridge.resizeIsland('tucked');
      }
    }, 4500);
  }

  async triggerHardwareEvent(payload: HardwareEventPayload) {
    if (this.hwTimer) clearTimeout(this.hwTimer);
    this.activeNotification = null;
    this.activeHardwareEvent = payload;

    if (this.mode === 'tucked') {
      this.mode = 'compact';
      await tauriBridge.resizeIsland('compact');
    }

    this.hwTimer = window.setTimeout(async () => {
      this.activeHardwareEvent = null;
      if (!this.isHovered && this.mode === 'compact') {
        this.mode = 'tucked';
        await tauriBridge.resizeIsland('tucked');
      }
    }, 2800);
  }

  dismissNotification() {
    this.activeNotification = null;
    this.activeHardwareEvent = null;
    if (this.notifTimer) clearTimeout(this.notifTimer);
    if (this.hwTimer) clearTimeout(this.hwTimer);
    if (!this.isHovered && this.mode === 'compact') {
      this.mode = 'tucked';
      tauriBridge.resizeIsland('tucked');
    }
  }

  async showHud(type: 'volume' | 'brightness', value: number) {
    this.hudType = type;
    this.hudValue = value;
    if (this.hudTimer) clearTimeout(this.hudTimer);

    if (this.mode === 'tucked') {
      this.mode = 'compact';
      await tauriBridge.resizeIsland('compact');
    }

    this.hudTimer = window.setTimeout(async () => {
      this.hudType = null;
      if (!this.isHovered && this.mode === 'compact') {
        this.mode = 'tucked';
        await tauriBridge.resizeIsland('tucked');
      }
    }, 2200);
  }
}

export const islandState = new IslandStateStore();
