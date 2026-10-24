<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Check, Plus, Trash2, ListTodo, AlertCircle, Clock } from 'lucide-svelte';
  import { islandState } from '$lib/stores/islandState.svelte';

  interface Todo {
    id: string;
    text: string;
    completed: boolean;
    createdAt: number;
  }

  let todos = $state<Todo[]>([]);
  let newTodoText = $state('');
  let filter = $state<'all' | 'active' | 'done'>('all');
  let inputElement: HTMLInputElement | null = null;

  onMount(() => {
    try {
      const stored = localStorage.getItem('arch_island_todos');
      if (stored) {
        todos = JSON.parse(stored);
      }
    } catch (e) {
      console.warn('Failed loading todos:', e);
    }

    // Auto-focus input when tab is opened
    setTimeout(() => {
      inputElement?.focus();
    }, 100);
  });

  onDestroy(() => {
    islandState.hasModalOpen = false;
  });

  function saveTodos() {
    try {
      localStorage.setItem('arch_island_todos', JSON.stringify(todos));
    } catch (e) {
      console.warn('Failed saving todos:', e);
    }
  }

  function addTodo() {
    const text = newTodoText.trim();
    if (!text) return;
    todos = [
      {
        id: crypto.randomUUID(),
        text,
        completed: false,
        createdAt: Date.now(),
      },
      ...todos,
    ];
    newTodoText = '';
    saveTodos();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      addTodo();
    }
  }

  function toggleTodo(id: string) {
    todos = todos.map((t) => (t.id === id ? { ...t, completed: !t.completed } : t));
    saveTodos();
  }

  function deleteTodo(id: string, e: MouseEvent) {
    e.stopPropagation();
    todos = todos.filter((t) => t.id !== id);
    saveTodos();
  }

  function clearCompleted() {
    todos = todos.filter((t) => !t.completed);
    saveTodos();
  }

  function handleFocus() {
    islandState.hasModalOpen = true;
  }

  function handleBlur() {
    islandState.hasModalOpen = false;
  }

  const filteredTodos = $derived(
    todos.filter((t) => {
      if (filter === 'active') return !t.completed;
      if (filter === 'done') return t.completed;
      return true;
    })
  );

  const activeCount = $derived(todos.filter((t) => !t.completed).length);
</script>

<div class="todo-container">
  <!-- Reminder Banner for Pending Tasks -->
  {#if activeCount > 0}
    <div class="reminder-banner">
      <div class="reminder-left">
        <span class="pulsing-amber-dot"></span>
        <AlertCircle size={13} color="#f59e0b" />
        <span class="reminder-text"><strong>Pengingat:</strong> Ada <b>{activeCount}</b> tugas yang belum selesai!</span>
      </div>
    </div>
  {/if}

  <!-- Top Input Form -->
  <form class="input-row" onsubmit={(e) => { e.preventDefault(); addTodo(); }}>
    <div class="input-wrapper">
      <input
        type="text"
        placeholder="Ketik tugas baru lalu tekan Enter..."
        bind:this={inputElement}
        bind:value={newTodoText}
        onkeydown={handleKeydown}
        onfocus={handleFocus}
        onblur={handleBlur}
        class="todo-input"
      />
      <button
        type="button"
        class="add-btn"
        onclick={addTodo}
        disabled={!newTodoText.trim()}
        title="Tambah Task"
      >
        <Plus size={15} />
      </button>
    </div>
  </form>

  <!-- Sub Header / Filters -->
  <div class="filter-bar">
    <div class="filters">
      <button class="filter-btn" class:active={filter === 'all'} onclick={() => (filter = 'all')}>
        Semua ({todos.length})
      </button>
      <button class="filter-btn" class:active={filter === 'active'} onclick={() => (filter = 'active')}>
        Aktif ({activeCount})
      </button>
      <button class="filter-btn" class:active={filter === 'done'} onclick={() => (filter = 'done')}>
        Selesai ({todos.length - activeCount})
      </button>
    </div>

    {#if todos.some((t) => t.completed)}
      <button class="clear-btn" onclick={clearCompleted}>
        Hapus Selesai
      </button>
    {/if}
  </div>

  <!-- Tasks List -->
  <div class="todo-list">
    {#if filteredTodos.length === 0}
      <div class="empty-state">
        <ListTodo size={24} class="empty-icon" />
        <p>
          {filter === 'done'
            ? 'Belum ada tugas yang selesai'
            : filter === 'active'
              ? 'Semua tugas sudah beres! 🎉'
              : 'Belum ada tugas. Tulis sesuatu di atas!'}
        </p>
      </div>
    {:else}
      {#each filteredTodos as todo (todo.id)}
        <div
          class="todo-item"
          class:completed={todo.completed}
          onclick={() => toggleTodo(todo.id)}
          role="button"
          tabindex="0"
          onkeydown={(e) => e.key === ' ' && toggleTodo(todo.id)}
        >
          <div class="check-box" class:checked={todo.completed}>
            {#if todo.completed}
              <Check size={11} strokeWidth={3} />
            {/if}
          </div>

          <div class="todo-body">
            <span class="todo-text">{todo.text}</span>
            {#if !todo.completed}
              <div class="pending-pill">
                <span class="pulsing-amber-dot small"></span>
                <span>Belum Selesai</span>
              </div>
            {/if}
          </div>

          <button
            type="button"
            class="delete-btn"
            onclick={(e) => deleteTodo(todo.id, e)}
            title="Hapus tugas"
          >
            <Trash2 size={13} />
          </button>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .todo-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 10px 14px 14px;
    box-sizing: border-box;
    gap: 8px;
    user-select: none;
  }

  .reminder-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: rgba(245, 158, 11, 0.12);
    border: 1px solid rgba(245, 158, 11, 0.35);
    border-radius: 8px;
    padding: 5px 9px;
    animation: fadeIn 0.2s ease-out;
  }

  .reminder-left {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .reminder-text {
    font-size: 11px;
    color: #fde68a;
  }

  .pulsing-amber-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #f59e0b;
    box-shadow: 0 0 6px #f59e0b;
    animation: pulse 1.8s infinite;
    flex-shrink: 0;
  }

  .pulsing-amber-dot.small {
    width: 5px;
    height: 5px;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.4; transform: scale(0.85); }
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .input-row {
    width: 100%;
    margin: 0;
  }

  .input-wrapper {
    display: flex;
    align-items: center;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 9px;
    padding: 2px 4px 2px 10px;
    transition: all 0.15s ease;
  }

  .input-wrapper:focus-within {
    border-color: #1793d1;
    background: rgba(23, 147, 209, 0.08);
    box-shadow: 0 0 0 2px rgba(23, 147, 209, 0.2);
  }

  .todo-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: #f8fafc;
    font-size: 12px;
    font-family: inherit;
    padding: 7px 0;
  }

  .todo-input::placeholder {
    color: rgba(255, 255, 255, 0.4);
  }

  .add-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    border-radius: 6px;
    background: #1793d1;
    color: #ffffff;
    border: none;
    cursor: pointer;
    transition: opacity 0.15s, transform 0.1s;
  }

  .add-btn:hover:not(:disabled) {
    opacity: 0.95;
    transform: scale(1.05);
  }

  .add-btn:disabled {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.25);
    cursor: not-allowed;
  }

  .filter-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 2px;
    font-size: 11px;
  }

  .filters {
    display: flex;
    gap: 4px;
  }

  .filter-btn {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.45);
    font-size: 10.5px;
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 5px;
    transition: all 0.15s ease;
  }

  .filter-btn:hover {
    color: #ffffff;
    background: rgba(255, 255, 255, 0.06);
  }

  .filter-btn.active {
    color: #1793d1;
    background: rgba(23, 147, 209, 0.15);
    font-weight: 600;
  }

  .clear-btn {
    background: transparent;
    border: none;
    color: rgba(239, 68, 68, 0.7);
    font-size: 10px;
    cursor: pointer;
    padding: 2px 4px;
    transition: color 0.15s;
  }

  .clear-btn:hover {
    color: #ef4444;
  }

  .todo-list {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding-right: 2px;
    min-height: 0;
  }

  .todo-list::-webkit-scrollbar {
    width: 3px;
  }

  .todo-list::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 3px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: rgba(255, 255, 255, 0.35);
    gap: 6px;
    font-size: 11px;
    text-align: center;
    padding: 20px 0;
  }

  :global(.empty-icon) {
    opacity: 0.35;
  }

  .todo-item {
    display: flex;
    align-items: center;
    gap: 10px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.07);
    padding: 8px 10px;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .todo-item:hover {
    background: rgba(255, 255, 255, 0.07);
    border-color: rgba(255, 255, 255, 0.12);
  }

  .check-box {
    width: 16px;
    height: 16px;
    border-radius: 4px;
    border: 1.5px solid rgba(255, 255, 255, 0.35);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    transition: all 0.15s;
    flex-shrink: 0;
  }

  /* SUDAH SELESAI: Centang Hijau */
  .check-box.checked {
    background: #10b981;
    border-color: #10b981;
  }

  .todo-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .todo-text {
    font-size: 12px;
    color: #f1f5f9;
    line-height: 1.35;
    word-break: break-word;
    transition: all 0.2s;
  }

  /* SUDAH SELESAI: Tulisan dicoret dengan WARNA HIJAU */
  .todo-item.completed .todo-text {
    text-decoration: line-through;
    text-decoration-color: #4ade80;
    color: #4ade80;
    opacity: 0.85;
  }

  /* BELUM SELESAI: Badge Pengingat */
  .pending-pill {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    background: rgba(245, 158, 11, 0.12);
    border: 1px solid rgba(245, 158, 11, 0.3);
    color: #f59e0b;
    font-size: 9px;
    font-weight: 600;
    padding: 1px 6px;
    border-radius: 4px;
    width: fit-content;
  }

  .delete-btn {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.25);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s;
  }

  .delete-btn:hover {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.12);
  }
</style>
