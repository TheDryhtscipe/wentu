<script>
  import { createEventDispatcher } from 'svelte';
  import { AlertCircle, Loader2, Pencil } from 'lucide-svelte';
  import Card from '../../components/ui/Card.svelte';
  import Button from '../../components/ui/Button.svelte';
  import { api } from '../../lib/api.js';
  import { addTrackedWentu } from '../../lib/wentuTracker.js';

  export let data;

  const dispatch = createEventDispatcher();

  let loading = false;
  let error = '';

  function edit(stepId) {
    dispatch('edit', { id: stepId });
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

  function getIncludedKeys() {
    return new Set(
      getDaysInRange()
        .filter((d) => !data.excludedDays.includes(formatDate(d)))
        .map(formatDate)
    );
  }

  const dateFmt = { weekday: 'short', month: 'short', day: 'numeric' };
  const dtFmt = { weekday: 'short', month: 'short', day: 'numeric', hour: 'numeric', minute: '2-digit' };

  $: dayCount = (() => {
    if (!data.dateRangeStart || !data.dateRangeEnd) return 0;
    const all = getDaysInRange();
    return all.filter((d) => !data.excludedDays.includes(formatDate(d))).length;
  })();

  $: deadlineDate = (() => {
    if (!data.prefDeadline || !data.prefDeadlineTime) return null;
    const d = new Date(`${data.prefDeadline}T${data.prefDeadlineTime}`);
    return isNaN(d.getTime()) ? null : d;
  })();

  $: configuredDays = Object.keys(data.dayTimeSlots)
    .filter((key) => {
      const all = getDaysInRange();
      return all.some((d) => formatDate(d) === key) && !data.excludedDays.includes(key);
    })
    .sort();

  async function submit() {
    error = '';

    if (!data.title || !data.creatorName || !data.dateRangeStart || !data.dateRangeEnd || !data.prefDeadline || !data.prefDeadlineTime) {
      error = 'Title, name, date range, and voting-closes time are required.';
      return;
    }

    const deadline = new Date(`${data.prefDeadline}T${data.prefDeadlineTime}`);
    if (isNaN(deadline.getTime()) || deadline <= new Date()) {
      error = 'Voting-closes time must be in the future.';
      return;
    }

    if (data.enableTimeSlots) {
      const includedKeys = getIncludedKeys();
      if (includedKeys.size === 0) {
        error = 'Keep at least one day in the range.';
        return;
      }
      const missing = [...includedKeys].filter(
        (key) => !data.dayTimeSlots[key] || data.dayTimeSlots[key].length === 0
      );
      if (missing.length > 0) {
        error = 'Configure time slots for every included day before submitting.';
        return;
      }
    }

    loading = true;
    try {
      const includedKeys = getIncludedKeys();
      const filteredTimeSlots = Object.fromEntries(
        Object.entries(data.dayTimeSlots).filter(([key]) => includedKeys.has(key))
      );

      // Calendar returns local midnight; backend stores dates as UTC
      // midnight to avoid tz drift.
      const startUTC = new Date(Date.UTC(
        data.dateRangeStart.getFullYear(),
        data.dateRangeStart.getMonth(),
        data.dateRangeStart.getDate()
      ));
      const endUTC = new Date(Date.UTC(
        data.dateRangeEnd.getFullYear(),
        data.dateRangeEnd.getMonth(),
        data.dateRangeEnd.getDate()
      ));

      const body = {
        title: data.title,
        description: data.description || null,
        creator_name: data.creatorName,
        date_range_start: startUTC.toISOString(),
        date_range_end: endUTC.toISOString(),
        pref_deadline: deadline.toISOString(),
        enable_time_slots: data.enableTimeSlots || null,
        timezone: data.enableTimeSlots ? data.timezone : null,
        day_time_slots: data.enableTimeSlots ? filteredTimeSlots : null,
      };

      const result = await api.post('/api/wentu', body);

      addTrackedWentu(
        result.slug,
        data.title,
        'owner',
        data.creatorName,
        result.creator_participant_id,
        result.creator_participant_key
      );

      dispatch('navigate', {
        page: 'view',
        params: {
          slug: result.slug,
          creatorName: data.creatorName,
          creatorKey: result.creator_key,
          creatorParticipantId: result.creator_participant_id,
          creatorParticipantKey: result.creator_participant_key,
        },
      });
    } catch (err) {
      error = err.message || 'Something went wrong creating the wentu.';
    } finally {
      loading = false;
    }
  }
</script>

<div class="space-y-4">
  <!-- Basics summary -->
  <Card>
    <div class="flex items-start justify-between gap-2 mb-3">
      <h3 class="text-base font-semibold text-text-primary">Basics</h3>
      <button
        type="button"
        class="inline-flex items-center gap-1 text-sm text-action-primary hover:underline cursor-pointer"
        on:click={() => edit('basics')}
      >
        <Pencil size={14} aria-hidden="true" />
        Edit
      </button>
    </div>
    <dl class="space-y-2 text-sm">
      <div class="flex gap-2">
        <dt class="text-text-muted w-28 shrink-0">Title</dt>
        <dd class="text-text-primary">{data.title}</dd>
      </div>
      {#if data.description}
        <div class="flex gap-2">
          <dt class="text-text-muted w-28 shrink-0">Description</dt>
          <dd class="text-text-primary whitespace-pre-wrap">{data.description}</dd>
        </div>
      {/if}
      <div class="flex gap-2">
        <dt class="text-text-muted w-28 shrink-0">Your name</dt>
        <dd class="text-text-primary">{data.creatorName}</dd>
      </div>
      <div class="flex gap-2">
        <dt class="text-text-muted w-28 shrink-0">Voting closes</dt>
        <dd class="text-text-primary">
          {deadlineDate ? deadlineDate.toLocaleString('en-US', dtFmt) : '—'}
        </dd>
      </div>
    </dl>
  </Card>

  <!-- Dates summary -->
  <Card>
    <div class="flex items-start justify-between gap-2 mb-3">
      <h3 class="text-base font-semibold text-text-primary">Dates</h3>
      <button
        type="button"
        class="inline-flex items-center gap-1 text-sm text-action-primary hover:underline cursor-pointer"
        on:click={() => edit('dates')}
      >
        <Pencil size={14} aria-hidden="true" />
        Edit
      </button>
    </div>
    <dl class="space-y-2 text-sm">
      <div class="flex gap-2">
        <dt class="text-text-muted w-28 shrink-0">Range</dt>
        <dd class="text-text-primary">
          {#if data.dateRangeStart && data.dateRangeEnd}
            {data.dateRangeStart.toLocaleDateString('en-US', dateFmt)} – {data.dateRangeEnd.toLocaleDateString('en-US', dateFmt)}
          {:else}
            —
          {/if}
        </dd>
      </div>
      <div class="flex gap-2">
        <dt class="text-text-muted w-28 shrink-0">Days</dt>
        <dd class="text-text-primary">{dayCount} {dayCount === 1 ? 'day' : 'days'}</dd>
      </div>
      <div class="flex gap-2">
        <dt class="text-text-muted w-28 shrink-0">Specific times?</dt>
        <dd class="text-text-primary">{data.enableTimeSlots ? 'Yes' : 'No'}</dd>
      </div>
    </dl>
  </Card>

  <!-- Time slots summary (only when enabled) -->
  {#if data.enableTimeSlots}
    <Card>
      <div class="flex items-start justify-between gap-2 mb-3">
        <h3 class="text-base font-semibold text-text-primary">Time slots</h3>
        <button
          type="button"
          class="inline-flex items-center gap-1 text-sm text-action-primary hover:underline cursor-pointer"
          on:click={() => edit('timeslots')}
        >
          <Pencil size={14} aria-hidden="true" />
          Edit
        </button>
      </div>
      <dl class="space-y-2 text-sm">
        <div class="flex gap-2">
          <dt class="text-text-muted w-28 shrink-0">Timezone</dt>
          <dd class="text-text-primary">{data.timezone}</dd>
        </div>
        <div class="flex flex-col gap-1">
          <dt class="text-text-muted">Per-day slots</dt>
          <dd class="text-text-primary">
            <ul class="mt-1 space-y-1">
              {#each configuredDays as key (key)}
                {@const dayDate = new Date(key)}
                <li>
                  <span class="text-text-muted">{dayDate.toLocaleDateString('en-US', dateFmt)}:</span>
                  {data.dayTimeSlots[key].join(', ')}
                </li>
              {/each}
            </ul>
          </dd>
        </div>
      </dl>
    </Card>
  {/if}

  {#if error}
    <Card class="bg-error/10 border-error/50">
      <div class="flex items-center gap-2 text-error text-sm">
        <AlertCircle size={20} aria-hidden="true" class="flex-shrink-0" />
        <p>{error}</p>
      </div>
    </Card>
  {/if}

  <Button variant="primary" fullWidth disabled={loading} on:click={submit}>
    {#if loading}
      <Loader2 size={18} class="animate-spin" aria-hidden="true" />
      Creating…
    {:else}
      Create Wentu
    {/if}
  </Button>
</div>
