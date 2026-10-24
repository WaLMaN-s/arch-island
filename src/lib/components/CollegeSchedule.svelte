<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    Clock,
    MapPin,
    User,
    Calendar,
    Bell,
    Plus,
    Pencil,
    Trash2,
    BookOpen,
    X,
    Check,
    RotateCcw,
    Settings,
  } from 'lucide-svelte';
  import { reminderService, DEFAULT_SCHEDULE, type CollegeClass } from '$lib/services/reminderService';
  import { tauriBridge } from '$lib/services/tauriBridge';
  import { islandState } from '$lib/stores/islandState.svelte';

  const DAYS = ['Senin', 'Selasa', 'Rabu', 'Kamis', 'Jumat', 'Sabtu'] as const;
  type DayType = (typeof DAYS)[number];

  const JS_DAY_MAP: Record<number, DayType> = {
    1: 'Senin',
    2: 'Selasa',
    3: 'Rabu',
    4: 'Kamis',
    5: 'Jumat',
    6: 'Sabtu',
  };

  const currentDayIndex = new Date().getDay();
  const todayDay: DayType = JS_DAY_MAP[currentDayIndex] || 'Senin';

  let selectedDay = $state<DayType>(todayDay);
  let schedule = $state<CollegeClass[]>([]);
  let testSent = $state(false);

  // Form Modal State
  let showForm = $state(false);
  let editingId = $state<string | null>(null);
  let formDay = $state<DayType>('Senin');
  let formTitle = $state('');
  let formTime = $state('');
  let formRoom = $state('');
  let formLecturer = $state('');
  let formJamKe = $state('');

  onMount(() => {
    schedule = reminderService.getSchedule();
  });

  onDestroy(() => {
    islandState.hasModalOpen = false;
  });

  const dayClasses = $derived(schedule.filter((c) => c.day === selectedDay));
  const todayClasses = $derived(schedule.filter((c) => c.day === todayDay));

  function openAddForm() {
    editingId = null;
    formDay = selectedDay;
    formTitle = '';
    formTime = '';
    formRoom = '';
    formLecturer = '';
    formJamKe = '';
    showForm = true;
    islandState.hasModalOpen = true;
  }

  function openEditForm(cls: CollegeClass, e: MouseEvent) {
    e.stopPropagation();
    editingId = cls.id;
    formDay = cls.day;
    formTitle = cls.title;
    formTime = cls.time;
    formRoom = cls.room;
    formLecturer = cls.lecturer;
    formJamKe = cls.jamKe || '';
    showForm = true;
    islandState.hasModalOpen = true;
  }

  function closeForm() {
    showForm = false;
    editingId = null;
    islandState.hasModalOpen = false;
  }

  function saveClass() {
    const title = formTitle.trim();
    if (!title) return;

    if (editingId) {
      schedule = schedule.map((c) =>
        c.id === editingId
          ? {
              ...c,
              day: formDay,
              title,
              time: formTime.trim() || '08:00 - 10:00',
              room: formRoom.trim() || '-',
              lecturer: formLecturer.trim() || '-',
              jamKe: formJamKe.trim() || '-',
            }
          : c
      );
    } else {
      const newCls: CollegeClass = {
        id: crypto.randomUUID(),
        day: formDay,
        title,
        time: formTime.trim() || '08:00 - 10:00',
        room: formRoom.trim() || '-',
        lecturer: formLecturer.trim() || '-',
        jamKe: formJamKe.trim() || '-',
      };
      schedule = [...schedule, newCls];
    }

    reminderService.saveSchedule(schedule);
    selectedDay = formDay;
    closeForm();
  }

  function deleteClass(id: string, e: MouseEvent) {
    e.stopPropagation();
    schedule = schedule.filter((c) => c.id !== id);
    reminderService.saveSchedule(schedule);
  }

  function resetToDefault() {
    schedule = [...DEFAULT_SCHEDULE];
    reminderService.saveSchedule(schedule);
  }

  async function handleTestNotification() {
    const tomorrowIndex = (new Date().getDay() + 1) % 7;
    const tomorrowMap: Record<number, string> = {
      0: 'Minggu',
      1: 'Senin',
      2: 'Selasa',
      3: 'Rabu',
      4: 'Kamis',
      5: 'Jumat',
      6: 'Sabtu',
    };
    const tomorrowDayName = tomorrowMap[tomorrowIndex];
    const tomorrowClasses = schedule.filter((c) => c.day === tomorrowDayName);

    if (tomorrowClasses.length > 0) {
      const classList = tomorrowClasses
        .map((c) => `• ${c.title} (${c.time} @ ${c.room})`)
        .join('\n');

      await tauriBridge.sendDesktopNotification(
        `🎓 Jadwal Kuliah Besok (${tomorrowDayName})`,
        `Ada ${tomorrowClasses.length} mata kuliah:\n${classList}`,
        'appointment-soon'
      );
    } else {
      await tauriBridge.sendDesktopNotification(
        `🎓 Jadwal Kuliah Besok (${tomorrowDayName})`,
        `Besok (${tomorrowDayName}) tidak ada jadwal kuliah (Libur)! 🎉`,
        'appointment-soon'
      );
    }

    testSent = true;
    setTimeout(() => {
      testSent = false;
    }, 3000);
  }
</script>

<div class="schedule-container">
  <!-- Top Bar: Hari Ini Summary, Tambah, Test Notification -->
  <div class="schedule-topbar">
    <div class="today-badge">
      <Calendar size={13} color="#7aa2f7" />
      <span>Hari ini: <strong>{todayDay}</strong> ({todayClasses.length})</span>
    </div>

    <div class="topbar-actions">
      <button
        type="button"
        class="action-btn add"
        onclick={openAddForm}
        title="Tambah atau Ubah Mata Kuliah"
      >
        <Plus size={12} />
        <span>Tambah</span>
      </button>

      <button
        type="button"
        class="action-btn test"
        class:sent={testSent}
        onclick={handleTestNotification}
        title="Uji coba notifikasi malam (H-1) sekarang"
      >
        <Bell size={12} />
        <span>{testSent ? 'Terkirim!' : 'Test Notif'}</span>
      </button>
    </div>
  </div>

  <!-- Day Selector Tabs -->
  <div class="day-tabs">
    {#each DAYS as day}
      <button
        type="button"
        class="day-tab"
        class:active={selectedDay === day}
        class:is-today={todayDay === day}
        onclick={() => (selectedDay = day)}
      >
        <span>{day}</span>
        {#if todayDay === day}
          <span class="today-dot" title="Hari ini"></span>
        {/if}
      </button>
    {/each}
  </div>

  <!-- Classes List -->
  <div class="classes-list">
    {#if dayClasses.length === 0}
      <div class="empty-schedule">
        <BookOpen size={26} class="empty-icon" />
        <p>Tidak ada jadwal kuliah hari {selectedDay}! 🎉</p>
        <button type="button" class="empty-add-btn" onclick={openAddForm}>
          <Plus size={12} />
          <span>Tambah Jadwal untuk {selectedDay}</span>
        </button>
      </div>
    {:else}
      {#each dayClasses as cls (cls.id)}
        <div class="class-card">
          <div class="card-left-bar"></div>

          <div class="card-content">
            <div class="class-header-row">
              <h4 class="class-title">{cls.title}</h4>
              <div class="header-right-meta">
                <div class="time-chip">
                  <Clock size={11} />
                  <span>{cls.time}</span>
                </div>
                <div class="card-actions">
                  <button
                    type="button"
                    class="card-act-btn edit"
                    onclick={(e) => openEditForm(cls, e)}
                    title="Ubah Jadwal"
                  >
                    <Pencil size={11} />
                  </button>
                  <button
                    type="button"
                    class="card-act-btn delete"
                    onclick={(e) => deleteClass(cls.id, e)}
                    title="Hapus Jadwal"
                  >
                    <Trash2 size={11} />
                  </button>
                </div>
              </div>
            </div>

            <div class="meta-row">
              <div class="meta-chip room" title="Ruangan">
                <MapPin size={11} />
                <span>{cls.room}</span>
              </div>

              <div class="meta-chip lecturer" title="Dosen Pengampu">
                <User size={11} />
                <span>{cls.lecturer}</span>
              </div>

              {#if cls.jamKe && cls.jamKe !== '-'}
                <div class="meta-chip jam" title="Jam ke">
                  <span>Jam: {cls.jamKe}</span>
                </div>
              {/if}
            </div>
          </div>
        </div>
      {/each}

      <div class="reset-wrapper">
        <button type="button" class="reset-btn" onclick={resetToDefault} title="Kembalikan ke jadwal bawaan">
          <RotateCcw size={10} />
          <span>Reset ke Jadwal Default</span>
        </button>
      </div>
    {/if}
  </div>

  <!-- Inline Modal: Form Tambah/Ubah Jadwal -->
  {#if showForm}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="form-overlay" onclick={closeForm}>
      <div class="form-card" onclick={(e) => e.stopPropagation()}>
        <div class="form-header">
          <span class="form-title">{editingId ? '✏️ Ubah Jadwal Kuliah' : '➕ Tambah Jadwal Baru'}</span>
          <button type="button" class="form-close" onclick={closeForm}>
            <X size={13} />
          </button>
        </div>

        <form class="form-body" onsubmit={(e) => { e.preventDefault(); saveClass(); }}>
          <div class="form-grid">
            <div class="form-group">
              <label for="hari-select">Hari:</label>
              <select id="hari-select" bind:value={formDay} class="form-input select">
                {#each DAYS as d}
                  <option value={d}>{d}</option>
                {/each}
              </select>
            </div>

            <div class="form-group">
              <label for="jam-input">Jam (misal: 08:30 - 10:30):</label>
              <input
                id="jam-input"
                type="text"
                placeholder="07:30 - 09:30"
                bind:value={formTime}
                class="form-input"
              />
            </div>
          </div>

          <div class="form-group">
            <label for="matkul-input">Nama Mata Kuliah:</label>
            <input
              id="matkul-input"
              type="text"
              placeholder="Contoh: Pemrograman Web Lanjut"
              bind:value={formTitle}
              class="form-input"
              required
            />
          </div>

          <div class="form-grid">
            <div class="form-group">
              <label for="ruang-input">Ruangan / Link:</label>
              <input
                id="ruang-input"
                type="text"
                placeholder="G421 / link"
                bind:value={formRoom}
                class="form-input"
              />
            </div>

            <div class="form-group">
              <label for="jamke-input">Jam Ke (SKS):</label>
              <input
                id="jamke-input"
                type="text"
                placeholder="1\2 atau 2/3/4"
                bind:value={formJamKe}
                class="form-input"
              />
            </div>
          </div>

          <div class="form-group">
            <label for="dosen-input">Dosen Pengampu:</label>
            <input
              id="dosen-input"
              type="text"
              placeholder="Nama Dosen"
              bind:value={formLecturer}
              class="form-input"
            />
          </div>

          <div class="form-footer">
            <button type="button" class="btn-cancel" onclick={closeForm}>Batal</button>
            <button type="submit" class="btn-save" disabled={!formTitle.trim()}>
              <Check size={13} />
              <span>{editingId ? 'Simpan Perubahan' : 'Tambah Jadwal'}</span>
            </button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>

<style>
  .schedule-container {
    position: relative;
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 10px 14px 14px;
    box-sizing: border-box;
    gap: 8px;
    user-select: none;
  }

  .schedule-topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 2px;
  }

  .today-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: #c0caf5;
    background: rgba(122, 162, 247, 0.12);
    border: 1px solid rgba(122, 162, 247, 0.25);
    padding: 3px 8px;
    border-radius: 7px;
  }

  .today-badge strong {
    color: #7aa2f7;
  }

  .topbar-actions {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 10.5px;
    padding: 3px 8px;
    border-radius: 7px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .action-btn.add {
    background: rgba(23, 147, 209, 0.15);
    border: 1px solid rgba(23, 147, 209, 0.35);
    color: #38bdf8;
  }

  .action-btn.add:hover {
    background: #1793d1;
    color: #ffffff;
  }

  .action-btn.test {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #94a3b8;
  }

  .action-btn.test:hover {
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
  }

  .action-btn.test.sent {
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    border-color: rgba(34, 197, 94, 0.35);
  }

  .day-tabs {
    display: flex;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.07);
    border-radius: 9px;
    padding: 2px;
    gap: 2px;
  }

  .day-tab {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    padding: 5px 0;
    background: transparent;
    border: none;
    border-radius: 7px;
    color: rgba(255, 255, 255, 0.5);
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .day-tab:hover {
    color: #ffffff;
    background: rgba(255, 255, 255, 0.06);
  }

  .day-tab.active {
    background: #1793d1;
    color: #ffffff;
    box-shadow: 0 2px 6px rgba(23, 147, 209, 0.35);
  }

  .day-tab.is-today:not(.active) {
    color: #7aa2f7;
  }

  .today-dot {
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: #7aa2f7;
    box-shadow: 0 0 5px #7aa2f7;
  }

  .classes-list {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 7px;
    padding-right: 2px;
    min-height: 0;
  }

  .classes-list::-webkit-scrollbar {
    width: 3px;
  }

  .classes-list::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 3px;
  }

  .empty-schedule {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: rgba(255, 255, 255, 0.4);
    gap: 8px;
    font-size: 11.5px;
    text-align: center;
    padding: 25px 0;
  }

  :global(.empty-icon) {
    opacity: 0.35;
  }

  .empty-add-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    background: rgba(23, 147, 209, 0.12);
    border: 1px solid rgba(23, 147, 209, 0.3);
    color: #38bdf8;
    font-size: 11px;
    padding: 5px 10px;
    border-radius: 7px;
    cursor: pointer;
    transition: all 0.15s;
  }

  .empty-add-btn:hover {
    background: #1793d1;
    color: #fff;
  }

  .class-card {
    display: flex;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 10px;
    overflow: hidden;
    transition: all 0.15s ease;
  }

  .class-card:hover {
    background: rgba(255, 255, 255, 0.07);
    border-color: rgba(23, 147, 209, 0.4);
    transform: translateY(-1px);
  }

  .card-left-bar {
    width: 4px;
    background: #1793d1;
    flex-shrink: 0;
  }

  .card-content {
    flex: 1;
    padding: 8px 10px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    min-width: 0;
  }

  .class-header-row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 8px;
  }

  .class-title {
    margin: 0;
    font-size: 12px;
    font-weight: 700;
    color: #f8fafc;
    line-height: 1.3;
    flex: 1;
  }

  .header-right-meta {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .time-chip {
    display: flex;
    align-items: center;
    gap: 4px;
    background: rgba(23, 147, 209, 0.15);
    border: 1px solid rgba(23, 147, 209, 0.3);
    color: #38bdf8;
    font-size: 10.5px;
    font-weight: 600;
    padding: 2px 6px;
    border-radius: 6px;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .card-actions {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .card-act-btn {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.3);
    padding: 3px;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s;
  }

  .card-act-btn.edit:hover {
    color: #38bdf8;
    background: rgba(23, 147, 209, 0.15);
  }

  .card-act-btn.delete:hover {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.15);
  }

  .meta-row {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 5px;
  }

  .meta-chip {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 5px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.06);
    color: #94a3b8;
  }

  .meta-chip.room {
    background: rgba(168, 85, 247, 0.12);
    border-color: rgba(168, 85, 247, 0.25);
    color: #c084fc;
    font-weight: 600;
  }

  .meta-chip.lecturer {
    color: #cbd5e1;
  }

  .meta-chip.jam {
    color: rgba(255, 255, 255, 0.45);
  }

  .reset-wrapper {
    display: flex;
    justify-content: center;
    padding: 6px 0 2px;
  }

  .reset-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.3);
    font-size: 10px;
    cursor: pointer;
    transition: color 0.15s;
  }

  .reset-btn:hover {
    color: rgba(255, 255, 255, 0.7);
  }

  /* Form Modal */
  .form-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 12px;
    z-index: 50;
    border-radius: 20px;
    animation: fadeIn 0.15s ease-out;
  }

  .form-card {
    width: 100%;
    max-width: 380px;
    background: rgba(20, 23, 34, 0.98);
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 14px;
    padding: 12px;
    box-shadow: 0 16px 36px rgba(0, 0, 0, 0.8);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .form-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    padding-bottom: 6px;
  }

  .form-title {
    font-size: 12px;
    font-weight: 700;
    color: #f8fafc;
  }

  .form-close {
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    padding: 2px;
    border-radius: 4px;
  }

  .form-close:hover {
    color: #fff;
  }

  .form-body {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .form-group label {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.6);
  }

  .form-input {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 7px;
    color: #f8fafc;
    font-size: 11px;
    padding: 5px 8px;
    outline: none;
    font-family: inherit;
    transition: all 0.15s;
  }

  .form-input:focus {
    border-color: #1793d1;
    background: rgba(23, 147, 209, 0.08);
  }

  .form-input.select {
    cursor: pointer;
  }

  .form-input.select option {
    background: #1a1e2e;
    color: #fff;
  }

  .form-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 6px;
    margin-top: 4px;
  }

  .btn-cancel {
    background: transparent;
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: #94a3b8;
    font-size: 11px;
    padding: 5px 10px;
    border-radius: 7px;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-cancel:hover {
    background: rgba(255, 255, 255, 0.06);
    color: #fff;
  }

  .btn-save {
    display: flex;
    align-items: center;
    gap: 4px;
    background: #1793d1;
    border: none;
    color: #fff;
    font-size: 11px;
    font-weight: 600;
    padding: 5px 12px;
    border-radius: 7px;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-save:hover:not(:disabled) {
    background: #0284c7;
    transform: scale(1.02);
  }

  .btn-save:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
