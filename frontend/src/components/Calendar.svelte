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
  let isMouseDown = false;
  let dragStartDay = null;
  let ignoreClick = false;
  let startedWithMouseDown = false;
  let touchStartDay = null;
  let touchMoved = false;
  let touchStartedSelection = false;

  // Touch event state management
  let interactionType = null;  // 'mouse' | 'touch' | null
  let touchIdentifier = null;  // Track active touch

  const monthNames = [
    'January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December'
  ];

  const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  const dayNamesMobile = ['S', 'M', 'T', 'W', 'T', 'F', 'S'];

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
    // Don't start mouse interaction if touch is active
    if (interactionType === 'touch') return;

    interactionType = 'mouse';
    isMouseDown = true;
    isDragging = false;
    dragStartDay = day;
    const key = getDayKey(day);
    if (!startDate || endDate) {
      startDate = key;
      endDate = null;
      startedWithMouseDown = true;
    } else {
      startedWithMouseDown = false;
    }
    hoveredDate = key;
  }

  function handleMouseEnter(day) {
    // Only respond to mouse hover if in mouse mode
    if (interactionType === 'touch') return;

    if (isMouseDown && dragStartDay !== null) {
      const currentKey = getDayKey(day);
      hoveredDate = currentKey;
      endDate = currentKey;
      if (day !== dragStartDay) {
        isDragging = true;
      }
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
    if (interactionType !== 'mouse' || !isMouseDown) return;

    const key = getDayKey(day);
    if (isDragging) {
      endDate = key;
      ignoreClick = true;
      startedWithMouseDown = false;

      // Emit the event with properly ordered dates
      const start = parseKey(startDate);
      const end = parseKey(endDate);
      dispatch('daterange', {
        start: start < end ? start : end,
        end: start < end ? end : start
      });
    }

    isMouseDown = false;
    isDragging = false;
    interactionType = null;
    dragStartDay = null;
  }

  function applyClickSelection(key) {
    if (!startDate) {
      startDate = key;
      endDate = null;
      hoveredDate = key;
      return;
    }

    if (!endDate) {
      endDate = key;

      // Emit the event with properly ordered dates
      const start = new Date(startDate);
      const end = new Date(endDate);
      dispatch('daterange', {
        start: start < end ? start : end,
        end: start < end ? end : start
      });
      return;
    }

    // Reset for new selection
    startDate = key;
    endDate = null;
    hoveredDate = key;
  }

  function handleClick(day) {
    // Don't handle click if we just finished dragging
    if (ignoreClick) {
      ignoreClick = false;
      return;
    }

    if (startedWithMouseDown) {
      startedWithMouseDown = false;
      return;
    }

    // Don't interfere with ongoing interactions
    if (interactionType !== null) return;

    applyClickSelection(getDayKey(day));
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
    isMouseDown = false;
    ignoreClick = false;
    startedWithMouseDown = false;
    touchStartDay = null;
    touchMoved = false;
    touchStartedSelection = false;
    interactionType = null;
    touchIdentifier = null;
  }

  // Touch event handlers
  function handleTouchStart(event, day) {
    // Prevent scrolling and zooming
    event.preventDefault();

    // Only handle single touch
    if (event.touches.length !== 1) return;

    // If already in mouse interaction, ignore touch
    if (interactionType === 'mouse') return;

    interactionType = 'touch';
    touchIdentifier = event.touches[0].identifier;
    isDragging = true;
    dragStartDay = day;
    touchStartDay = day;
    touchMoved = false;

    const key = getDayKey(day);
    if (!startDate || endDate) {
      startDate = key;
      endDate = null;
      touchStartedSelection = true;
    } else {
      touchStartedSelection = false;
    }
    hoveredDate = key;
  }

  function handleTouchMove(event) {
    event.preventDefault();

    if (!isDragging || interactionType !== 'touch') return;

    // Find which touch is ours
    const touch = Array.from(event.touches).find(
      t => t.identifier === touchIdentifier
    );

    if (!touch) return;

    // Find element under touch point
    const element = document.elementFromPoint(touch.clientX, touch.clientY);
    if (!element) return;

    // Check if element is a day cell with data-day attribute
    const dayCell = element.closest('[data-day]');
    if (!dayCell) return;

    const day = parseInt(dayCell.dataset.day);
    if (isNaN(day)) return;

    touchMoved = true;
    const currentKey = getDayKey(day);
    hoveredDate = currentKey;
    endDate = currentKey;
  }

  function handleTouchEnd(event) {
    event.preventDefault();

    if (!isDragging || interactionType !== 'touch') return;

    // Find the touch that ended
    const touch = Array.from(event.changedTouches).find(
      t => t.identifier === touchIdentifier
    );

    if (!touch) return;

    if (touchMoved && startDate && endDate) {
      const start = parseKey(startDate);
      const end = parseKey(endDate);
      dispatch('daterange', {
        start: start < end ? start : end,
        end: start < end ? end : start
      });
    } else if (!touchMoved && touchStartDay !== null && !touchStartedSelection) {
      applyClickSelection(getDayKey(touchStartDay));
    }

    isDragging = false;
    interactionType = null;
    touchIdentifier = null;
    dragStartDay = null;
    touchStartDay = null;
    touchMoved = false;
    touchStartedSelection = false;
  }

  function handleTouchCancel(event) {
    // Reset state if touch is cancelled
    if (interactionType === 'touch') {
      isDragging = false;
      isMouseDown = false;
      interactionType = null;
      touchIdentifier = null;
      dragStartDay = null;
      touchStartDay = null;
      touchMoved = false;
      touchStartedSelection = false;
    }
  }

  function cancelMouseDrag() {
    if (interactionType !== 'mouse') return;
    isDragging = false;
    isMouseDown = false;
    interactionType = null;
    dragStartDay = null;
    startedWithMouseDown = false;
  }
</script>

<div class="calendar-wrapper">
  <div class="calendar-header">
    <button type="button" on:click={previousMonth} class="btn-secondary px-3 py-1" aria-label="Previous month">
      <ChevronLeft size={18} />
    </button>
    <h2 class="text-text-primary font-medium">{monthNames[month]} {year}</h2>
    <button type="button" on:click={nextMonth} class="btn-secondary px-3 py-1" aria-label="Next month">
      <ChevronRight size={18} />
    </button>
  </div>

  <div
    class="calendar-grid"
    on:mouseup={cancelMouseDrag}
    on:mouseleave={cancelMouseDrag}
    on:touchmove={handleTouchMove}
    on:touchend={handleTouchEnd}
    on:touchcancel={handleTouchCancel}
  >
    {#each dayNames as dayName, i}
      <div class="day-header">
        <span class="day-header-mobile">{dayNamesMobile[i]}</span>
        <span class="day-header-desktop">{dayName}</span>
      </div>
    {/each}

    {#each paddingDays as _}
      <div class="day-cell empty"></div>
    {/each}

    {#each days as day (day)}
      {@const key = getDayKey(day)}
      <button
        data-day={day}
        on:mousedown={() => handleMouseDown(day)}
        on:mouseenter={() => handleMouseEnter(day)}
        on:mouseleave={handleMouseLeave}
        on:mouseup={() => handleMouseUp(day)}
        on:click={() => handleClick(day)}
        on:touchstart={(e) => handleTouchStart(e, day)}
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
    padding: 1rem;
  }

  @media (min-width: 640px) {
    .calendar-wrapper {
      padding: 1.5rem;
    }
  }

  .calendar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    gap: 0.5rem;
  }

  @media (min-width: 640px) {
    .calendar-header {
      margin-bottom: 1.5rem;
      gap: 1rem;
    }
  }

  .calendar-header h2 {
    flex: 1;
    text-align: center;
    margin: 0;
    font-size: 1rem;
  }

  @media (min-width: 640px) {
    .calendar-header h2 {
      font-size: 1.25rem;
    }
  }

  .calendar-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 0.25rem;
    margin-bottom: 1rem;
    user-select: none;
  }

  @media (min-width: 640px) {
    .calendar-grid {
      gap: 0.5rem;
      margin-bottom: 1.5rem;
    }
  }

  .day-header {
    text-align: center;
    font-weight: 600;
    padding: 0.125rem;
    color: #55aa99; /* pine-teal-500 (6.91:1 contrast) */
    font-size: 0.75rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .day-header-mobile {
    display: inline;
  }

  .day-header-desktop {
    display: none;
  }

  @media (min-width: 640px) {
    .day-header {
      padding: 0.5rem;
      font-size: 0.875rem;
    }

    .day-header-mobile {
      display: none;
    }

    .day-header-desktop {
      display: inline;
    }
  }

  .day-cell {
    aspect-ratio: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid rgba(255, 156, 99, 0.1);
    border-radius: 0.25rem;
    background: transparent;
    color: #77bbad; /* pine-teal-400 (8.64:1 contrast) */
    cursor: pointer;
    font-size: 0.75rem;
    transition: all 0.15s ease;
    font-weight: 500;
    padding: 0;
    min-height: 28px;
  }

  @media (min-width: 640px) {
    .day-cell {
      border-radius: 0.375rem;
      font-size: 0.875rem;
      min-height: 44px;
      min-width: 44px;
    }
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
    flex-direction: column;
    gap: 0.5rem;
  }

  @media (min-width: 640px) {
    .calendar-footer {
      flex-direction: row;
      justify-content: space-between;
      align-items: center;
      gap: 1rem;
    }
  }

  .calendar-footer p {
    margin: 0;
    font-size: 0.75rem;
  }

  @media (min-width: 640px) {
    .calendar-footer p {
      font-size: 0.875rem;
    }
  }

  /* Touch-specific improvements */
  @media (hover: none) and (pointer: coarse) {
    .calendar-grid {
      /* Prevent text selection during drag */
      -webkit-user-select: none;
      user-select: none;
      /* Prevent callout on long-press */
      -webkit-touch-callout: none;
    }

    /* Improve visual feedback for touch */
    .day-cell:active {
      transform: scale(0.95);
    }
  }
</style>
