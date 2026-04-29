<script>
  import { onMount } from 'svelte';
  import { XCircle, Info, ChevronDown, ChevronRight, Users, Trophy } from 'lucide-svelte';
  import { api } from '../lib/api.js';
  import Card from './ui/Card.svelte';
  import STVRoundBar from './STVRoundBar.svelte';

  export let results = null;
  export let wentu = null;
  export let isCreator = false;
  export let participantId = '';
  export let participantKey = '';

  let voters = [];
  let loadingVoters = false;
  let showVoterList = false;

  const SHOW_EXPLANATION_KEY = 'wentu-show-stv-explanation';
  let showExplanation = false;

  function toggleExplanation() {
    showExplanation = !showExplanation;
    if (typeof localStorage === 'undefined') return;
    try {
      if (showExplanation) {
        localStorage.setItem(SHOW_EXPLANATION_KEY, '1');
      } else {
        localStorage.removeItem(SHOW_EXPLANATION_KEY);
      }
    } catch {
      // localStorage write failure (e.g., private browsing quota) — UI state still updates
    }
  }

  function findDateLabel(id) {
    return wentu?.date_options?.find((d) => d.id === id)?.label || 'Unknown date';
  }

  function findWinningRound(res) {
    if (!res?.winner || !res.rounds?.length) return null;
    const winnerId = res.winner;
    for (const round of res.rounds) {
      const count = round.vote_counts?.[winnerId] ?? 0;
      const quota = round.quota ?? res.quota;
      if (count >= quota) {
        return { roundNumber: round.round_number, count };
      }
    }
    return null;
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
    showExplanation = typeof localStorage !== 'undefined'
      && localStorage.getItem(SHOW_EXPLANATION_KEY) === '1';
    if (isCreator) {
      loadVoters();
    }
  });
</script>

{#if results}
  {@const winningRound = findWinningRound(results)}
  {@const eligible = results.total_participants ?? null}

  <div class="space-y-3 sm:space-y-4">

    <!-- Hero Card -->
    {#if results.winner}
      <Card surface="success">
        <div class="flex items-start gap-3 sm:gap-4">
          <Trophy size={32} class="text-success flex-shrink-0 mt-1" />
          <div class="flex-1 min-w-0">
            <p class="text-text-secondary text-xs sm:text-sm">Winning date</p>
            <p class="text-success text-2xl sm:text-3xl font-bold leading-tight break-words">
              {findDateLabel(results.winner)}
            </p>
            <p class="text-text-secondary text-xs sm:text-sm mt-1">
              {#if winningRound}
                Won round {winningRound.roundNumber} with {winningRound.count} vote{winningRound.count !== 1 ? 's' : ''}
                {#if eligible !== null}
                  · {results.total_voters} of {eligible} voted
                {:else}
                  · {results.total_voters} voted
                {/if}
              {:else if eligible !== null}
                {results.total_voters} of {eligible} voted
              {:else}
                {results.total_voters} voted
              {/if}
            </p>
          </div>
        </div>
      </Card>
    {:else}
      <Card surface="accent">
        <div class="flex items-start gap-3 sm:gap-4">
          <Info size={28} class="text-accent flex-shrink-0 mt-1" />
          <div class="flex-1 min-w-0">
            <p class="text-text-secondary text-xs sm:text-sm">Results pending</p>
            <p class="text-accent text-xl sm:text-2xl font-bold leading-tight">No votes yet</p>
          </div>
        </div>
      </Card>
    {/if}

    <!-- Explanation disclosure -->
    {#if results.winner || (results.rounds && results.rounds.length > 0)}
      <div>
        <button
          type="button"
          class="flex items-center gap-2 text-text-secondary hover:text-accent transition-colors text-sm w-full text-left focus:outline-offset-2"
          aria-expanded={showExplanation}
          aria-controls="stv-explanation-body"
          on:click={toggleExplanation}
        >
          {#if showExplanation}
            <ChevronDown size={16} class="flex-shrink-0" aria-hidden="true" />
          {:else}
            <ChevronRight size={16} class="flex-shrink-0" aria-hidden="true" />
          {/if}
          <span class="font-medium">How this was calculated</span>
        </button>

        {#if showExplanation}
          <div
            id="stv-explanation-body"
            class="bg-dark-bg rounded p-3 sm:p-4 mt-2 text-text-secondary text-xs sm:text-sm space-y-2"
          >
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
        {/if}
      </div>
    {/if}

    <!-- Results Card with round-by-round bars: Task G fills this in -->

    <!-- Creator-only voter Card: Task H fills this in -->

  </div>
{/if}
