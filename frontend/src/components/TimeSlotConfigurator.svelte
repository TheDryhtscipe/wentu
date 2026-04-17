<script>
  import { createEventDispatcher } from 'svelte';
  import { Trash2, Plus, X } from 'lucide-svelte';
  import Button from './ui/Button.svelte';

  export let date;  // Date object
  export let timeSlots = [];  // ["10:00", "13:00"]
  export let allowRemove = false;

  const dispatch = createEventDispatcher();
  const quickTimes = ['10:00', '13:00', '19:00'];
  let newTime = '';
  let editingIndex = null;
  let editTime = '';

  $: if (!Array.isArray(timeSlots)) {
    timeSlots = [];
  }

  function addTimeSlot(time) {
    if (timeSlots.length >= 3 || timeSlots.includes(time)) return;
    timeSlots = [...timeSlots, time].sort();
    dispatch('change', timeSlots);
  }

  function removeTimeSlot(index) {
    timeSlots = timeSlots.filter((_, i) => i !== index);
    dispatch('change', timeSlots);
  }

  function addCustomTime() {
    if (newTime && !timeSlots.includes(newTime)) {
      addTimeSlot(newTime);
      newTime = '';
    }
  }

  function handleExclude(event) {
    event.preventDefault();
    console.log('TimeSlotConfigurator: exclude button clicked, dispatching event');
    dispatch('exclude');
  }

  function startEdit(index) {
    editingIndex = index;
    editTime = timeSlots[index];
  }

  function cancelEdit() {
    editingIndex = null;
    editTime = '';
  }

  function saveEdit() {
    if (editingIndex === null) return;
    if (!editTime) {
      cancelEdit();
      return;
    }
    if (timeSlots.some((slot, i) => slot === editTime && i !== editingIndex)) return;

    timeSlots = timeSlots.map((slot, i) => (i === editingIndex ? editTime : slot)).sort();
    dispatch('change', timeSlots);
    cancelEdit();
  }

  $: formattedDate = date.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
</script>

<div class="border border-accent/20 rounded-lg p-3 sm:p-4 bg-content-bg">
  <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between mb-2 sm:mb-3 gap-1 sm:gap-0">
    <h4 class="text-text-primary font-medium text-sm sm:text-base">{formattedDate}</h4>
    <div class="flex items-center gap-2">
      <span class="text-text-secondary text-xs sm:text-sm">{timeSlots.length}/3 slots</span>
      {#if allowRemove}
        <button
          type="button"
          class="text-error hover:text-accent p-1 rounded bg-error/10"
          on:click={handleExclude}
          aria-label="Remove {formattedDate} from range"
        >
          <X size={16} />
        </button>
      {/if}
    </div>
  </div>

  <!-- Quick select -->
  <div class="mb-2 sm:mb-3">
    <p class="text-text-secondary text-xs sm:text-sm mb-2">Quick select:</p>
    <div class="flex gap-2 flex-wrap">
      {#each quickTimes as time}
        <Button type="button" variant="secondary" class="text-xs sm:text-sm px-2 sm:px-3 py-1" on:click={() => addTimeSlot(time)} disabled={timeSlots.length >= 3}>
          {time}
        </Button>
      {/each}
    </div>
  </div>

  <!-- Custom time -->
  <div class="mb-2 sm:mb-3">
    <label for="custom-time" class="text-text-secondary text-xs sm:text-sm mb-1 block">Custom time (HH:MM format):</label>
    <div class="flex gap-2">
      <!-- Plain input retained: title attribute not forwarded by Input primitive ($$restProps gap). Class="input" kept as marker so input[type="time"].input CSS hook applies the clock SVG. Tokens upgraded to match primitive styling. -->
      <input
        id="custom-time"
        type="time"
        class="input flex-1 px-2 sm:px-3 py-2 bg-surface-card border border-border-subtle rounded text-text-primary placeholder-text-secondary focus:border-focus-ring focus:outline-none text-sm sm:text-base"
        bind:value={newTime}
        disabled={timeSlots.length >= 3}
        placeholder="14:30"
        title="Enter time in HH:MM format (e.g., 14:30)"
      />
      <Button type="button" variant="secondary" class="px-2 sm:px-3" on:click={addCustomTime} disabled={!newTime || timeSlots.length >= 3}>
        <Plus size={18} />
      </Button>
    </div>
  </div>

  <!-- Selected times -->
  {#if timeSlots.length > 0}
    <div class="space-y-2">
      <p class="text-text-secondary text-xs sm:text-sm">Selected times (click to edit):</p>
      {#each timeSlots as slot, i}
        {#if editingIndex === i}
          <div class="flex items-center gap-2 bg-dark-bg p-2 rounded">
            <!-- Plain input retained: title attribute not forwarded by Input primitive ($$restProps gap). Class="input" kept as marker so input[type="time"].input CSS hook applies the clock SVG. Tokens upgraded to match primitive styling. -->
            <input
              type="time"
              class="input flex-1 px-2 sm:px-3 py-2 bg-surface-card border border-border-subtle rounded text-text-primary placeholder-text-secondary focus:border-focus-ring focus:outline-none text-sm sm:text-base"
              bind:value={editTime}
              placeholder="14:30"
              title="Enter time in HH:MM format (e.g., 14:30)"
            />
            <Button type="button" variant="secondary" class="text-xs sm:text-sm px-2 sm:px-3 py-1" on:click={saveEdit}>
              Save
            </Button>
            <Button type="button" variant="secondary" class="text-xs sm:text-sm px-2 sm:px-3 py-1" on:click={cancelEdit}>
              Cancel
            </Button>
          </div>
        {:else}
          <div class="flex items-center justify-between bg-dark-bg p-2 rounded gap-2">
            <button
              type="button"
              class="text-left flex-1"
              on:click={() => startEdit(i)}
              aria-label="Edit {slot}"
            >
              <span class="text-text-primary font-medium text-sm sm:text-base">{slot}</span>
            </button>
            <button type="button" class="text-error hover:text-accent" on:click={() => removeTimeSlot(i)} aria-label="Remove {slot}">
              <Trash2 size={16} />
            </button>
          </div>
        {/if}
      {/each}
    </div>
  {/if}
</div>
