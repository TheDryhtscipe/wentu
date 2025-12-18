<script>
  import { dndzone } from 'svelte-dnd-action';
  import { flip } from 'svelte/animate';
  import { X, RotateCcw } from 'lucide-svelte';
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';

  export let timeSlots = [];
  export let dateLabel = '';

  const dispatch = createEventDispatcher();

  let orderedSlots = [];
  let removedSlots = [];
  let modalElement;

  // Initialize with a copy of timeSlots
  onMount(() => {
    orderedSlots = timeSlots.map(slot => ({ ...slot }));
    document.addEventListener('keydown', handleKeydown);
    // Focus the modal for accessibility
    if (modalElement) {
      modalElement.focus();
    }
  });

  onDestroy(() => {
    document.removeEventListener('keydown', handleKeydown);
  });

  function handleKeydown(e) {
    if (e.key === 'Escape') {
      cancel();
    }
  }

  function handleConsider(e) {
    orderedSlots = e.detail.items;
  }

  function handleFinalize(e) {
    orderedSlots = e.detail.items;
  }

  function removeSlot(id) {
    const slot = orderedSlots.find(s => s.id === id);
    if (slot) {
      orderedSlots = orderedSlots.filter(s => s.id !== id);
      removedSlots = [...removedSlots, slot];
    }
  }

  function restoreSlot(id) {
    const slot = removedSlots.find(s => s.id === id);
    if (slot) {
      removedSlots = removedSlots.filter(s => s.id !== id);
      orderedSlots = [...orderedSlots, slot];
    }
  }

  function confirm() {
    dispatch('confirm', {
      orderedTimeSlots: orderedSlots,
      removedTimeSlots: removedSlots
    });
  }

  function cancel() {
    dispatch('cancel');
  }

  function getTimeFromLabel(label) {
    // Extract time portion from label like "Mon, Dec 15 @ 10:00 AM"
    const match = label.match(/@\s*(.+)$/);
    return match ? match[1] : label;
  }
</script>

<!-- Backdrop -->
<div
  class="fixed inset-0 bg-dark-bg/80 z-40"
  on:click={cancel}
  on:keydown={(e) => e.key === 'Enter' && cancel()}
  role="button"
  tabindex="-1"
  aria-label="Close modal"
></div>

<!-- Modal -->
<div
  bind:this={modalElement}
  class="fixed inset-x-4 top-1/2 -translate-y-1/2 sm:inset-x-auto sm:left-1/2 sm:-translate-x-1/2 sm:max-w-md sm:w-full z-50 bg-content-bg border border-accent/20 rounded-lg p-4 sm:p-6 max-h-[80vh] overflow-y-auto"
  role="dialog"
  aria-modal="true"
  aria-labelledby="modal-title"
  tabindex="-1"
>
  <h3 id="modal-title" class="text-lg font-bold text-accent mb-2">{dateLabel}</h3>

  <!-- Warning message -->
  <p class="text-text-secondary text-sm mb-4">
    This date has multiple time slots. Drag to reorder, or remove times you don't want.
    All remaining times will be added to your preferences in this order.
  </p>

  <!-- Drag-drop list -->
  {#if orderedSlots.length > 0}
    <div
      use:dndzone={{
        items: orderedSlots,
        dropTargetStyle: { outline: '2px dashed #ff9c63' },
      }}
      on:consider={handleConsider}
      on:finalize={handleFinalize}
      class="space-y-2"
    >
      {#each orderedSlots as slot, idx (slot.id)}
        <div
          animate:flip={{ duration: 200 }}
          class="bg-dark-bg p-3 rounded flex items-center justify-between gap-3 cursor-move"
        >
          <div class="flex items-center gap-3 flex-1 min-w-0">
            <div class="bg-accent text-dark-bg w-7 h-7 rounded-full flex items-center justify-center font-bold text-sm flex-shrink-0">
              {idx + 1}
            </div>
            <p class="text-text-primary font-medium text-sm truncate">
              {getTimeFromLabel(slot.label)}
            </p>
          </div>
          <button
            on:click|stopPropagation={() => removeSlot(slot.id)}
            class="text-error hover:text-accent px-2 py-1 bg-error/10 rounded transition-colors focus:outline-offset-2 flex-shrink-0"
            aria-label="Remove {getTimeFromLabel(slot.label)} from selection"
            type="button"
          >
            <X size={16} aria-hidden="true" />
          </button>
        </div>
      {/each}
    </div>
  {:else}
    <div class="bg-dark-bg/50 p-4 rounded text-center">
      <p class="text-text-secondary text-sm">All times removed</p>
    </div>
  {/if}

  <!-- Removed slots section -->
  {#if removedSlots.length > 0}
    <div class="mt-4 pt-4 border-t border-accent/20">
      <h4 class="text-sm font-semibold text-text-secondary mb-2">
        Removed ({removedSlots.length})
      </h4>
      <div class="space-y-2">
        {#each removedSlots as slot (slot.id)}
          <div class="bg-dark-bg/50 p-3 rounded flex items-center justify-between gap-3 opacity-60">
            <p class="text-text-secondary text-sm truncate flex-1">
              {getTimeFromLabel(slot.label)}
            </p>
            <button
              on:click={() => restoreSlot(slot.id)}
              class="text-accent hover:text-accent/80 px-2 py-1 bg-accent/10 rounded transition-colors focus:outline-offset-2 flex-shrink-0"
              aria-label="Restore {getTimeFromLabel(slot.label)}"
              type="button"
            >
              <RotateCcw size={14} aria-hidden="true" />
            </button>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Action buttons -->
  <div class="flex gap-3 mt-6">
    <button class="btn-secondary flex-1" on:click={cancel} type="button">
      Cancel
    </button>
    <button
      class="btn-primary flex-1"
      on:click={confirm}
      type="button"
    >
      {#if orderedSlots.length > 0}
        Add {orderedSlots.length} time{orderedSlots.length !== 1 ? 's' : ''}
      {:else}
        Remove all
      {/if}
    </button>
  </div>
</div>
