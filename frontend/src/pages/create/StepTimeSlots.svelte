<script>
  import { createEventDispatcher, onMount } from 'svelte';
  import { Copy } from 'lucide-svelte';
  import TimeSlotConfigurator from '../../components/TimeSlotConfigurator.svelte';
  import TimezonePicker from '../../components/TimezonePicker.svelte';
  import Card from '../../components/ui/Card.svelte';
  import Button from '../../components/ui/Button.svelte';

  export let data;

  const dispatch = createEventDispatcher();

  let showCopyOptions = false;
  let showFloatingCopyOptions = false;

  // Track whether the inline "Copy times" button is in view. When it scrolls
  // out of view (and the user has configured at least one day), a floating
  // variant appears anchored to the right edge so the affordance is always
  // reachable.
  let inlineCopyButton = null;
  let inlineCopyVisible = true;
  let observer = null;

  onMount(() => {
    if (typeof IntersectionObserver === 'undefined') return;
    observer = new IntersectionObserver(
      ([entry]) => {
        inlineCopyVisible = entry.isIntersecting;
      },
      { threshold: 0.1 }
    );
    return () => {
      observer?.disconnect();
      observer = null;
    };
  });

  // Re-wire the observer whenever the inline button reference changes.
  // The button is inside an {#if}, so it mounts/unmounts as the user adds
  // or clears their first configured slot.
  $: if (observer && inlineCopyButton) {
    observer.disconnect();
    observer.observe(inlineCopyButton);
    // Assume visible initially — the IntersectionObserver callback fires
    // asynchronously and will correct this shortly.
    inlineCopyVisible = true;
  } else if (observer && !inlineCopyButton) {
    observer.disconnect();
    inlineCopyVisible = true;
  }

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
        <span bind:this={inlineCopyButton}>
          <Button variant="secondary" class="text-xs sm:text-sm" on:click={() => (showCopyOptions = !showCopyOptions)}>
            <Copy size={16} class="inline mr-1" />
            Copy times
          </Button>
        </span>
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

<!--
  Floating "Copy times" affordance. Appears only when:
  (a) at least one day has been configured (same gate as the inline button),
  (b) the inline button is not currently in view (IntersectionObserver),
  (c) the viewport is at least `lg:` — below that, the max-w-2xl content
      container consumes most of the width and there's no empty space to
      float into without overlapping content.
  Its popover anchors to the floating button, on the right edge, so the
  affordance and its options stay visually co-located.
-->
{#if Object.keys(data.dayTimeSlots).length > 0 && !inlineCopyVisible}
  <!--
    Anchor the floating button to the right edge of the content container
    (max-w-2xl = 42rem, centered), not the viewport. Formula: viewport
    centre + half content width + small gap. Keeps the button visually
    tethered to the content so it reads as a companion to the time-slots
    card rather than a disconnected corner element.
  -->
  <div
    class="hidden lg:block fixed top-1/2 -translate-y-1/2 z-30"
    style="left: calc(50% + 21rem + 1rem);"
  >
    <div class="relative">
      <Button
        variant="primary"
        class="shadow-lg"
        aria-label="Copy times to other days"
        on:click={() => (showFloatingCopyOptions = !showFloatingCopyOptions)}
      >
        <Copy size={16} class="inline mr-1" />
        Copy times
      </Button>

      {#if showFloatingCopyOptions}
        <div
          class="absolute left-0 mt-2 w-64 bg-surface-elevated border border-accent/30 rounded-lg p-3 shadow-xl"
          role="dialog"
          aria-label="Copy times options"
        >
          <p class="text-text-secondary text-xs mb-2">
            Copy times from {getFirstConfiguredDate()} to:
          </p>
          <div class="flex flex-col gap-2">
            <Button
              variant="secondary"
              class="text-sm w-full justify-start"
              on:click={() => { copyAll(); showFloatingCopyOptions = false; }}
            >
              All days
            </Button>
            <Button
              variant="secondary"
              class="text-sm w-full justify-start"
              on:click={() => { copyWeekdays(); showFloatingCopyOptions = false; }}
            >
              Weekdays
            </Button>
            <Button
              variant="secondary"
              class="text-sm w-full justify-start"
              on:click={() => { copyWeekends(); showFloatingCopyOptions = false; }}
            >
              Weekends
            </Button>
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}
