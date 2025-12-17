<script>
  import { createEventDispatcher, onMount } from 'svelte';
  import { ArrowLeft, Loader2, AlertCircle, Lock, Copy, CheckCircle } from 'lucide-svelte';
  import DragDropPreferences from '../components/DragDropPreferences.svelte';
  import STVResults from '../components/STVResults.svelte';
  import ExpiryTimer from '../components/ExpiryTimer.svelte';
  import { api } from '../lib/api.js';

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

  $: deadlineReached = wentu && new Date() > new Date(wentu.pref_deadline);

  async function copyToClipboard() {
    try {
      await navigator.clipboard.writeText(wentu.slug);
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
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function joinWentu() {
    if (!participantName.trim()) {
      error = 'Please enter your name';
      return;
    }

    try {
      const data = await api.post(`/api/wentu/${slug}/join`, { name: participantName });
      participantId = data.participant_id;
      participantKey = data.participant_key;
      showJoinForm = false;
    } catch (err) {
      error = err.message;
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
</script>

<div class="max-w-4xl mx-auto">
  <button class="mb-6 text-accent hover:underline flex items-center gap-1" on:click={goHome}>
    <ArrowLeft size={16} />
    Back to home
  </button>

  {#if loading}
    <div class="card text-center">
      <div class="flex items-center justify-center gap-2 text-text-secondary">
        <Loader2 size={20} class="animate-spin" />
        <p>Loading...</p>
      </div>
    </div>
  {:else if error && !wentu}
    <div class="card bg-error/10 border-error/50">
      <div class="flex items-center gap-2 text-error">
        <AlertCircle size={20} />
        <p>{error}</p>
      </div>
    </div>
  {:else if wentu}
    <div class="card mb-6">
      <div class="flex justify-between items-start mb-4">
        <div>
          <h2 class="text-3xl font-bold text-accent">{wentu.title}</h2>
          {#if wentu.description}
            <p class="text-text-secondary mt-2">{wentu.description}</p>
          {/if}
          <p class="text-text-secondary text-sm mt-2">Created by {wentu.creator_name}</p>
        </div>
        <ExpiryTimer expiresAt={new Date(wentu.expires_at)} />
      </div>

      <div class="bg-dark-bg rounded p-3 mb-4">
        <p class="text-text-secondary text-sm mb-2">Share code:</p>
        <div class="flex items-center justify-between gap-3">
          <p class="text-accent font-mono text-lg">{wentu.slug}</p>
          <button
            on:click={copyToClipboard}
            class="btn-secondary px-3 py-2 flex items-center gap-2 text-sm"
            title="Copy to clipboard"
          >
            {#if copied}
              <CheckCircle size={16} class="text-success" />
              <span class="text-success">Copied!</span>
            {:else}
              <Copy size={16} />
              Copy
            {/if}
          </button>
        </div>
      </div>

      <p class="text-text-secondary text-sm">Status: <span class="text-accent font-medium">{wentu.status}</span></p>
      
      <div class="grid grid-cols-2 gap-4 mt-4 text-sm">
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
      <div class="card mb-6">
        <h3 class="text-xl font-bold text-accent mb-4">Join as participant</h3>
        <input
          class="input w-full mb-4"
          type="text"
          placeholder="Your name"
          bind:value={participantName}
          aria-label="Your name"
        />
        {#if error}
          <p class="text-error text-sm mb-4">{error}</p>
        {/if}
        <button class="btn-primary w-full" on:click={joinWentu}>Join Wentu</button>
      </div>
    {:else}
      <div class="card mb-6">
        <h3 class="text-xl font-bold text-accent mb-4">Your preferences</h3>
        {#if deadlineReached}
          <div class="flex items-center gap-2 text-error text-sm mb-4">
            <Lock size={16} />
            <p>Preference deadline has passed. You can no longer edit.</p>
          </div>
        {:else}
          <p class="text-text-secondary text-sm mb-4">Drag to order dates by preference</p>
        {/if}
        <DragDropPreferences
          bind:items={preferences}
          disabled={deadlineReached}
          on:remove={(e) => removePreference(e.detail.id)}
        />

        {#if preferences.length === 0 && removedPreferences.length > 0}
          <div class="bg-accent/10 border border-accent/30 rounded p-4 my-4">
            <p class="text-text-secondary text-sm">
              All dates removed. You can restore dates below or submit with no preferences.
            </p>
          </div>
        {/if}

        {#if removedPreferences.length > 0 && !deadlineReached}
          <div class="mt-6 pt-6 border-t border-accent/20">
            <h4 class="text-lg font-semibold text-text-secondary mb-3">
              Removed dates ({removedPreferences.length})
            </h4>
            <div class="space-y-2">
              {#each removedPreferences as removed}
                <div class="bg-dark-bg/50 p-3 rounded flex items-center justify-between opacity-60">
                  <div class="flex-1">
                    <p class="text-text-secondary font-medium">{removed.label}</p>
                    <p class="text-text-secondary/70 text-sm">
                      {new Date(removed.start).toLocaleDateString()} –
                      {new Date(removed.end).toLocaleDateString()}
                    </p>
                  </div>
                  <button
                    on:click={() => restorePreference(removed.id)}
                    class="btn-secondary text-sm px-3 py-1.5"
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
          <div class="flex items-center gap-2 text-error text-sm mt-4">
            <AlertCircle size={16} />
            <p>{error}</p>
          </div>
        {/if}

        {#if !deadlineReached}
          <button class="btn-primary w-full mt-4" on:click={submitPreferences}>
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
