<script>
  import { createEventDispatcher } from 'svelte';
  import { Trash2, Plus } from 'lucide-svelte';

  export let date;  // Date object
  export let timeSlots = [];  // ["10:00", "13:00"]

  const dispatch = createEventDispatcher();
  const quickTimes = ['10:00', '13:00', '19:00'];
  let newTime = '';

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

  $: formattedDate = date.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
</script>

<div class="border border-accent/20 rounded-lg p-4 bg-content-bg">
  <div class="flex items-center justify-between mb-3">
    <h4 class="text-text-primary font-medium">{formattedDate}</h4>
    <span class="text-text-secondary text-sm">{timeSlots.length}/3 slots</span>
  </div>

  <!-- Quick select -->
  <div class="mb-3">
    <p class="text-text-secondary text-sm mb-2">Quick select:</p>
    <div class="flex gap-2 flex-wrap">
      {#each quickTimes as time}
        <button class="btn-secondary text-sm px-3 py-1" on:click={() => addTimeSlot(time)} disabled={timeSlots.length >= 3}>
          {time}
        </button>
      {/each}
    </div>
  </div>

  <!-- Custom time -->
  <div class="mb-3">
    <label class="text-text-secondary text-sm mb-1 block">Custom time:</label>
    <div class="flex gap-2">
      <input type="time" class="input flex-1" bind:value={newTime} disabled={timeSlots.length >= 3} />
      <button class="btn-secondary px-3" on:click={addCustomTime} disabled={!newTime || timeSlots.length >= 3}>
        <Plus size={18} />
      </button>
    </div>
  </div>

  <!-- Selected times -->
  {#if timeSlots.length > 0}
    <div class="space-y-2">
      <p class="text-text-secondary text-sm">Selected times:</p>
      {#each timeSlots as slot, i}
        <div class="flex items-center justify-between bg-dark-bg p-2 rounded">
          <span class="text-text-primary font-medium">{slot}</span>
          <button class="text-error hover:text-accent" on:click={() => removeTimeSlot(i)}>
            <Trash2 size={16} />
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>
