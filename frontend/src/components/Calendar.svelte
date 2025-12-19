<script>
  import { createEventDispatcher } from 'svelte';
  import { ChevronLeft, ChevronRight } from 'lucide-svelte';

  const dispatch = createEventDispatcher();

  export let year = new Date().getFullYear();
  export let month = new Date().getMonth();
  export let mode = 'range';  // 'range' or 'preferences'
  export let selectedDates = [];  // For preferences mode: array of { date, order }
  export let availableDateOptions = [];  // For preferences mode: array of { start, end, label, id }



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
  let touchStartX = null;  // Track X position for movement threshold
  let touchStartY = null;  // Track Y position for movement threshold
  const TOUCH_MOVE_THRESHOLD = 10;  // pixels - only count as drag if moved > this distance
  
  // Span-in-progress state for preferences mode
  let spanStartKey = null;

  // Reactive maps for template reactivity (Svelte can't track dependencies inside function calls)
  $: orderByDateKey = new Map(
    selectedDates.map(d => [d.date || d.dateStart, d.order])
  );
  $: selectedDateKeys = new Set(
    selectedDates.flatMap(d => [d.date, d.dateStart, d.dateEnd].filter(Boolean))
  );
  
  // Auto-navigation on drag
  let dragAutoNavTimeout = null;

  const monthNames = [
    'January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December'
  ];

  const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  const dayNamesMobile = ['S', 'M', 'T', 'W', 'T', 'F', 'S'];

  let firstDay;
  let lastDay;
  let daysInMonth;
  let startingDayOfWeek;

  $: {
    firstDay = new Date(year, month, 1);
    lastDay = new Date(year, month + 1, 0);
    daysInMonth = lastDay.getDate();
    startingDayOfWeek = firstDay.getDay();
  }

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
    if (mode === 'preferences') {
      return selectedDates.some(d => (d.date || d.dateStart) === key || d.dateEnd === key);
    }
    return startDate === key || endDate === key;
  }

  function getDateOrder(day) {
    const key = getDayKey(day);
    const selected = selectedDates.find(d => (d.date || d.dateStart) === key || d.dateEnd === key);
    return selected ? selected.order : null;
  }

  function isDateInSelection(day) {
    const key = getDayKey(day);
    const date = new Date(year, month, day);
    
    for (const selection of selectedDates) {
      if (selection.dateStart && selection.dateEnd) {
        const start = parseKey(selection.dateStart);
        const end = parseKey(selection.dateEnd);
        if (date > start && date < end) {
          return true;
        }
      }
    }
    return false;
  }

  function isDateAvailable(day) {
    // Check if this date falls within any available date option
    if (mode !== 'preferences' || availableDateOptions.length === 0) {
      return true;
    }

    // Get the calendar date as a string (YYYY-MM-DD)
    const checkDateKey = getDayKey(day);

    // Check if any option's start date matches this calendar day
    const available = availableDateOptions.some(option => {
      // Parse the start time and extract just the date part
      const optStart = new Date(option.start);
      const optYear = optStart.getUTCFullYear();
      const optMonth = String(optStart.getUTCMonth() + 1).padStart(2, '0');
      const optDay = String(optStart.getUTCDate()).padStart(2, '0');
      const optDateKey = `${optYear}-${optMonth}-${optDay}`;

      // A calendar day is available if any option starts on that day
      return checkDateKey === optDateKey;
    });

    return available;
  }

  function movePreferenceToEnd(key) {
    // Move a date to the end of the preference list
    const existing = selectedDates.findIndex(d => (d.date || d.dateStart) === key || d.dateEnd === key);
    
    if (existing !== -1) {
      // Remove from current position
      const item = selectedDates[existing];
      selectedDates = selectedDates.filter((_, i) => i !== existing);
      // Add to end with new order
      const nextOrder = selectedDates.length + 1;
      selectedDates = [...selectedDates, { ...item, order: nextOrder }];
    } else {
      // If not already in list, add to end
      const nextOrder = Math.max(...selectedDates.map(d => d.order), 0) + 1;
      selectedDates = [...selectedDates, { date: key, order: nextOrder }];
    }
  }

  function togglePreferenceSelection(key) {
    const existing = selectedDates.findIndex(d => (d.date || d.dateStart) === key || d.dateEnd === key);
    
    if (existing !== -1) {
      // Remove the date
      selectedDates = selectedDates.filter((_, i) => i !== existing);
      // Renumber remaining dates
      selectedDates = selectedDates.map((d, i) => ({ ...d, order: i + 1 }));
      if (spanStartKey === key) {
        spanStartKey = null;
      }
    } else {
      // Add to the end with next order number
      const nextOrder = Math.max(...selectedDates.map(d => d.order), 0) + 1;
      selectedDates = [...selectedDates, { date: key, order: nextOrder }];
      spanStartKey = key;
    }
  }

  function handleMouseDown(day) {
    // Don't start mouse interaction if touch is active
    if (interactionType === 'touch') return;

    interactionType = 'mouse';
    isMouseDown = true;
    isDragging = false;
    dragStartDay = day;
    const key = getDayKey(day);
    
    if (mode === 'preferences') {
      // In preferences mode, track the start date for the span
      spanStartKey = key;
      startedWithMouseDown = true;
    } else {
      if (!startDate || endDate) {
        startDate = key;
        endDate = null;
        startedWithMouseDown = true;
      } else {
        startedWithMouseDown = false;
      }
    }
    
    hoveredDate = key;
  }

  function handleMouseEnter(day) {
    // Only respond to mouse hover if in mouse mode
    if (interactionType === 'touch') return;

    if (isMouseDown && dragStartDay !== null) {
      const currentKey = getDayKey(day);
      hoveredDate = currentKey;
      
      if (mode === 'preferences') {
        // Track end date for visual feedback
        endDate = currentKey;
      } else {
        endDate = currentKey;
      }
      
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

      if (mode === 'preferences') {
        // In preferences mode, dragging creates a span
        if (spanStartKey && spanStartKey !== key) {
          const start = parseKey(spanStartKey);
          const end = parseKey(key);
          const minKey = start < end ? spanStartKey : key;
          const maxKey = start < end ? key : spanStartKey;
          const nextOrder = Math.max(...selectedDates.map(d => d.order), 0) + 1;
          selectedDates = [...selectedDates, { dateStart: minKey, dateEnd: maxKey, order: nextOrder }];
          spanStartKey = null;
          dispatch('preferenceschange', { selectedDates });
        }
      } else {
        // Emit the event with properly ordered dates
        const start = parseKey(startDate);
        const end = parseKey(endDate);
        dispatch('daterange', {
          start: start < end ? start : end,
          end: start < end ? end : start
        });
      }
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
    const key = getDayKey(day);
    
    if (mode === 'preferences') {
      // Guard: only allow clicking available dates
      if (!isDateAvailable(day)) {
        return;
      }
      togglePreferenceSelection(key);
      dispatch('preferenceschange', { selectedDates });
      return;
    }

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

    applyClickSelection(key);
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
    if (mode === 'preferences') {
      selectedDates = [];
      spanStartKey = null;
      dispatch('preferenceschange', { selectedDates: [] });
    }
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
    const touch = event.touches[0];
    touchIdentifier = touch.identifier;
    touchStartX = touch.clientX;
    touchStartY = touch.clientY;
    dragStartDay = day;
    touchStartDay = day;
    touchMoved = false;

    if (mode === 'preferences') {
      // In preferences mode, we're doing tap-to-select, not dragging
      // Don't set isDragging yet - let handleTouchMove decide if it's a drag
      hoveredDate = getDayKey(day);
    } else {
      // In range mode, just track the touch start
      // Don't set dates yet - let handleTouchEnd handle it via applyClickSelection
      hoveredDate = getDayKey(day);
    }
  }

  function handleTouchMove(event) {
    event.preventDefault();

    if (interactionType !== 'touch') return;

    // Find which touch is ours
    const touch = Array.from(event.touches).find(
      t => t.identifier === touchIdentifier
    );

    if (!touch) return;

    // Check if touch has moved beyond threshold (ignore tiny jitter)
    const distance = Math.sqrt(
      Math.pow(touch.clientX - touchStartX, 2) + 
      Math.pow(touch.clientY - touchStartY, 2)
    );
    
    if (distance < TOUCH_MOVE_THRESHOLD) {
      // Too small to be a drag, ignore
      return;
    }

    touchMoved = true;

    // Find element under touch point
    const element = document.elementFromPoint(touch.clientX, touch.clientY);
    if (!element) return;

    // Check if element is a day cell with data-day attribute
    const dayCell = element.closest('[data-day]');
    if (!dayCell) return;

    const day = parseInt(dayCell.dataset.day);
    if (isNaN(day)) return;

    if (mode === 'range') {
      // In range mode, start drag when finger moves beyond threshold
      if (!isDragging) {
        isDragging = true;
        // Set startDate from the initial touch point
        const startKey = getDayKey(dragStartDay);
        if (!startDate || endDate) {
          startDate = startKey;
          endDate = null;
        }
      }
      // Update endDate as finger moves
      const currentKey = getDayKey(day);
      hoveredDate = currentKey;
      endDate = currentKey;
    }
  }

  function handleTouchEnd(event) {
    event.preventDefault();

    if (interactionType !== 'touch') return;

    // Find the touch that ended
    const touch = Array.from(event.changedTouches).find(
      t => t.identifier === touchIdentifier
    );

    if (!touch) return;

    if (mode === 'preferences') {
      // In preferences mode, handle tap-to-select for reordering
      if (!touchMoved && touchStartDay !== null) {
        const key = getDayKey(touchStartDay);
        // Guard: only allow clicking available dates
        if (isDateAvailable(touchStartDay)) {
          togglePreferenceSelection(key);
          dispatch('preferenceschange', { selectedDates });
        }
      }
    } else {
      // In range mode, handle drag or tap selection
      if (isDragging && touchMoved && startDate && endDate) {
        // Drag complete - emit the range
        const start = parseKey(startDate);
        const end = parseKey(endDate);
        dispatch('daterange', {
          start: start < end ? start : end,
          end: start < end ? end : start
        });
      } else if (!touchMoved && touchStartDay !== null) {
        // Simple tap - use applyClickSelection for consistent behavior with desktop clicks
        applyClickSelection(getDayKey(touchStartDay));
      }
    }

    isDragging = false;
    interactionType = null;
    touchIdentifier = null;
    touchStartX = null;
    touchStartY = null;
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
      touchStartX = null;
      touchStartY = null;
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
    role="grid"
    tabindex="0"
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

    {#each days as day (year + '-' + month + '-' + day)}
      {@const key = getDayKey(day)}
      {@const order = mode === 'preferences' ? orderByDateKey.get(key) : null}
      {@const inSelection = mode === 'preferences' ? isDateInSelection(day) : false}
      {@const available = isDateAvailable(day)}
      {@const isSelectedDate = mode === 'preferences' ? selectedDateKeys.has(key) : (startDate === key || endDate === key)}
      <button
        data-day={day}
        on:mousedown={() => available && handleMouseDown(day)}
        on:mouseenter={() => available && handleMouseEnter(day)}
        on:mouseleave={handleMouseLeave}
        on:mouseup={() => available && handleMouseUp(day)}
        on:click={() => available && handleClick(day)}
        on:touchstart={(e) => available && handleTouchStart(e, day)}
        disabled={!available}
        class="day-cell"
        class:in-range={mode === 'range' ? isInRange(day) : inSelection}
        class:selected={isSelectedDate}
        class:drag-endpoint={isDragging && hoveredDate === key}
        class:unavailable={!available}
        aria-label="Select {monthNames[month]} {day}"
      >
        <span class="day-number">{day}</span>
        {#if order}
          <span class="order-badge">{order}</span>
        {/if}
      </button>
    {/each}
  </div>

  <div class="calendar-footer">
    {#if mode === 'preferences'}
      {#if selectedDates.length > 0}
        <p class="text-text-secondary text-sm">
          Selected {selectedDates.length} option{selectedDates.length !== 1 ? 's' : ''}
        </p>
      {/if}
      {#if selectedDates.length > 0}
        <button on:click={reset} class="btn-secondary px-3 py-1 text-sm">Clear selection</button>
      {/if}
    {:else}
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
    position: relative;
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

  .day-number {
    display: flex;
    align-items: center;
    justify-content: center;
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

  .day-cell.unavailable {
    color: #4a4a4a;
    border-color: rgba(255, 156, 99, 0.05);
    cursor: not-allowed;
    opacity: 0.4;
  }

  .day-cell:not(.empty):not(.unavailable):hover {
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

  .order-badge {
    position: absolute;
    top: 2px;
    right: 2px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.2em;
    height: 1.2em;
    font-weight: 700;
    font-size: 0.6em;
    background: var(--accent, #ff9c63);
    color: var(--dark-bg, #2b1313);
    border-radius: 50%;
  }

  @media (min-width: 640px) {
    .order-badge {
      width: 1.4em;
      height: 1.4em;
      font-size: 0.65em;
    }
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
