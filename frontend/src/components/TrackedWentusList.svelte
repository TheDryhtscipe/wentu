<script>
  import { createEventDispatcher, onMount } from 'svelte';
  import { X, Crown } from 'lucide-svelte';
  import { getTrackedWentus, removeTrackedWentu } from '../lib/wentuTracker.js';

  const dispatch = createEventDispatcher();
  let trackedWentus = [];

  onMount(() => {
    trackedWentus = getTrackedWentus();
  });

  function handleRemove(slug, event) {
    event.stopPropagation(); // Prevent navigation when clicking remove
    removeTrackedWentu(slug);
    trackedWentus = getTrackedWentus(); // Refresh the list
  }

  function handleClick(slug) {
    dispatch('navigate', { page: 'view', params: { slug } });
  }
</script>

{#if trackedWentus.length > 0}
  <div class="card">
    <h3 class="text-lg sm:text-xl font-bold text-accent mb-3 sm:mb-4">
      Your Wentus
    </h3>
    <div class="space-y-2">
      {#each trackedWentus as wentu}
        <div class="bg-dark-bg p-3 rounded flex items-center justify-between gap-3 group hover:bg-dark-bg/80 transition-colors">
          <button
            on:click={() => handleClick(wentu.slug)}
            class="flex-1 text-left min-w-0"
          >
            <p class="text-text-primary font-medium text-sm sm:text-base truncate group-hover:text-accent transition-colors">
              {wentu.title}
            </p>
            <div class="flex items-center gap-2 mt-1 flex-wrap">
              <span class="text-xs px-2 py-0.5 rounded inline-flex items-center gap-1 {wentu.role === 'owner' ? 'bg-accent/20 text-accent' : 'bg-text-secondary/20 text-text-secondary'}">
                {#if wentu.role === 'owner'}
                  <Crown size={12} />
                {/if}
                {wentu.role === 'owner' ? 'Owner' : 'Participant'}
              </span>
              <span class="text-text-secondary text-xs font-mono">
                {wentu.slug}
              </span>
            </div>
          </button>
          <button
            on:click={(e) => handleRemove(wentu.slug, e)}
            class="text-error hover:text-accent px-2 py-1 bg-error/10 hover:bg-error/20 rounded transition-colors flex-shrink-0"
            aria-label="Remove {wentu.title} from list"
          >
            <X size={16} />
          </button>
        </div>
      {/each}
    </div>
  </div>
{/if}
