<script>
  import { onMount, onDestroy } from 'svelte';
  import { CheckCircle, AlertCircle, Info, X } from 'lucide-svelte';

  export let variant = 'info';
  export let message = '';
  export let duration = 4000;
  export let onDismiss = () => {};

  let timeoutId = null;

  const variantConfig = {
    success: { icon: CheckCircle, border: 'border-success', iconClass: 'text-success' },
    error: { icon: AlertCircle, border: 'border-error', iconClass: 'text-error' },
    info: { icon: Info, border: 'border-border-strong', iconClass: 'text-accent' },
  };

  $: config = variantConfig[variant] ?? variantConfig.info;
  $: isError = variant === 'error';

  onMount(() => {
    if (duration > 0) {
      timeoutId = setTimeout(() => {
        onDismiss();
      }, duration);
    }
  });

  onDestroy(() => {
    if (timeoutId !== null) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
  });

  function handleDismiss() {
    if (timeoutId !== null) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
    onDismiss();
  }
</script>

<div
  role={isError ? 'alert' : 'status'}
  aria-live={isError ? 'assertive' : 'polite'}
  class="bg-surface-elevated border {config.border} rounded-lg p-3 sm:p-4 shadow-lg flex items-start gap-3 min-w-[240px] max-w-sm"
>
  <svelte:component this={config.icon} size={20} class="flex-shrink-0 mt-0.5 {config.iconClass}" aria-hidden="true" />
  <p class="flex-1 text-sm text-text-primary">{message}</p>
  <button
    type="button"
    class="flex-shrink-0 text-text-secondary hover:text-text-primary transition-colors cursor-pointer"
    aria-label="Dismiss notification"
    on:click={handleDismiss}
  >
    <X size={16} aria-hidden="true" />
  </button>
</div>
