<script>
  import { createEventDispatcher } from 'svelte';
  import Button from './ui/Button.svelte';

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
      <!-- Plain input retained: aria-label not forwarded by Input primitive ($$restProps gap). Tokens upgraded to match primitive styling. -->
      <input
        class="w-full px-2 sm:px-3 py-2 bg-surface-card border border-border-subtle rounded text-text-primary placeholder-text-secondary focus:border-focus-ring focus:outline-none text-sm sm:text-base"
        type="date"
        bind:value={startDate}
        on:keydown={handleKeydown}
        aria-label="Start date"
      />
    </div>
    <div>
      <label class="block text-text-primary text-sm font-medium mb-2">End date</label>
      <!-- Plain input retained: aria-label not forwarded by Input primitive ($$restProps gap). Tokens upgraded to match primitive styling. -->
      <input
        class="w-full px-2 sm:px-3 py-2 bg-surface-card border border-border-subtle rounded text-text-primary placeholder-text-secondary focus:border-focus-ring focus:outline-none text-sm sm:text-base"
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

  <Button variant="secondary" fullWidth on:click={addDateRange}>
    Add date range
  </Button>
</div>
