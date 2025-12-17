<script>
  import { dndzone } from 'svelte-dnd-action';
  import { flip } from 'svelte/animate';
  import { X } from 'lucide-svelte';

  export let items = [];

  let dragDisabled = false;

  function handleConsider(e) {
    items = e.detail.items;
  }

  function handleFinalize(e) {
    items = e.detail.items;
  }

  function removeItem(id) {
    items = items.filter((item) => item.id !== id);
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
  class="space-y-2"
>
  {#each items as item (item.id)}
    <div animate:flip={{ duration: 200 }} class="bg-dark-bg p-4 rounded flex justify-between items-center cursor-move">
      <div class="flex-1">
        <p class="text-text-primary font-medium">{item.label}</p>
      </div>
      <button
        on:click={() => removeItem(item.id)}
        class="text-error hover:text-accent ml-4 px-3 py-1 bg-error/10 rounded"
        aria-label="Remove date"
      >
        <X size={16} />
      </button>
    </div>
  {/each}
</div>
