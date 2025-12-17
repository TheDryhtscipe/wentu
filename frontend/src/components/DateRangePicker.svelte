<script>
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  let startDate = '';
  let endDate = '';
  let error = '';

  function addDateRange() {
    if (!startDate || !endDate) {
      error = 'Both start and end dates required';
      return;
    }

    const start = new Date(startDate);
    const end = new Date(endDate);

    if (start > end) {
      error = 'Start date must be before end date';
      return;
    }

    dispatch('daterange', { start, end });
    startDate = '';
    endDate = '';
    error = '';
  }

  function handleKeydown(e) {
    if (e.key === 'Enter') {
      addDateRange();
    }
  }
</script>

<div>
  <div class="grid gap-4 md:grid-cols-2 mb-4">
    <div>
      <label class="block text-text-primary text-sm font-medium mb-2">Start date</label>
      <input
        class="input w-full"
        type="date"
        bind:value={startDate}
        on:keydown={handleKeydown}
        aria-label="Start date"
      />
    </div>
    <div>
      <label class="block text-text-primary text-sm font-medium mb-2">End date</label>
      <input
        class="input w-full"
        type="date"
        bind:value={endDate}
        on:keydown={handleKeydown}
        aria-label="End date"
      />
    </div>
  </div>

  {#if error}
    <p class="text-error text-sm mb-4">{error}</p>
  {/if}

  <button class="btn-secondary w-full" on:click={addDateRange}>
    Add date range
  </button>
</div>
