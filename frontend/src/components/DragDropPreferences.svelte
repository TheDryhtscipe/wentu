<script>
  import { dndzone } from 'svelte-dnd-action';
  import { flip } from 'svelte/animate';
  import { X } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';

  export let items = [];
  export let disabled = false;

  const dispatch = createEventDispatcher();

  $: dragDisabled = disabled;

  function handleConsider(e) {
    items = e.detail.items;
  }

  function handleFinalize(e) {
    items = e.detail.items;
  }

  function handleRemove(id) {
    dispatch('remove', { id });
  }
</script>

<div
  use:dndzone={{
    items,
    dragDisabled,
    dropTargetStyle: { outline: '2px dashed #ff9c63' },
  }}
  on:consider={handleConsider}
  on:finalize={handleFinalize}
  class="space-y-3"
>
  {#each items as item, idx (item.id)}
    <div animate:flip={{ duration: 200 }} class="bg-dark-bg p-4 rounded flex items-center justify-between gap-4 cursor-move">
      <div class="flex items-center gap-4 flex-1">
        <div class="bg-accent text-dark-bg w-8 h-8 rounded-full flex items-center justify-center font-bold flex-shrink-0">
          {idx + 1}
        </div>
        <div class="flex-1">
          <p class="text-text-primary font-medium">{item.label}</p>
          <p class="text-text-secondary text-sm">
            {new Date(item.start).toLocaleDateString()} – {new Date(item.end).toLocaleDateString()}
          </p>
        </div>
      </div>
      {#if !disabled}
        <button
          on:click|stopPropagation={() => handleRemove(item.id)}
          class="text-error hover:text-accent ml-4 px-3 py-1 bg-error/10 rounded transition-colors focus:outline-offset-2"
          aria-label="Remove {item.label} from preferences"
          type="button"
        >
          <X size={16} aria-hidden="true" />
        </button>
      {/if}
    </div>
  {/each}
</div>
