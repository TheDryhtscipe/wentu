<script>
  import { createEventDispatcher } from 'svelte';
  import { FileText, MessageSquare, User, CalendarClock } from 'lucide-svelte';
  import Input from '../../components/ui/Input.svelte';
  import Textarea from '../../components/ui/Textarea.svelte';
  import Select from '../../components/ui/Select.svelte';
  import WhatIsAWentu from '../../components/WhatIsAWentu.svelte';

  export let data;

  const dispatch = createEventDispatcher();

  const SHORTCUT_TODAY = 'today';
  const SHORTCUT_TOMORROW = 'tomorrow';
  const SHORTCUT_3_DAYS = '3days';
  const SHORTCUT_CUSTOM = 'custom';

  function formatYMD(d) {
    const y = d.getFullYear();
    const m = String(d.getMonth() + 1).padStart(2, '0');
    const day = String(d.getDate()).padStart(2, '0');
    return `${y}-${m}-${day}`;
  }

  function endOfToday() {
    return { date: formatYMD(new Date()), time: '23:59' };
  }
  function endOfTomorrow() {
    const d = new Date();
    d.setDate(d.getDate() + 1);
    return { date: formatYMD(d), time: '23:59' };
  }
  function endOfIn3Days() {
    const d = new Date();
    d.setDate(d.getDate() + 3);
    return { date: formatYMD(d), time: '23:59' };
  }

  // Default to "In 3 days EOD" if the shell hasn't populated prefDeadline yet.
  if (!data.prefDeadline) {
    const { date, time } = endOfIn3Days();
    data.prefDeadline = date;
    data.prefDeadlineTime = time;
  }

  let shortcut = SHORTCUT_3_DAYS;

  const timeOptions = (() => {
    const opts = [];
    for (let h = 0; h < 24; h++) {
      for (let m = 0; m < 60; m += 15) {
        const value = `${String(h).padStart(2, '0')}:${String(m).padStart(2, '0')}`;
        const d = new Date(2000, 0, 1, h, m);
        const label = d.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' });
        opts.push({ value, label });
      }
    }
    // Explicit end-of-day option so chips defaulting to 23:59 have a
    // matching <option> on the custom reveal.
    opts.push({ value: '23:59', label: '11:59 PM (end of day)' });
    return opts;
  })();

  function applyShortcut(which) {
    shortcut = which;
    if (which === SHORTCUT_TODAY) {
      const { date, time } = endOfToday();
      data.prefDeadline = date;
      data.prefDeadlineTime = time;
    } else if (which === SHORTCUT_TOMORROW) {
      const { date, time } = endOfTomorrow();
      data.prefDeadline = date;
      data.prefDeadlineTime = time;
    } else if (which === SHORTCUT_3_DAYS) {
      const { date, time } = endOfIn3Days();
      data.prefDeadline = date;
      data.prefDeadlineTime = time;
    }
    notifyChange();
  }

  function notifyChange() {
    dispatch('change');
  }

  $: deadlineIso = (() => {
    if (!data.prefDeadline || !data.prefDeadlineTime) return null;
    const d = new Date(`${data.prefDeadline}T${data.prefDeadlineTime}`);
    return isNaN(d.getTime()) ? null : d;
  })();
  $: deadlineInFuture = deadlineIso ? deadlineIso.getTime() > Date.now() : false;
  $: titleValid = data.title.trim().length > 0;
  $: creatorNameValid = data.creatorName.trim().length > 0;
  $: valid = titleValid && creatorNameValid && deadlineInFuture;

  // Surface validity to the shell so it can gate Next.
  $: dispatch('valid', valid);

  const shortcutChips = [
    { id: SHORTCUT_TODAY, label: 'End of today' },
    { id: SHORTCUT_TOMORROW, label: 'Tomorrow EOD' },
    { id: SHORTCUT_3_DAYS, label: 'In 3 days EOD' },
    { id: SHORTCUT_CUSTOM, label: 'Custom…' },
  ];
</script>

<WhatIsAWentu />

<div class="space-y-5">
  <div>
    <label for="basics-title" class="flex items-center gap-2 text-text-primary font-medium mb-2 text-sm sm:text-base">
      <FileText size={18} aria-hidden="true" />
      Title <span class="text-error" aria-hidden="true">*</span>
    </label>
    <Input
      id="basics-title"
      class="w-full"
      placeholder="Team offsite"
      bind:value={data.title}
      on:input={notifyChange}
      required
    />
  </div>

  <div>
    <label for="basics-description" class="flex items-center gap-2 text-text-primary font-medium mb-2 text-sm sm:text-base">
      <MessageSquare size={18} aria-hidden="true" />
      Description
    </label>
    <Textarea
      id="basics-description"
      class="w-full"
      placeholder="Optional details about the meeting"
      rows={3}
      bind:value={data.description}
      on:input={notifyChange}
    />
  </div>

  <div>
    <label for="basics-name" class="flex items-center gap-2 text-text-primary font-medium mb-2 text-sm sm:text-base">
      <User size={18} aria-hidden="true" />
      Your name <span class="text-error" aria-hidden="true">*</span>
    </label>
    <Input
      id="basics-name"
      class="w-full"
      placeholder="Alice"
      bind:value={data.creatorName}
      on:input={notifyChange}
      required
    />
  </div>

  <div>
    <label class="flex items-center gap-2 text-text-primary font-medium mb-2 text-sm sm:text-base">
      <CalendarClock size={18} aria-hidden="true" />
      Voting closes <span class="text-error" aria-hidden="true">*</span>
    </label>
    <p class="text-text-secondary text-xs sm:text-sm mb-2">
      Participants can edit their ranking until this date and time.
    </p>

    <div class="flex flex-wrap gap-2 mb-3" role="group" aria-label="Voting closes shortcuts">
      {#each shortcutChips as chip (chip.id)}
        {@const active = shortcut === chip.id}
        <button
          type="button"
          class="rounded-full border px-3 py-1 text-sm transition-colors cursor-pointer {active ? 'border-action-primary bg-action-primary text-dark-bg' : 'border-border-strong text-text-primary hover:bg-action-secondary-hover'}"
          aria-pressed={active}
          on:click={() => applyShortcut(chip.id)}
        >
          {chip.label}
        </button>
      {/each}
    </div>

    {#if shortcut === SHORTCUT_CUSTOM}
      <div class="flex flex-col sm:flex-row gap-2">
        <Input
          class="flex-1"
          type="date"
          bind:value={data.prefDeadline}
          on:change={notifyChange}
          aria-label="Voting-closes date"
        />
        <Select
          class="w-full sm:w-auto"
          bind:value={data.prefDeadlineTime}
          on:change={notifyChange}
          aria-label="Voting-closes time"
        >
          {#each timeOptions as opt (opt.value)}
            <option value={opt.value}>{opt.label}</option>
          {/each}
        </Select>
      </div>
    {/if}

    {#if !deadlineInFuture && deadlineIso}
      <p class="mt-2 text-xs text-error">Voting-closes time must be in the future.</p>
    {/if}
  </div>
</div>
