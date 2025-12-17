<script>
  import { createEventDispatcher } from 'svelte';
  import { ArrowLeft, Loader2, AlertCircle, FileText, MessageSquare, User, Calendar as CalendarIcon, Copy } from 'lucide-svelte';
  import Calendar from '../components/Calendar.svelte';
  import TimeSlotConfigurator from '../components/TimeSlotConfigurator.svelte';
  import TimezonePicker from '../components/TimezonePicker.svelte';

  const dispatch = createEventDispatcher();

  let title = '';
  let description = '';
  let creatorName = '';
  let dateRangeStart = null;
  let dateRangeEnd = null;
  let loading = false;
  let error = '';

  // Time slot configuration
  let enableTimeSlots = false;
  let timezone = 'Europe/London';
  let dayTimeSlots = {};  // { "2025-12-20": ["10:00", "13:00"] }
  let showCopyOptions = false;
  
  // Initialize deadline to 1 day from now
  function getDefaultDeadline() {
    const tomorrow = new Date();
    tomorrow.setDate(tomorrow.getDate() + 1);
    const y = tomorrow.getFullYear();
    const m = String(tomorrow.getMonth() + 1).padStart(2, '0');
    const d = String(tomorrow.getDate()).padStart(2, '0');
    return `${y}-${m}-${d}`;
  }
  
  let prefDeadline = getDefaultDeadline();

  function setDateRange(start, end) {
    dateRangeStart = start;
    dateRangeEnd = end;
    error = '';
  }

  function formatDate(date) {
    const y = date.getFullYear();
    const m = String(date.getMonth() + 1).padStart(2, '0');
    const d = String(date.getDate()).padStart(2, '0');
    return `${y}-${m}-${d}`;
  }

  function getDaysInRange() {
    if (!dateRangeStart || !dateRangeEnd) return [];
    const days = [];
    const current = new Date(dateRangeStart);
    const end = new Date(dateRangeEnd);

    while (current <= end) {
      days.push(new Date(current));
      current.setDate(current.getDate() + 1);
    }
    return days;
  }

  function handleTimeSlotChange(day, slots) {
    const key = formatDate(day);
    if (slots && slots.length > 0) {
      dayTimeSlots[key] = slots;
    } else {
      delete dayTimeSlots[key];
    }
    dayTimeSlots = { ...dayTimeSlots };
  }

  function getFirstConfiguredDate() {
    const keys = Object.keys(dayTimeSlots).sort();
    if (keys.length === 0) return '';
    const date = new Date(keys[0]);
    return date.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
  }

  function copyToAllDays() {
    const firstKey = Object.keys(dayTimeSlots).sort()[0];
    if (!firstKey) return;
    const slots = dayTimeSlots[firstKey];
    getDaysInRange().forEach(day => {
      dayTimeSlots[formatDate(day)] = [...slots];
    });
    dayTimeSlots = { ...dayTimeSlots };
    showCopyOptions = false;
  }

  function copyToWeekdays() {
    const firstKey = Object.keys(dayTimeSlots).sort()[0];
    if (!firstKey) return;
    const slots = dayTimeSlots[firstKey];
    getDaysInRange().forEach(day => {
      const dayOfWeek = day.getDay();
      if (dayOfWeek >= 1 && dayOfWeek <= 5) {
        dayTimeSlots[formatDate(day)] = [...slots];
      }
    });
    dayTimeSlots = { ...dayTimeSlots };
    showCopyOptions = false;
  }

  function copyToWeekends() {
    const firstKey = Object.keys(dayTimeSlots).sort()[0];
    if (!firstKey) return;
    const slots = dayTimeSlots[firstKey];
    getDaysInRange().forEach(day => {
      const dayOfWeek = day.getDay();
      if (dayOfWeek === 0 || dayOfWeek === 6) {
        dayTimeSlots[formatDate(day)] = [...slots];
      }
    });
    dayTimeSlots = { ...dayTimeSlots };
    showCopyOptions = false;
  }

  async function submitCreate() {
    console.log('submitCreate called', { title, creatorName, dateRangeStart, dateRangeEnd, prefDeadline, enableTimeSlots, timezone, dayTimeSlots });

    if (!title || !creatorName || !dateRangeStart || !dateRangeEnd || !prefDeadline) {
      error = 'Title, name, date range, and preference deadline required';
      console.log('Validation failed:', { title, creatorName, dateRangeStart, dateRangeEnd, prefDeadline });
      return;
    }

    // Validation for time slots
    if (enableTimeSlots) {
      const days = getDaysInRange();
      const missingDays = days.filter(day => {
        const key = formatDate(day);
        return !dayTimeSlots[key] || dayTimeSlots[key].length === 0;
      });

      if (missingDays.length > 0) {
        error = `Please configure time slots for all selected days. Missing: ${missingDays.map(d => d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })).join(', ')}`;
        return;
      }
    }

    loading = true;
    error = '';

    try {
      const body = {
        title,
        description: description || null,
        creator_name: creatorName,
        date_range_start: dateRangeStart.toISOString(),
        date_range_end: dateRangeEnd.toISOString(),
        pref_deadline: new Date(prefDeadline).toISOString(),
        expires_in_days: 7,
        enable_time_slots: enableTimeSlots || null,
        timezone: enableTimeSlots ? timezone : null,
        day_time_slots: enableTimeSlots ? dayTimeSlots : null,
      };
      console.log('Creating wentu with:', body);
      
      const response = await fetch('http://127.0.0.1:3000/api/wentu', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });

      if (!response.ok) {
        const errData = await response.json().catch(() => ({}));
        console.error('Create failed:', response.status, errData);
        throw new Error(errData.error || 'Failed to create wentu');
      }

      const data = await response.json();
      console.log('Wentu created:', data);
      dispatch('navigate', { 
        page: 'view', 
        params: { 
          slug: data.slug,
          creatorName: creatorName,
          creatorKey: data.creator_key,
          creatorParticipantId: data.creator_participant_id,
          creatorParticipantKey: data.creator_participant_key,
        } 
      });
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  function goHome() {
    dispatch('navigate', { page: 'home' });
  }
</script>

<div class="max-w-4xl mx-auto">
  <button class="mb-6 text-accent hover:underline flex items-center gap-1" on:click={goHome}>
    <ArrowLeft size={16} />
    Back to home
  </button>

  <h2 class="text-3xl font-bold text-accent mb-8">Create a new Wentu</h2>

  <div class="card mb-6">
    <div class="mb-6">
      <label class="flex items-center gap-2 text-text-primary font-medium mb-2">
        <FileText size={18} />
        Title <span class="text-error">*</span>
      </label>
      <input class="input w-full" type="text" placeholder="Team offsite, Q1 planning, etc." bind:value={title} />
    </div>

    <div class="mb-6">
      <label class="flex items-center gap-2 text-text-primary font-medium mb-2">
        <MessageSquare size={18} />
        Description
      </label>
      <textarea
        class="input w-full"
        placeholder="Optional details about the meeting"
        rows="3"
        bind:value={description}
      />
    </div>

    <div class="mb-6">
      <label class="flex items-center gap-2 text-text-primary font-medium mb-2">
        <User size={18} />
        Your name <span class="text-error">*</span>
      </label>
      <input class="input w-full" type="text" placeholder="Alice" bind:value={creatorName} />
    </div>

    <div class="mb-6">
      <label class="flex items-center gap-2 text-text-primary font-medium mb-2">
        <CalendarIcon size={18} />
        Preference deadline <span class="text-error">*</span>
      </label>
      <p class="text-text-secondary text-sm mb-2">Participants can edit their preferences until this date</p>
      <input class="input w-full" type="date" bind:value={prefDeadline} />
    </div>
  </div>

  <!-- Calendar -->
  <div class="card mb-6">
    <h3 class="text-xl font-bold text-accent mb-4">Date range</h3>
    <p class="text-text-secondary text-sm mb-4">Select the date range. Participants will vote on which individual days they prefer within this range.</p>
    <Calendar on:daterange={(e) => setDateRange(e.detail.start, e.detail.end)} />
  </div>

  <!-- Selected range -->
  {#if dateRangeStart && dateRangeEnd}
    <div class="card mb-6">
      <h3 class="text-lg font-bold text-accent mb-4">Selected date range</h3>
      <div class="bg-dark-bg p-3 rounded flex justify-between items-center">
        <p class="text-text-primary font-medium">
          {dateRangeStart.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })} - {dateRangeEnd.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })}
        </p>
        <button
          on:click={() => { dateRangeStart = null; dateRangeEnd = null; }}
          class="text-error hover:text-accent text-sm px-3 py-1 bg-error/10 rounded"
        >
          Clear
        </button>
      </div>
    </div>
  {/if}

  <!-- Time slot configuration checkbox -->
  <div class="card mb-6">
    <label class="flex items-center gap-3 cursor-pointer">
      <input type="checkbox" class="w-5 h-5" bind:checked={enableTimeSlots} />
      <div>
        <span class="text-text-primary font-medium">Configure specific time slots</span>
        <p class="text-text-secondary text-sm">Add up to 3 start times per day (e.g., 10am, 1pm, 7pm)</p>
      </div>
    </label>
  </div>

  {#if enableTimeSlots && dateRangeStart && dateRangeEnd}
    <!-- Timezone picker -->
    <div class="card mb-6">
      <TimezonePicker bind:selectedTimezone={timezone} on:change={(e) => timezone = e.detail} />
    </div>

    <!-- Time slot configuration -->
    <div class="card mb-6">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-xl font-bold text-accent">Configure time slots</h3>
        {#if Object.keys(dayTimeSlots).length > 0}
          <button class="btn-secondary text-sm" on:click={() => showCopyOptions = !showCopyOptions}>
            <Copy size={16} class="inline mr-1" />
            Copy times
          </button>
        {/if}
      </div>

      <!-- Copy options -->
      {#if showCopyOptions && Object.keys(dayTimeSlots).length > 0}
        <div class="bg-dark-bg p-4 rounded mb-4 border border-accent/30">
          <p class="text-text-secondary text-sm mb-3">Copy times from {getFirstConfiguredDate()} to:</p>
          <div class="flex gap-2 flex-wrap">
            <button class="btn-secondary text-sm" on:click={copyToAllDays}>All selected days</button>
            <button class="btn-secondary text-sm" on:click={copyToWeekdays}>Weekdays only (Mon-Fri)</button>
            <button class="btn-secondary text-sm" on:click={copyToWeekends}>Weekends only (Sat-Sun)</button>
          </div>
        </div>
      {/if}

      <div class="space-y-4">
        {#each getDaysInRange() as day}
          <TimeSlotConfigurator
            date={day}
            bind:timeSlots={dayTimeSlots[formatDate(day)]}
            on:change={(e) => handleTimeSlotChange(day, e.detail)}
          />
        {/each}
      </div>
    </div>
  {/if}

  {#if error}
    <div class="card bg-error/10 border-error/50 mb-6">
      <div class="flex items-center gap-2 text-error">
        <AlertCircle size={20} />
        <p>{error}</p>
      </div>
    </div>
  {/if}

  <div class="flex gap-4">
    <button class="btn-secondary flex-1" on:click={goHome}>Cancel</button>
    <button
      class="btn-primary flex-1"
      on:click={submitCreate}
      disabled={loading || !title || !creatorName || !dateRangeStart || !dateRangeEnd || !prefDeadline}
    >
      {#if loading}
        <span class="flex items-center justify-center gap-2">
          <Loader2 size={18} class="animate-spin" />
          Creating...
        </span>
      {:else}
        Create Wentu
      {/if}
    </button>
  </div>
</div>
