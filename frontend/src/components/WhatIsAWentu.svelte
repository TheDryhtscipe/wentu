<script>
  import { X } from 'lucide-svelte';
  import Button from './ui/Button.svelte';

  const STORAGE_KEY = 'wentu-onboarding-dismissed';

  // Synchronous read so first-timers don't see a flash-of-nothing and
  // return visitors don't see a flash-of-strip. Falls back to visible if
  // storage is unavailable (SSR, blocked cookies, etc.).
  function initialDismissed() {
    try {
      return typeof localStorage !== 'undefined'
        && localStorage.getItem(STORAGE_KEY) === '1';
    } catch {
      return false;
    }
  }

  let dismissed = initialDismissed();

  function dismiss() {
    try {
      localStorage.setItem(STORAGE_KEY, '1');
    } catch {
      // localStorage blocked — still hide for this session.
    }
    dismissed = true;
  }
</script>

{#if !dismissed}
  <aside
    class="rounded-lg border border-border-subtle bg-surface-elevated p-4 mb-6 flex items-start gap-3"
    aria-label="What is a Wentu?"
  >
    <div class="flex-1">
      <h3 class="text-sm font-semibold text-text-primary mb-1">What is a Wentu?</h3>
      <p class="text-sm text-text-secondary">
        A wentu lets a group rank dates for a meeting. No accounts needed: just share the link, and everyone can rank the options.
      </p>
      <div class="mt-3">
        <Button variant="ghost" size="sm" on:click={dismiss}>Got it</Button>
      </div>
    </div>
    <button
      type="button"
      class="text-text-muted hover:text-text-primary shrink-0 cursor-pointer"
      aria-label="Dismiss"
      on:click={dismiss}
    >
      <X size={18} aria-hidden="true" />
    </button>
  </aside>
{/if}
