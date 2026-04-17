<script>
  import { createEventDispatcher } from 'svelte';
  import { Clock3 } from 'lucide-svelte';
  import Calendar from '../../components/Calendar.svelte';

  export let data;

  const dispatch = createEventDispatcher();

  function handleDateRange(event) {
    data.dateRangeStart = event.detail.start;
    data.dateRangeEnd = event.detail.end;
    dispatch('change');
  }

  function clearRange() {
    data.dateRangeStart = null;
    data.dateRangeEnd = null;
    dispatch('change');
  }

  function toggleTimeSlots() {
    data.enableTimeSlots = !data.enableTimeSlots;
    dispatch('change');
  }

  $: hasRange = !!(data.dateRangeStart && data.dateRangeEnd);
  $: valid = hasRange;
  $: dispatch('valid', valid);

  const fmtOpts = { weekday: 'short', month: 'short', day: 'numeric' };
</script>

<div class="space-y-5">
  <div>
    <h3 class="text-base font-semibold text-text-primary mb-1">Date range</h3>
    <p class="text-text-secondary text-sm mb-3">
      Pick the window participants can rank. Each day in the range becomes a ranking option.
    </p>
    <Calendar mode="range" on:daterange={handleDateRange} />
  </div>

  {#if hasRange}
    <div class="rounded-lg border border-border-subtle bg-surface-card p-3 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-2">
      <p class="text-text-primary font-medium text-sm sm:text-base">
        {data.dateRangeStart.toLocaleDateString('en-US', fmtOpts)} – {data.dateRangeEnd.toLocaleDateString('en-US', fmtOpts)}
      </p>
      <button
        type="button"
        class="text-error hover:text-accent text-xs sm:text-sm px-3 py-1 bg-error/10 rounded cursor-pointer"
        on:click={clearRange}
      >
        Clear
      </button>
    </div>
  {/if}

  <!--
    Prominent time-slot toggle. Styled as a large toggle card (not a buried
    checkbox) so it reads as a first-class choice: "do you want to
    configure times within each day, or just rank whole days?"
  -->
  <button
    type="button"
    class="w-full text-left rounded-lg border-2 p-4 transition-colors cursor-pointer focus:outline-offset-2 {data.enableTimeSlots ? 'border-action-primary bg-action-primary/10' : 'border-border-subtle bg-surface-card hover:border-border-strong'}"
    aria-pressed={data.enableTimeSlots}
    on:click={toggleTimeSlots}
  >
    <div class="flex items-start gap-3">
      <Clock3 size={24} class="flex-shrink-0 mt-0.5 {data.enableTimeSlots ? 'text-action-primary' : 'text-text-muted'}" aria-hidden="true" />
      <div class="flex-1">
        <div class="flex items-center justify-between gap-3">
          <h3 class="text-base font-semibold text-text-primary">Specific times?</h3>
          <span
            class="inline-flex h-6 w-10 items-center rounded-full p-0.5 transition-colors {data.enableTimeSlots ? 'bg-action-primary' : 'bg-border-strong'}"
            aria-hidden="true"
          >
            <span class="h-5 w-5 rounded-full bg-dark-bg transition-transform {data.enableTimeSlots ? 'translate-x-4' : 'translate-x-0'}"></span>
          </span>
        </div>
        <p class="text-text-secondary text-sm mt-1">
          Add up to 3 start times per day (e.g. 10am, 1pm, 7pm). Off by default.
        </p>
      </div>
    </div>
  </button>
</div>
