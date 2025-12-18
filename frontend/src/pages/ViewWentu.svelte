<script>
  import { createEventDispatcher, onMount } from 'svelte';
  import { ArrowLeft, Loader2, AlertCircle, Lock, Copy, CheckCircle } from 'lucide-svelte';
  import DragDropPreferences from '../components/DragDropPreferences.svelte';
  import Calendar from '../components/Calendar.svelte';
  import TimeOrderingModal from '../components/TimeOrderingModal.svelte';
  import STVResults from '../components/STVResults.svelte';
  import ExpiryTimer from '../components/ExpiryTimer.svelte';
  import { api } from '../lib/api.js';
  import { addTrackedWentu, getTrackedWentu } from '../lib/wentuTracker.js';

  const dispatch = createEventDispatcher();

  export let slug = '';
  export let creatorName = '';
  export let creatorKey = '';
  export let creatorParticipantId = '';
  export let creatorParticipantKey = '';

  let wentu = null;
  let loading = true;
  let error = '';
  let participantId = '';
  let participantKey = '';
  let participantName = '';
  let showJoinForm = true;
  let preferences = [];
  let removedPreferences = [];
  let lastAction = '';
  let stvResults = null;
  let copied = false;
  
  // Preferences calendar
  let preferenceSelections = [];  // Array of { date/dateStart, dateEnd?, order }
  let showPreferencesCalendar = false;

  // Time ordering modal state
  let showTimeOrderingModal = false;
  let modalTimeSlots = [];
  let modalDateLabel = '';
  let pendingDateKey = '';

  // Auto-show calendar when there are >5 options
  $: if (preferences.length > 5) {
    showPreferencesCalendar = true;
  }

  // Group date options by calendar day for detecting multi-time dates
  $: dateOptionsByDay = (() => {
    const groups = new Map();
    for (const option of wentu?.date_options || []) {
      const startDate = new Date(option.start);
      const year = startDate.getUTCFullYear();
      const month = String(startDate.getUTCMonth() + 1).padStart(2, '0');
      const day = String(startDate.getUTCDate()).padStart(2, '0');
      const dateKey = `${year}-${month}-${day}`;

      if (!groups.has(dateKey)) {
        groups.set(dateKey, []);
      }
      groups.get(dateKey).push(option);
    }
    return groups;
  })();

  // Calendar starts empty - user clicks to reorder preferences
  // preferenceSelections will be populated by user clicks via the calendar

  $: deadlineReached = wentu && new Date() > new Date(wentu.pref_deadline);

  async function copyToClipboard() {
    try {
      const url = `${window.location.origin}/wentu/${wentu.slug}`;
      await navigator.clipboard.writeText(url);
      copied = true;
      setTimeout(() => copied = false, 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  }

  function loadRemovedPreferences() {
    try {
      const stored = localStorage.getItem(`wentu-removed-${slug}`);
      if (stored) {
        removedPreferences = JSON.parse(stored);
        preferences = preferences.filter(p =>
          !removedPreferences.some(r => r.id === p.id)
        );
      }
    } catch (err) {
      console.error('Failed to load removed preferences:', err);
    }
  }

  function saveRemovedPreferences() {
    try {
      localStorage.setItem(
        `wentu-removed-${slug}`,
        JSON.stringify(removedPreferences)
      );
    } catch (err) {
      console.error('Failed to save removed preferences:', err);
    }
  }

  onMount(async () => {
    await loadWentu();
  });

  async function loadWentu() {
    try {
      wentu = await api.get(`/api/wentu/${slug}`);
      preferences = wentu.date_options.map((d, i) => ({ ...d, order: i }));

      // Load removed preferences from localStorage
      loadRemovedPreferences();

      // Auto-login creator if we have creator credentials
      if (
        creatorName &&
        creatorKey &&
        creatorName === wentu.creator_name &&
        creatorParticipantId &&
        creatorParticipantKey
      ) {
        participantName = creatorName;
        participantId = creatorParticipantId;
        participantKey = creatorParticipantKey;
        showJoinForm = false;
      }

      const tracked = getTrackedWentu(slug);
      if (tracked) {
        if (tracked.name && !participantName) {
          participantName = tracked.name;
        }
        if (tracked.participantId && tracked.participantKey) {
          participantId = tracked.participantId;
          participantKey = tracked.participantKey;
          showJoinForm = false;
        } else if (tracked.name && !participantId && !participantKey) {
          await joinWentu({ silent: true });
        }
      }
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function joinWentu({ silent } = {}) {
    if (!participantName.trim()) {
      if (!silent) {
        error = 'Please enter your name';
      }
      return;
    }

    try {
      const data = await api.post(`/api/wentu/${slug}/join`, { name: participantName });
      participantId = data.participant_id;
      participantKey = data.participant_key;

      // Track participation in localStorage
      addTrackedWentu(slug, wentu.title, 'participant', participantName, participantId, participantKey);

      showJoinForm = false;
    } catch (err) {
      if (!silent) {
        error = err.message;
      }
    }
  }

  async function submitPreferences() {
    try {
      const rankings = preferences.map((p, idx) => ({
        date_option_id: p.id,
        preference_order: idx + 1,
      }));

      await api.post(`/api/wentu/${slug}/preferences`, {
        participant_id: participantId,
        participant_key: participantKey,
        rankings,
      });

      error = '';
      await loadSTVResults();
    } catch (err) {
      if (err.message && err.message.startsWith('HTTP 401') && participantName.trim()) {
        try {
          await joinWentu({ silent: true });
          await api.post(`/api/wentu/${slug}/preferences`, {
            participant_id: participantId,
            participant_key: participantKey,
            rankings: preferences.map((p, idx) => ({
              date_option_id: p.id,
              preference_order: idx + 1,
            })),
          });
          error = '';
          await loadSTVResults();
          return;
        } catch (retryErr) {
          error = retryErr.message;
          return;
        }
      }
      error = err.message;
    }
  }

  async function loadSTVResults() {
    try {
      stvResults = await api.get(`/api/wentu/${slug}/stv-results`);
    } catch (err) {
      console.error('Failed to load STV results:', err);
    }
  }

  function removePreference(id) {
    const toRemove = preferences.find(p => p.id === id);
    if (!toRemove) return;

    preferences = preferences.filter(p => p.id !== id);
    removedPreferences = [...removedPreferences, toRemove];
    saveRemovedPreferences();

    lastAction = `${toRemove.label} removed from preferences. ${preferences.length} dates remaining.`;
  }

  function restorePreference(id) {
    const toRestore = removedPreferences.find(r => r.id === id);
    if (!toRestore) return;

    removedPreferences = removedPreferences.filter(r => r.id !== id);
    preferences = [...preferences, toRestore];
    saveRemovedPreferences();

    lastAction = `${toRestore.label} restored to preferences.`;
  }

  function goHome() {
    dispatch('navigate', { page: 'home' });
  }

  function handlePreferencesChange(event) {
    const calendarSelections = event.detail.selectedDates;

    // Find the most recently clicked date (highest order number)
    const latestSelection = calendarSelections
      .filter(s => s.date)
      .sort((a, b) => b.order - a.order)[0];

    if (!latestSelection) {
      // No new single-date selection, just sync state
      preferenceSelections = [...calendarSelections];
      syncPreferencesFromSelections(calendarSelections);
      return;
    }

    const clickedDateKey = latestSelection.date;
    const timeSlotsForDate = dateOptionsByDay.get(clickedDateKey) || [];

    // Filter out already-removed time slots
    const removedIds = new Set(removedPreferences.map(r => r.id));
    const availableTimeSlots = timeSlotsForDate.filter(t => !removedIds.has(t.id));

    if (availableTimeSlots.length > 1) {
      // Multiple time slots available - open modal
      // Remove the clicked date from selections since user hasn't confirmed yet
      const updatedSelections = calendarSelections.filter(s => s.date !== clickedDateKey);
      preferenceSelections = [...updatedSelections];
      openTimeOrderingModal(clickedDateKey, availableTimeSlots);
    } else {
      // Single time slot (or none available) - add directly (current behavior)
      preferenceSelections = [...calendarSelections];
      syncPreferencesFromSelections(calendarSelections);
    }
  }

  function syncPreferencesFromSelections(calendarSelections) {
    // Get clicked dates in click order
    const clickedDatesInOrder = calendarSelections
      .sort((a, b) => a.order - b.order)
      .flatMap((selection) => {
        if (selection.date) {
          return wentu.date_options.filter(d => {
            const startDate = new Date(d.start);
            const year = startDate.getUTCFullYear();
            const month = String(startDate.getUTCMonth() + 1).padStart(2, '0');
            const day = String(startDate.getUTCDate()).padStart(2, '0');
            const dateKey = `${year}-${month}-${day}`;
            return dateKey === selection.date;
          });
        }
        return [];
      });

    // Get clicked date IDs
    const clickedIds = new Set(clickedDatesInOrder.map(d => d.id));

    // Get unclicked dates in their original order (excluding removed ones)
    const removedIds = new Set(removedPreferences.map(r => r.id));
    const unclickedDates = wentu.date_options.filter(d =>
      !clickedIds.has(d.id) && !removedIds.has(d.id)
    );

    // Combine: clicked dates in click order, then unclicked dates
    preferences = [...clickedDatesInOrder, ...unclickedDates];
  }

  function openTimeOrderingModal(dateKey, timeSlots) {
    pendingDateKey = dateKey;
    modalTimeSlots = timeSlots;

    // Create human-readable date label
    const [year, month, day] = dateKey.split('-').map(Number);
    const date = new Date(Date.UTC(year, month - 1, day));
    modalDateLabel = date.toLocaleDateString('en-US', {
      weekday: 'long',
      month: 'long',
      day: 'numeric',
      timeZone: 'UTC'
    });

    showTimeOrderingModal = true;
  }

  function handleTimeOrderingConfirm(event) {
    const { orderedTimeSlots, removedTimeSlots } = event.detail;

    // Get IDs of slots being added or removed
    const slotsToAddIds = new Set(orderedTimeSlots.map(s => s.id));
    const slotsToRemoveIds = new Set(removedTimeSlots.map(s => s.id));

    // Remove from preferences any slots that are being added or removed
    preferences = preferences.filter(p =>
      !slotsToAddIds.has(p.id) && !slotsToRemoveIds.has(p.id)
    );

    // Add ordered slots to the beginning (top of list)
    preferences = [...orderedTimeSlots, ...preferences];

    // Add removed slots to removedPreferences
    if (removedTimeSlots.length > 0) {
      removedPreferences = [...removedPreferences, ...removedTimeSlots];
      saveRemovedPreferences();
    }

    // Update calendar selections to include this date
    const maxOrder = Math.max(...preferenceSelections.map(s => s.order), 0);
    preferenceSelections = [...preferenceSelections, { date: pendingDateKey, order: maxOrder + 1 }];

    closeTimeOrderingModal();
  }

  function handleTimeOrderingCancel() {
    // Don't add this date to calendar selections - just close modal
    closeTimeOrderingModal();
  }

  function closeTimeOrderingModal() {
    showTimeOrderingModal = false;
    modalTimeSlots = [];
    modalDateLabel = '';
    pendingDateKey = '';
  }


</script>

<div class="max-w-4xl mx-auto px-4 sm:px-0">
  <button class="mb-4 sm:mb-6 text-accent hover:underline flex items-center gap-1 text-sm sm:text-base" on:click={goHome}>
    <ArrowLeft size={16} />
    Back to home
  </button>

  {#if loading}
    <div class="card text-center">
      <div class="flex items-center justify-center gap-2 text-text-secondary text-sm sm:text-base">
        <Loader2 size={20} class="animate-spin" />
        <p>Loading...</p>
      </div>
    </div>
  {:else if error && !wentu}
    <div class="card bg-error/10 border-error/50">
      <div class="flex items-center gap-2 text-error text-sm">
        <AlertCircle size={20} class="flex-shrink-0" />
        <p>{error}</p>
      </div>
    </div>
  {:else if wentu}
    <div class="card mb-4 sm:mb-6">
      <div class="flex flex-col sm:flex-row justify-between items-start mb-3 sm:mb-4 gap-3 sm:gap-0">
        <div class="flex-1">
          <h2 class="text-2xl sm:text-3xl font-bold text-accent">{wentu.title}</h2>
          {#if wentu.description}
            <p class="text-text-secondary text-sm sm:text-base mt-2">{wentu.description}</p>
          {/if}
          <p class="text-text-secondary text-xs sm:text-sm mt-2">Created by {wentu.creator_name}</p>
        </div>
        <ExpiryTimer expiresAt={new Date(wentu.expires_at)} />
      </div>

      <div class="bg-dark-bg rounded p-3 mb-3 sm:mb-4">
        <p class="text-text-secondary text-xs sm:text-sm mb-2">Share this link with participants:</p>
        <div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-2 sm:gap-3">
          <div class="flex-1 bg-content-bg rounded p-2 sm:p-3 border border-accent/30">
            <p class="text-accent font-mono text-sm sm:text-base break-all select-all">{window.location.origin}/wentu/{wentu.slug}</p>
          </div>
          <button
            on:click={copyToClipboard}
            class="btn-secondary px-3 py-2 flex items-center justify-center gap-2 text-sm flex-shrink-0"
            title="Copy link to clipboard"
          >
            {#if copied}
              <CheckCircle size={16} class="text-success" />
              <span class="text-success">Copied!</span>
            {:else}
              <Copy size={16} />
              <span>Copy link</span>
            {/if}
          </button>
        </div>
      </div>

      <p class="text-text-secondary text-xs sm:text-sm mb-3 sm:mb-4">
        Status: <span class="text-accent font-medium">{wentu.status}</span>
      </p>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 sm:gap-4 text-xs sm:text-sm">
        <div>
          <p class="text-text-secondary">Pref. deadline:</p>
          <p class="text-accent">{new Date(wentu.pref_deadline).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })}</p>
        </div>
        <div>
          <p class="text-text-secondary">Expires:</p>
          <p class="text-accent">{new Date(wentu.expires_at).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })}</p>
        </div>
      </div>
    </div>

    {#if showJoinForm}
      <div class="card mb-4 sm:mb-6">
        <h3 class="text-lg sm:text-xl font-bold text-accent mb-3 sm:mb-4">Join as participant</h3>
        <input
          class="input w-full mb-3 sm:mb-4"
          type="text"
          placeholder="Your name"
          bind:value={participantName}
          aria-label="Your name"
        />
        {#if error}
          <p class="text-error text-xs sm:text-sm mb-3 sm:mb-4">{error}</p>
        {/if}
        <button class="btn-primary w-full" on:click={joinWentu}>Join Wentu</button>
      </div>
    {:else}
      <div class="card mb-4 sm:mb-6">
        <h3 class="text-lg sm:text-xl font-bold text-accent mb-3 sm:mb-4">Your preferences</h3>
        {#if deadlineReached}
          <div class="flex items-center gap-2 text-error text-xs sm:text-sm mb-3 sm:mb-4">
            <Lock size={16} class="flex-shrink-0" />
            <p>Preference deadline has passed. You can no longer edit.</p>
          </div>
        {:else}
          <p class="text-text-secondary text-xs sm:text-sm mb-3 sm:mb-4">Drag to order dates by preference, or use the calendar below</p>
        {/if}

        <!-- Preferences Calendar Drawer -->
        <div class="bg-content-bg rounded p-3 sm:p-4 mb-3 sm:mb-4 border border-accent/20">
          <button 
            on:click={() => showPreferencesCalendar = !showPreferencesCalendar}
            disabled={deadlineReached}
            class="w-full text-left font-medium text-accent hover:text-accent/80 transition-colors flex items-center justify-between disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span>Reorder by clicking dates on calendar (optional)</span>
            <span class="text-sm">{showPreferencesCalendar ? '▼' : '▶'}</span>
          </button>
            
          {#if showPreferencesCalendar && !deadlineReached}
            <div class="mt-4 pt-4 border-t border-accent/20">
              <p class="text-text-secondary text-xs sm:text-sm mb-3 sm:mb-4">Click dates in order of preference. Each click moves that date to the end of your list. Drag the list below to reorder further.</p>
              <Calendar 
                mode="preferences"
                selectedDates={preferenceSelections}
                availableDateOptions={wentu.date_options}
                on:preferenceschange={handlePreferencesChange}
              />
            </div>
          {/if}
        </div>

        <DragDropPreferences
          bind:items={preferences}
          disabled={deadlineReached}
          on:remove={(e) => removePreference(e.detail.id)}
        />

        {#if preferences.length === 0 && removedPreferences.length > 0}
          <div class="bg-accent/10 border border-accent/30 rounded p-3 sm:p-4 my-3 sm:my-4">
            <p class="text-text-secondary text-xs sm:text-sm">
              All dates removed. You can restore dates below or submit with no preferences.
            </p>
          </div>
        {/if}

        {#if removedPreferences.length > 0 && !deadlineReached}
          <div class="mt-4 sm:mt-6 pt-4 sm:pt-6 border-t border-accent/20">
            <h4 class="text-base sm:text-lg font-semibold text-text-secondary mb-3">
              Removed dates ({removedPreferences.length})
            </h4>
            <div class="space-y-2">
              {#each removedPreferences as removed}
                <div class="bg-dark-bg/50 p-3 rounded flex items-center justify-between opacity-60 gap-2">
                  <div class="flex-1 min-w-0">
                    <p class="text-text-secondary font-medium text-sm sm:text-base truncate">{removed.label}</p>
                    <p class="text-text-secondary/70 text-xs sm:text-sm">
                      {new Date(removed.start).toLocaleDateString()} –
                      {new Date(removed.end).toLocaleDateString()}
                    </p>
                  </div>
                  <button
                    on:click={() => restorePreference(removed.id)}
                    class="btn-secondary text-xs sm:text-sm px-2 sm:px-3 py-1.5 flex-shrink-0"
                    aria-label="Restore {removed.label} to preferences"
                  >
                    Restore
                  </button>
                </div>
              {/each}
            </div>
          </div>
        {/if}

        <div class="sr-only" role="status" aria-live="polite" aria-atomic="true">
          {lastAction}
        </div>

        {#if error}
          <div class="flex items-center gap-2 text-error text-xs sm:text-sm mt-3 sm:mt-4">
            <AlertCircle size={16} class="flex-shrink-0" />
            <p>{error}</p>
          </div>
        {/if}

        {#if !deadlineReached}
          <button class="btn-primary w-full mt-3 sm:mt-4" on:click={submitPreferences}>
            Submit preferences
          </button>
        {/if}
      </div>

      {#if stvResults}
        <STVResults results={stvResults} {wentu} />
      {/if}
    {/if}
  {/if}
</div>

{#if showTimeOrderingModal}
  <TimeOrderingModal
    timeSlots={modalTimeSlots}
    dateLabel={modalDateLabel}
    on:confirm={handleTimeOrderingConfirm}
    on:cancel={handleTimeOrderingCancel}
  />
{/if}
