<script>
  import { onMount } from 'svelte';
  import { Clock } from 'lucide-svelte';

  export let expiresAt = null;

  let timeLeft = '';

  onMount(() => {
    function updateTimer() {
      if (!expiresAt) return;

      const now = new Date();
      const diff = new Date(expiresAt) - now;

      if (diff <= 0) {
        timeLeft = 'Expired';
        return;
      }

      const hours = Math.floor(diff / (1000 * 60 * 60));
      const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));

      if (hours > 0) {
        timeLeft = `${hours}h ${minutes}m left`;
      } else {
        timeLeft = `${minutes}m left`;
      }
    }

    updateTimer();
    const interval = setInterval(updateTimer, 30000); // Update every 30 seconds
    return () => clearInterval(interval);
  });
</script>

{#if timeLeft}
  <div class="text-right">
    <p class="text-text-secondary text-sm flex items-center justify-end gap-1">
      <Clock size={14} />
      Expires in
    </p>
    <p class="text-accent font-mono font-bold">{timeLeft}</p>
  </div>
{/if}
