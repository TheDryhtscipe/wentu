<script>
  import { createEventDispatcher } from 'svelte';
  import { Copy } from 'lucide-svelte';
  import TimeSlotConfigurator from '../../components/TimeSlotConfigurator.svelte';
  import TimezonePicker from '../../components/TimezonePicker.svelte';
  import Card from '../../components/ui/Card.svelte';
  import Button from '../../components/ui/Button.svelte';

  export let data;

  const dispatch = createEventDispatcher();

  let showCopyOptions = false;

  function formatDate(d) {
    const y = d.getFullYear();
    const m = String(d.getMonth() + 1).padStart(2, '0');
    const day = String(d.getDate()).padStart(2, '0');
    return `${y}-${m}-${day}`;
  }

  function getDaysInRange() {
    if (!data.dateRangeStart || !data.dateRangeEnd) return [];
    const days = [];
    const current = new Date(data.dateRangeStart);
    const end = new Date(data.dateRangeEnd);
    while (current <= end) {
      days.push(new Date(current));
      current.setDate(current.getDate() + 1);
    }
    return days;
  }

  function isDayExcluded(day) {
    return data.excludedDays.includes(formatDate(day));
  }

  function getIncludedDays() {
    return getDaysInRange().filter((d) => !isDayExcluded(d));
  }

  function excludeDay(day) {
    const key = formatDate(day);
    if (!data.excludedDays.includes(key)) {
      data.excludedDays = [...data.excludedDays, key];
    }
    if (data.dayTimeSlots[key]) {
      const next = { ...data.dayTimeSlots };
      delete next[key];
      data.dayTimeSlots = next;
    }
    dispatch('change');
  }

  function restoreDay(day) {
    const key = formatDate(day);
    data.excludedDays = data.excludedDays.filter((d) => d !== key);
    dispatch('change');
  }

  function handleTimeSlotChange(day, slots) {
    const key = formatDate(day);
    const next = { ...data.dayTimeSlots };
    if (slots && slots.length > 0) {
      next[key] = slots;
    } else {
      delete next[key];
    }
    data.dayTimeSlots = next;
    dispatch('change');
  }

  function getFirstConfiguredDate() {
    const keys = Object.keys(data.dayTimeSlots).sort();
    if (keys.length === 0) return '';
    const d = new Date(keys[0]);
    return d.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
  }

  function copyTo(predicate) {
    const firstKey = Object.keys(data.dayTimeSlots).sort()[0];
    if (!firstKey) return;
    const slots = data.dayTimeSlots[firstKey];
    const next = { ...data.dayTimeSlots };
    getDaysInRange().forEach((day) => {
      if (isDayExcluded(day)) return;
      if (!predicate(day)) return;
      next[formatDate(day)] = [...slots];
    });
    data.dayTimeSlots = next;
    showCopyOptions = false;
    dispatch('change');
  }

  const copyAll = () => copyTo(() => true);
  const copyWeekdays = () => copyTo((d) => d.getDay() >= 1 && d.getDay() <= 5);
  const copyWeekends = () => copyTo((d) => d.getDay() === 0 || d.getDay() === 6);

  // Valid = every included day has at least one configured time slot.
  $: included = getIncludedDays();
  $: missing = included.filter((d) => {
    const key = formatDate(d);
    return !data.dayTimeSlots[key] || data.dayTimeSlots[key].length === 0;
  });
  $: valid = included.length > 0 && missing.length === 0;
  $: dispatch('valid', valid);

  function handleTimezoneChange(e) {
    data.timezone = e.detail;
    dispatch('change');
  }

  $: days = getDaysInRange();
</script>

<div class="space-y-4">
  <Card>
    <TimezonePicker bind:selectedTimezone={data.timezone} on:change={handleTimezoneChange} />
  </Card>

  <Card>
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between mb-3 sm:mb-4 gap-2 sm:gap-0">
      <h3 class="text-lg sm:text-xl font-bold text-accent">Configure time slots</h3>
      {#if Object.keys(data.dayTimeSlots).length > 0}
        <Button variant="secondary" class="text-xs sm:text-sm" on:click={() => (showCopyOptions = !showCopyOptions)}>
          <Copy size={16} class="inline mr-1" />
          Copy times
        </Button>
      {/if}
    </div>

    {#if showCopyOptions && Object.keys(data.dayTimeSlots).length > 0}
      <div class="bg-dark-bg p-3 sm:p-4 rounded mb-3 sm:mb-4 border border-accent/30">
        <p class="text-text-secondary text-xs sm:text-sm mb-2 sm:mb-3">
          Copy times from {getFirstConfiguredDate()} to:
        </p>
        <div class="flex gap-2 flex-wrap">
          <Button variant="secondary" class="text-xs sm:text-sm px-2 sm:px-3 py-1.5 sm:py-2" on:click={copyAll}>All days</Button>
          <Button variant="secondary" class="text-xs sm:text-sm px-2 sm:px-3 py-1.5 sm:py-2" on:click={copyWeekdays}>Weekdays</Button>
          <Button variant="secondary" class="text-xs sm:text-sm px-2 sm:px-3 py-1.5 sm:py-2" on:click={copyWeekends}>Weekends</Button>
        </div>
      </div>
    {/if}

    <div class="space-y-3 sm:space-y-4">
      {#each days as day (formatDate(day))}
        {@const dayKey = formatDate(day)}
        {#if data.excludedDays.includes(dayKey)}
          <Card elevated>
            <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-2">
              <div>
                <p class="text-text-secondary text-xs sm:text-sm">Excluded day</p>
                <p class="text-text-primary font-medium text-sm sm:text-base">
                  {day.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })}
                </p>
              </div>
              <Button
                variant="secondary"
                class="text-xs sm:text-sm px-2 sm:px-3 py-1"
                on:click={() => restoreDay(day)}
              >
                Restore day
              </Button>
            </div>
          </Card>
        {:else}
          <TimeSlotConfigurator
            date={day}
            timeSlots={data.dayTimeSlots[dayKey] || []}
            allowRemove={true}
            on:change={(e) => handleTimeSlotChange(day, e.detail)}
            on:exclude={() => excludeDay(day)}
          />
        {/if}
      {/each}
    </div>

    {#if missing.length > 0}
      <p class="mt-3 text-xs text-error">
        Configure time slots for every included day, or exclude days you don't want ranked.
      </p>
    {/if}
  </Card>
</div>
