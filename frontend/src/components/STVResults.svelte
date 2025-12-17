<script>
  import { XCircle } from 'lucide-svelte';

  export let results = null;
  export let wentu = null;

  function findDateLabel(id) {
    return wentu?.date_options?.find((d) => d.id === id)?.label || 'Unknown date';
  }
</script>

{#if results}
  <div class="card">
    <h3 class="text-xl font-bold text-accent mb-4">STV Results</h3>

    {#if results.winner}
      <div class="bg-success/10 border border-success/50 rounded p-4 mb-6">
        <p class="text-text-secondary text-sm">Winner (preferred date):</p>
        <p class="text-success text-lg font-bold">{findDateLabel(results.winner)}</p>
      </div>
    {/if}

    <div class="mb-6">
      <p class="text-text-secondary text-sm mb-3">Voting quota: {results.quota}</p>
      <div class="space-y-2">
        {#each results.rounds as round}
          <div class="bg-dark-bg p-3 rounded">
            <p class="text-accent font-medium mb-2">Round {round.round_number}</p>
            <div class="grid grid-cols-1 gap-2 text-sm">
              {#each Object.entries(round.vote_counts) as [dateId, count]}
                <div class="flex justify-between">
                  <span class="text-text-secondary">{findDateLabel(dateId)}</span>
                  <span class="text-text-primary">{count} vote{count !== 1 ? 's' : ''}</span>
                </div>
              {/each}
            </div>
            {#if round.eliminated}
              <p class="text-error text-sm mt-2 flex items-center gap-1">
                <XCircle size={14} />
                Eliminated: {findDateLabel(round.eliminated)}
              </p>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}
