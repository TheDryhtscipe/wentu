<script>
  import { XCircle } from 'lucide-svelte';

  export let round;            // { round_number, vote_counts: {[id]: number}, eliminated?: string|null, quota?: number }
  export let totalVoters;      // results.total_voters — anchor for bar widths
  export let fallbackQuota;    // results.quota — used when round.quota is undefined
  export let winnerId;         // results.winner — string|null
  export let isFinalRound;     // boolean — true when this is the last round in results.rounds
  export let findDateLabel;    // (id) => string — callback for date label lookup

  $: effectiveQuota = round.quota ?? fallbackQuota;
  $: quotaPercent = totalVoters > 0
    ? Math.min(100, (effectiveQuota / totalVoters) * 100)
    : 0;
  $: sortedRows = Object.entries(round.vote_counts)
    .map(([dateId, count]) => ({ dateId, count }))
    .sort((a, b) => b.count - a.count);

  $: statusText = (() => {
    if (isFinalRound && winnerId) return { text: `Winner: ${findDateLabel(winnerId)}`, tone: 'success' };
    if (round.eliminated) return { text: `${findDateLabel(round.eliminated)} eliminated`, tone: 'error' };
    return null;
  })();
</script>

<div class="space-y-2">
  <!-- Header row: Round N + status -->
  <div class="flex justify-between items-baseline gap-2">
    <h5 class="text-accent text-sm sm:text-base font-medium flex-shrink-0">
      Round {round.round_number}
    </h5>
    {#if statusText}
      <span
        class="text-xs truncate"
        class:text-success={statusText.tone === 'success'}
        class:text-error={statusText.tone === 'error'}
      >
        {statusText.text}
      </span>
    {/if}
  </div>

  <!-- Chart skeleton (Task C will populate) -->
  <div class="relative">
    <!-- rows -->
  </div>
</div>
