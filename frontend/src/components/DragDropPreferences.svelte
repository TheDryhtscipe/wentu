<script>
  import { dndzone } from 'svelte-dnd-action';
  import { flip } from 'svelte/animate';

  export let items = [];
  export let disabled = false;

  $: dragDisabled = disabled;

  function handleConsider(e) {
    items = e.detail.items;
  }

  function handleFinalize(e) {
    items = e.detail.items;
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
    <div animate:flip={{ duration: 200 }} class="bg-dark-bg p-4 rounded flex items-center gap-4 cursor-move">
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
  {/each}
</div>
