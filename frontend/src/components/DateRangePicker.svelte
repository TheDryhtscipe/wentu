<script>
  import { createEventDispatcher } from 'svelte';
  import Button from './ui/Button.svelte';
  import Input from './ui/Input.svelte';

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
      <Input
        label="Start date"
        type="date"
        class="w-full"
        bind:value={startDate}
        on:keydown={handleKeydown}
      />
    </div>
    <div>
      <Input
        label="End date"
        type="date"
        class="w-full"
        bind:value={endDate}
        on:keydown={handleKeydown}
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
