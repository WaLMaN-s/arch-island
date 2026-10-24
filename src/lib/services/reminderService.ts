import { tauriBridge } from '$lib/services/tauriBridge';

export interface CollegeClass {
  id: string;
  day: 'Senin' | 'Selasa' | 'Rabu' | 'Kamis' | 'Jumat' | 'Sabtu';
  title: string;
  time: string;
  jamKe: string;
  room: string;
  lecturer: string;
}

export const DEFAULT_SCHEDULE: CollegeClass[] = [
  {
    id: '1',
    day: 'Senin',
    title: 'Praktikum Robotika Cerdas',
    time: '07:30 - 09:30',
    jamKe: '1\\2',
    room: 'link',
    lecturer: 'TIM DOSEN',
  },
  {
    id: '2',
    day: 'Selasa',
    title: 'Pengelolaan Proyek Sistem Informasi',
    time: '08:30 - 12:30',
    jamKe: '2/3/4/5',
    room: 'G421',
    lecturer: 'WIDYA SILFIANTI',
  },
  {
    id: '3',
    day: 'Selasa',
    title: 'Sistem Penunjang Keputusan *',
    time: '13:30 - 15:30',
    jamKe: '7\\8',
    room: 'G342',
    lecturer: 'JALINAS/IRFAN',
  },
  {
    id: '4',
    day: 'Rabu',
    title: 'Business Intelligence',
    time: '09:30 - 11:30',
    jamKe: '3\\4',
    room: 'G143',
    lecturer: 'HAMUNGKASI',
  },
  {
    id: '5',
    day: 'Rabu',
    title: 'Sistem Multimedia',
    time: '11:30 - 13:30',
    jamKe: '5\\6',
    room: 'G143',
    lecturer: 'ROBBY CANDRA',
  },
  {
    id: '6',
    day: 'Kamis',
    title: 'Robotika Cerdas',
    time: 'Fleksibel / Async',
    jamKe: '-',
    room: 'UGTV',
    lecturer: 'TEAM TEACHING',
  },
  {
    id: '7',
    day: 'Sabtu',
    title: 'Sistem Terdistribusi',
    time: '07:30 - 09:30',
    jamKe: '1\\2',
    room: 'E246',
    lecturer: 'DELTA AGUS WARDIANTO',
  },
  {
    id: '8',
    day: 'Sabtu',
    title: 'Testing dan Implementasi Sistem *',
    time: '09:30 - 12:30',
    jamKe: '3\\4\\5',
    room: 'E246',
    lecturer: 'SYAHRIZAL ANDHIKA',
  },
];

const DAYS_ORDER: ('Minggu' | 'Senin' | 'Selasa' | 'Rabu' | 'Kamis' | 'Jumat' | 'Sabtu')[] = [
  'Minggu',
  'Senin',
  'Selasa',
  'Rabu',
  'Kamis',
  'Jumat',
  'Sabtu',
];

class ReminderService {
  private timer: number | null = null;
  private lastTaskNotifTime: number = 0;
  private lastScheduleNotifDate: string = '';

  init() {
    if (typeof window === 'undefined') return;

    // Load last notification timestamps from localStorage
    try {
      this.lastTaskNotifTime = Number(localStorage.getItem('last_task_notif_time') || 0);
      this.lastScheduleNotifDate = localStorage.getItem('last_schedule_notif_date') || '';
    } catch {}

    // Check every 30 seconds
    this.timer = window.setInterval(() => {
      this.checkTaskReminder();
      this.checkNightScheduleReminder();
    }, 30_000);

    // Initial check after short startup delay
    setTimeout(() => {
      this.checkTaskReminder();
      this.checkNightScheduleReminder();
    }, 5_000);
  }

  destroy() {
    if (this.timer) {
      clearInterval(this.timer);
      this.timer = null;
    }
  }

  // 1. Task reminder: every 1 hour (3600 seconds) if tasks are pending
  private checkTaskReminder() {
    const now = Date.now();
    const oneHourMs = 60 * 60 * 1000;

    if (now - this.lastTaskNotifTime < oneHourMs) {
      return;
    }

    try {
      const stored = localStorage.getItem('arch_island_todos');
      if (!stored) return;
      const todos = JSON.parse(stored);
      const pendingCount = todos.filter((t: any) => !t.completed).length;

      if (pendingCount > 0) {
        tauriBridge.sendDesktopNotification(
          '📌 Pengingat Tugas (Task)',
          `Ada ${pendingCount} tugas yang belum selesai! Cek tab Tasks di notch.`,
          'dialog-information'
        );
        this.lastTaskNotifTime = now;
        localStorage.setItem('last_task_notif_time', String(now));
      }
    } catch (e) {
      console.warn('checkTaskReminder error:', e);
    }
  }

  // 2. Night before reminder: check at 20:00 (8:00 PM) for tomorrow's classes
  private checkNightScheduleReminder() {
    const now = new Date();
    const hour = now.getHours();
    const todayStr = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`;

    // Trigger after 20:00 (8:00 PM) once per day
    if (hour >= 20 && this.lastScheduleNotifDate !== todayStr) {
      const tomorrowIndex = (now.getDay() + 1) % 7;
      const tomorrowDayName = DAYS_ORDER[tomorrowIndex];

      // Get tomorrow's classes
      const schedule = this.getSchedule();
      const tomorrowClasses = schedule.filter((c) => c.day === tomorrowDayName);

      if (tomorrowClasses.length > 0) {
        const classList = tomorrowClasses
          .map((c) => `• ${c.title} (${c.time} @ ${c.room})`)
          .join('\n');

        tauriBridge.sendDesktopNotification(
          `🎓 Jadwal Kuliah Besok (${tomorrowDayName})`,
          `Ada ${tomorrowClasses.length} mata kuliah:\n${classList}`,
          'appointment-soon'
        );
      }

      this.lastScheduleNotifDate = todayStr;
      try {
        localStorage.setItem('last_schedule_notif_date', todayStr);
      } catch {}
    }
  }

  getSchedule(): CollegeClass[] {
    try {
      const stored = localStorage.getItem('arch_island_schedule');
      if (stored) {
        return JSON.parse(stored);
      }
    } catch {}
    return DEFAULT_SCHEDULE;
  }

  saveSchedule(schedule: CollegeClass[]) {
    try {
      localStorage.setItem('arch_island_schedule', JSON.stringify(schedule));
    } catch {}
  }
}

export const reminderService = new ReminderService();
