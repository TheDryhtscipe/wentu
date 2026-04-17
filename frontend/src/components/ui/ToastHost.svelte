<script>
  import { fly } from 'svelte/transition';
  import { toast } from '../../lib/toast.js';
  import Toast from './Toast.svelte';
</script>

<div class="fixed z-50 top-2 left-2 right-2 sm:top-4 sm:right-4 sm:left-auto flex flex-col gap-2 items-stretch sm:items-end pointer-events-none">
  {#each $toast as t (t.id)}
    <div
      class="pointer-events-auto"
      in:fly={{ x: 100, y: -20, duration: 200 }}
      out:fly={{ x: 100, y: -20, duration: 200 }}
    >
      <Toast
        variant={t.variant}
        message={t.message}
        duration={t.duration}
        onDismiss={() => toast.dismiss(t.id)}
      />
    </div>
  {/each}
</div>
