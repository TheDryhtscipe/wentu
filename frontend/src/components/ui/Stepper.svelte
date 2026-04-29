<script>
  import { createEventDispatcher } from 'svelte';
  import { Check } from 'lucide-svelte';

  export let steps = [];
  export let current = '';
  export let completed = new Set();

  const dispatch = createEventDispatcher();

  $: visibleSteps = steps;
  $: currentIndex = visibleSteps.findIndex((s) => s.id === current);
  $: currentLabel = currentIndex >= 0 ? visibleSteps[currentIndex].label : '';

  function isCompleted(id) {
    return completed.has(id);
  }

  function handleClick(step) {
    if (isCompleted(step.id) && step.id !== current) {
      dispatch('navigate', { id: step.id });
    }
  }
</script>

<nav aria-label="Create wentu progress" class="w-full">
  <!-- Mobile: compact counter -->
  <div class="sm:hidden flex items-baseline gap-2">
    <span class="text-sm text-text-muted">Step {currentIndex + 1} of {visibleSteps.length}</span>
    <span class="text-sm font-medium text-text-primary">{currentLabel}</span>
  </div>

  <!-- Desktop: horizontal progress -->
  <ol class="hidden sm:flex items-center gap-2" role="list">
    {#each visibleSteps as step, i (step.id)}
      {@const done = isCompleted(step.id)}
      {@const active = step.id === current}
      {@const clickable = done && !active}
      <li class="flex items-center gap-2">
        <button
          type="button"
          class="flex items-center gap-2 rounded px-2 py-1 text-sm transition-colors focus:outline-offset-2 {clickable ? 'cursor-pointer hover:bg-action-secondary-hover' : 'cursor-default'}"
          disabled={!clickable}
          aria-current={active ? 'step' : undefined}
          on:click={() => handleClick(step)}
        >
          <span
            class="inline-flex h-6 w-6 items-center justify-center rounded-full border text-xs font-medium {active ? 'border-action-primary bg-action-primary text-dark-bg' : done ? 'border-action-primary text-action-primary' : 'border-border-strong text-text-muted'}"
            aria-hidden="true"
          >
            {#if done && !active}
              <Check size={14} strokeWidth={2.5} />
            {:else}
              {i + 1}
            {/if}
          </span>
          <span class="{active ? 'text-text-primary font-medium' : done ? 'text-text-primary' : 'text-text-muted'}">
            {step.label}
          </span>
        </button>
        {#if i < visibleSteps.length - 1}
          <span class="h-px w-6 bg-border-subtle" aria-hidden="true"></span>
        {/if}
      </li>
    {/each}
  </ol>
</nav>
