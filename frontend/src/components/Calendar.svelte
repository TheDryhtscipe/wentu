<script>
  import { createEventDispatcher } from 'svelte';
  import { ChevronLeft, ChevronRight } from 'lucide-svelte';

  const dispatch = createEventDispatcher();

  export let year = new Date().getFullYear();
  export let month = new Date().getMonth();

  let startDate = null;
  let endDate = null;
  let hoveredDate = null;
  let isDragging = false;
  let dragStartDay = null;

  const monthNames = [
    'January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December'
  ];

  const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

  $: firstDay = new Date(year, month, 1);
  $: lastDay = new Date(year, month + 1, 0);
  $: daysInMonth = lastDay.getDate();
  $: startingDayOfWeek = firstDay.getDay();

  $: days = Array.from({ length: daysInMonth }, (_, i) => i + 1);
  $: paddingDays = Array.from({ length: startingDayOfWeek }, () => null);

  function getDayKey(day) {
    const d = new Date(year, month, day);
    const y = d.getFullYear();
    const m = String(d.getMonth() + 1).padStart(2, '0');
    const da = String(d.getDate()).padStart(2, '0');
    return `${y}-${m}-${da}`;
  }

  function parseKey(key) {
    const [y, m, d] = key.split('-').map(Number);
    return new Date(y, m - 1, d);
  }

  function dateFromKey(key) {
    return parseKey(key);
  }

  function isInRange(day) {
    if (!startDate) return false;
    const checkEnd = endDate || hoveredDate;
    if (!checkEnd) return false;
    
    const date = new Date(year, month, day);
    const start = parseKey(startDate);
    const end = parseKey(checkEnd);
    const minDate = start < end ? start : end;
    const maxDate = start < end ? end : start;
    
    return date > minDate && date < maxDate;
  }

  function isSelected(day) {
    const key = getDayKey(day);
    return startDate === key || endDate === key;
  }

  function handleMouseDown(day) {
    isDragging = true;
    dragStartDay = day;
    const key = getDayKey(day);
    startDate = key;
    endDate = null;
    hoveredDate = key;
  }

  function handleMouseEnter(day) {
    if (isDragging && dragStartDay !== null) {
      const currentKey = getDayKey(day);
      hoveredDate = currentKey;
      endDate = currentKey;
    } else if (!isDragging) {
      hoveredDate = getDayKey(day);
    }
  }

  function handleMouseLeave() {
    if (!isDragging) {
      hoveredDate = null;
    }
  }

  function handleMouseUp(day) {
    if (!isDragging) return;
    isDragging = false;
    const key = getDayKey(day);
    endDate = key;
    dragStartDay = null;

    // Emit the event with properly ordered dates
    const start = parseKey(startDate);
    const end = parseKey(endDate);
    dispatch('daterange', {
      start: start < end ? start : end,
      end: start < end ? end : start
    });
  }

  function handleClick(day) {
    if (isDragging) return;

    const key = getDayKey(day);

    if (!startDate) {
      startDate = key;
      endDate = null;
      hoveredDate = key;
    } else if (!endDate) {
      endDate = key;
      
      // Emit the event with properly ordered dates
      const start = new Date(startDate);
      const end = new Date(endDate);
      dispatch('daterange', {
        start: start < end ? start : end,
        end: start < end ? end : start
      });
    } else {
      // Reset for new selection
      startDate = key;
      endDate = null;
      hoveredDate = key;
    }
  }

  function previousMonth() {
    if (month === 0) {
      month = 11;
      year--;
    } else {
      month--;
    }
  }

  function nextMonth() {
    if (month === 11) {
      month = 0;
      year++;
    } else {
      month++;
    }
  }

  function reset() {
    startDate = null;
    endDate = null;
    hoveredDate = null;
    isDragging = false;
  }
</script>

<div class="calendar-wrapper">
  <div class="calendar-header">
    <button on:click={previousMonth} class="btn-secondary px-3 py-1">
      <ChevronLeft size={18} />
    </button>
    <h2 class="text-text-primary font-medium">{monthNames[month]} {year}</h2>
    <button on:click={nextMonth} class="btn-secondary px-3 py-1">
      <ChevronRight size={18} />
    </button>
  </div>

  <div class="calendar-grid" on:mouseup={() => isDragging = false} on:mouseleave={() => isDragging && (isDragging = false)}>
    {#each dayNames as dayName}
      <div class="day-header">{dayName}</div>
    {/each}

    {#each paddingDays as _}
      <div class="day-cell empty"></div>
    {/each}

    {#each days as day (day)}
      {@const key = getDayKey(day)}
      <button
        on:mousedown={() => handleMouseDown(day)}
        on:mouseenter={() => handleMouseEnter(day)}
        on:mouseleave={handleMouseLeave}
        on:mouseup={() => handleMouseUp(day)}
        on:click={() => handleClick(day)}
        class="day-cell"
        class:in-range={isInRange(day)}
        class:selected={isSelected(day)}
        class:drag-endpoint={isDragging && hoveredDate === key}
        aria-label="Select {monthNames[month]} {day}"
      >
        {day}
      </button>
    {/each}
  </div>

  <div class="calendar-footer">
    {#if startDate}
      <p class="text-text-secondary text-sm">
        {#if endDate}
          Selected: {parseKey(startDate).toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })} to {parseKey(endDate).toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })}
        {:else}
          Start: {parseKey(startDate).toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })}
        {/if}
      </p>
    {/if}
    {#if startDate || endDate}
      <button on:click={reset} class="btn-secondary px-3 py-1 text-sm">Clear</button>
    {/if}
  </div>

</div>

<style>
  .calendar-wrapper {
    background: #0f0f14; /* bright-snow-950 */
    border: 1px solid rgba(255, 156, 99, 0.2);
    border-radius: 0.5rem;
    padding: 1.5rem;
  }

  .calendar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    gap: 1rem;
  }

  .calendar-header h2 {
    flex: 1;
    text-align: center;
    margin: 0;
  }

  .calendar-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    user-select: none;
  }

  .day-header {
    text-align: center;
    font-weight: 600;
    padding: 0.5rem;
    color: #55aa99; /* pine-teal-500 (6.91:1 contrast) */
    font-size: 0.875rem;
  }

  .day-cell {
    aspect-ratio: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid rgba(255, 156, 99, 0.1);
    border-radius: 0.375rem;
    background: transparent;
    color: #77bbad; /* pine-teal-400 (8.64:1 contrast) */
    cursor: pointer;
    font-size: 0.875rem;
    transition: all 0.15s ease;
    font-weight: 500;
    padding: 0;
  }

  .day-cell.empty {
    cursor: default;
    pointer-events: none;
  }

  .day-cell:not(.empty):hover {
    border-color: var(--accent, #ff9c63);
  }

  .day-cell.in-range {
    background: rgba(255, 156, 99, 0.25);
    border-color: rgba(255, 156, 99, 0.4);
  }

  .day-cell.drag-endpoint {
    background: rgba(255, 156, 99, 0.4);
    border-color: var(--accent, #ff9c63);
  }

  .day-cell.selected {
    background: var(--accent, #ff9c63);
    color: var(--dark-bg, #2b1313);
    border-color: var(--accent, #ff9c63);
    font-weight: 700;
  }

  .calendar-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  .calendar-footer p {
    margin: 0;
  }
</style>
