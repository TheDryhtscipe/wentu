<script>
  import { onMount } from 'svelte';
  import { XCircle, Info, ChevronDown, ChevronRight, Users } from 'lucide-svelte';
  import { api } from '../lib/api.js';

  export let results = null;
  export let wentu = null;
  export let isCreator = false;
  export let participantId = '';
  export let participantKey = '';

  let voters = [];
  let loadingVoters = false;
  let showVoterList = false;

  function findDateLabel(id) {
    return wentu?.date_options?.find((d) => d.id === id)?.label || 'Unknown date';
  }

  async function loadVoters() {
    if (!isCreator || !participantId || !participantKey) return;

    try {
      loadingVoters = true;
      const response = await api.post(`/api/wentu/${wentu.slug}/voters`, {
        participant_id: participantId,
        participant_key: participantKey,
      });
      voters = response.voters || [];
    } catch (err) {
      console.error('Failed to load voters:', err);
    } finally {
      loadingVoters = false;
    }
  }

  onMount(() => {
    if (isCreator) {
      loadVoters();
    }
  });
</script>

{#if results}
  <div class="card">
    <h3 class="text-lg sm:text-xl font-bold text-accent mb-3 sm:mb-4">Election Results</h3>

    <!-- Turnout Information -->
    <div class="bg-dark-bg rounded p-3 sm:p-4 mb-4 sm:mb-6">
      <div class="flex items-start gap-2 mb-3">
        <Info size={18} class="text-accent flex-shrink-0 mt-0.5" />
        <div class="flex-1">
          <p class="text-text-secondary text-xs sm:text-sm mb-2">
            <strong>Voter Turnout:</strong> {results.total_voters ?? 0} participant{(results.total_voters ?? 0) !== 1 ? 's' : ''} voted
          </p>
        </div>
      </div>

      <!-- Creator-only: Collapsible voter list -->
      {#if isCreator}
        <div class="mt-3 pt-3 border-t border-accent/20">
          <button
            on:click={() => showVoterList = !showVoterList}
            class="w-full text-left flex items-center gap-2 text-text-secondary hover:text-accent transition-colors text-xs sm:text-sm"
          >
            {#if showVoterList}
              <ChevronDown size={16} class="flex-shrink-0" />
            {:else}
              <ChevronRight size={16} class="flex-shrink-0" />
            {/if}
            <Users size={16} class="flex-shrink-0" />
            <span class="font-medium">Who has voted ({voters.length})</span>
          </button>

          {#if showVoterList}
            <div class="mt-3 pl-8">
              {#if loadingVoters}
                <p class="text-text-secondary text-xs italic">Loading...</p>
              {:else if voters.length === 0}
                <p class="text-text-secondary text-xs italic">No one has voted yet</p>
              {:else}
                <ul class="space-y-1">
                  {#each voters as voter}
                    <li class="text-text-secondary text-xs sm:text-sm">{voter}</li>
                  {/each}
                </ul>
              {/if}
            </div>
          {/if}
        </div>
      {/if}

      <!-- STV Explanation -->
      <div class="text-text-secondary text-xs sm:text-sm space-y-2">
        <p>
          <strong>How results are calculated:</strong> This election uses Single Transferable Vote (STV)
          to find the most preferred date. Each round, votes are counted for each participant's
          highest-ranked available option.
        </p>
        <p>
          To win, a date must reach the <strong>voting quota of {results.quota} vote{results.quota !== 1 ? 's' : ''}</strong>,
          which represents a simple majority (more than half of all voters).
        </p>
        <p>
          If no date reaches the quota, the date with the fewest votes is eliminated, and those
          votes transfer to the next preference on each ballot. This continues until a winner emerges.
        </p>
      </div>
    </div>

    <!-- Winner Display -->
    {#if results.winner}
      <div class="bg-success/10 border border-success/50 rounded p-3 sm:p-4 mb-4 sm:mb-6">
        <p class="text-text-secondary text-xs sm:text-sm">Winning Date:</p>
        <p class="text-success text-base sm:text-lg font-bold">{findDateLabel(results.winner)}</p>
      </div>
    {:else}
      <div class="bg-accent/10 border border-accent/30 rounded p-3 sm:p-4 mb-4 sm:mb-6">
        <p class="text-text-secondary text-xs sm:text-sm">No votes have been submitted yet.</p>
      </div>
    {/if}

    <!-- Round-by-Round Breakdown -->
    {#if results.rounds && results.rounds.length > 0}
      <div class="mb-4 sm:mb-6">
        <h4 class="text-base sm:text-lg font-semibold text-accent mb-3">
          Round-by-Round Breakdown
        </h4>
        <div class="space-y-2">
          {#each results.rounds as round}
            <div class="bg-dark-bg p-2 sm:p-3 rounded">
              <p class="text-accent font-medium mb-2 text-sm sm:text-base">
                Round {round.round_number}
                {#if round.round_number === results.rounds.length && results.winner}
                  <span class="text-success text-xs">(Winner determined)</span>
                {/if}
              </p>
              <div class="grid grid-cols-1 gap-1 sm:gap-2 text-xs sm:text-sm">
                {#each Object.entries(round.vote_counts).sort((a, b) => b[1] - a[1]) as [dateId, count]}
                  <div class="flex justify-between gap-2">
                    <span class="text-text-secondary truncate flex-1">{findDateLabel(dateId)}</span>
                    <span class="text-text-primary flex-shrink-0 font-medium">
                      {count} vote{count !== 1 ? 's' : ''}
                      {#if count >= results.quota}
                        <span class="text-success ml-1">✓ (Reached quota)</span>
                      {/if}
                    </span>
                  </div>
                {/each}
              </div>
              {#if round.eliminated}
                <p class="text-error text-xs sm:text-sm mt-2 flex items-center gap-1">
                  <XCircle size={14} class="flex-shrink-0" />
                  <span class="truncate">Eliminated: {findDateLabel(round.eliminated)}</span>
                </p>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
{/if}
