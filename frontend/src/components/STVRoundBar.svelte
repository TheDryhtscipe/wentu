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

  <!-- Chart wrapper: holds quota label, chart rows, and quota line -->
  <div class="relative">
    <!-- Quota label aligned to the chart's bar column -->
    <div class="grid grid-cols-[minmax(0,7rem)_1fr_auto] items-center gap-2 mb-1">
      <span></span>
      <div class="relative">
        <span
          class="absolute -top-0.5 text-[10px] sm:text-xs text-accent font-medium whitespace-nowrap"
          style="left: {quotaPercent}%; transform: translateX(-50%);"
        >
          Quota: {effectiveQuota}
        </span>
      </div>
      <span></span>
    </div>

    <!-- Chart rows -->
    <div class="relative space-y-1.5">
      <!-- Quota line: vertical dashed line spanning chart rows column -->
      <div
        class="grid grid-cols-[minmax(0,7rem)_1fr_auto] gap-2 absolute inset-0 pointer-events-none"
        aria-hidden="true"
      >
        <div></div>
        <div class="relative">
          <div
            class="absolute top-0 bottom-0 border-l-2 border-dashed border-accent/60"
            style="left: {quotaPercent}%"
          ></div>
        </div>
        <div></div>
      </div>

      {#each sortedRows as { dateId, count } (dateId)}
        {@const widthPercent = totalVoters > 0 ? (count / totalVoters) * 100 : 0}
        {@const isEliminated = round.eliminated === dateId}
        {@const reachedQuota = count >= effectiveQuota && effectiveQuota > 0}
        {@const isWinnerRow = isFinalRound && dateId === winnerId}
        {@const fillClass = isWinnerRow
          ? 'bg-success'
          : isEliminated
            ? 'bg-error/70'
            : 'bg-accent'}
        <div
          class="grid grid-cols-[minmax(0,7rem)_1fr_auto] items-center gap-2"
          class:opacity-50={isEliminated}
        >
          <span class="text-xs sm:text-sm text-text-secondary truncate">
            {findDateLabel(dateId)}
          </span>
          <div class="relative h-5 bg-dark-bg/60 rounded-sm overflow-hidden">
            <div
              class="absolute inset-y-0 left-0 rounded-sm {fillClass}"
              style="width: {widthPercent}%"
            ></div>
          </div>
          <span class="text-xs sm:text-sm text-text-primary font-medium flex items-center gap-1">
            {count} vote{count !== 1 ? 's' : ''}
            {#if isEliminated}
              <XCircle size={14} class="text-error" aria-hidden="true" />
              <span class="sr-only">eliminated</span>
            {:else if reachedQuota}
              <span class="text-success" aria-hidden="true">✓</span>
              <span class="sr-only">reached quota</span>
            {/if}
          </span>
        </div>
      {/each}
    </div>
  </div>
</div>
