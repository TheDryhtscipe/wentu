<!--
  Note: `class` prop is concatenated after internal classes, but Tailwind
  JIT orders rules by utility group in the emitted CSS, not by string
  position. To override a specific internal utility, consumers may need
  to use `tailwind-merge` or a more specific class. For the current set
  of consumers this has not been an issue; revisit if Task 3 migration
  hits override needs.
-->
<script>
  import { Loader2 } from 'lucide-svelte';

  export let variant = 'primary';
  export let size = 'md';
  export let type = 'button';
  export let disabled = false;
  export let loading = false;
  export let fullWidth = false;
  let className = '';
  export { className as class };

  const base = 'rounded font-medium transition-colors focus:outline-offset-2 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center justify-center gap-2';

  const variantClasses = {
    primary: 'bg-action-primary text-dark-bg hover:bg-action-primary-hover',
    secondary: 'bg-action-secondary text-text-primary border border-border-strong hover:bg-action-secondary-hover',
    ghost: 'bg-transparent text-text-primary hover:bg-action-secondary-hover border border-transparent',
    danger: 'bg-error text-dark-bg hover:bg-error/80',
  };

  const sizeClasses = {
    sm: 'px-2 py-1 text-sm',
    md: 'px-3 sm:px-4 py-2 text-sm sm:text-base',
    lg: 'px-5 py-3 text-base sm:text-lg',
  };

  $: variantClass = variantClasses[variant] ?? variantClasses.primary;
  $: sizeClass = sizeClasses[size] ?? sizeClasses.md;
  $: widthClass = fullWidth ? 'w-full' : '';
  $: isDisabled = disabled || loading;
</script>

<button
  {type}
  disabled={isDisabled}
  aria-busy={loading ? 'true' : undefined}
  class="{base} {variantClass} {sizeClass} {widthClass} {className}"
  on:click
  on:focus
  on:blur
  on:keydown
  on:mousedown
  on:mouseenter
  on:mouseleave
>
  {#if loading}
    <Loader2 class="animate-spin" size={16} aria-hidden="true" />
  {:else if $$slots.iconLeft}
    <slot name="iconLeft" />
  {/if}
  <slot />
  {#if $$slots.iconRight}
    <slot name="iconRight" />
  {/if}
</button>
