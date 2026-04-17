<!--
  Note: `class` prop is concatenated after internal classes, but Tailwind
  JIT orders rules by utility group in the emitted CSS, not by string
  position. To override a specific internal utility, consumers may need
  to use `tailwind-merge` or a more specific class. For the current set
  of consumers this has not been an issue; revisit if Task 3 migration
  hits override needs.
-->
<script>
  export let elevated = false;
  export let padded = true;
  let className = '';
  export { className as class };

  $: surfaceClass = elevated ? 'bg-surface-elevated' : 'bg-surface-card';
  $: paddingClass = padded ? 'p-3 sm:p-4' : '';
</script>

<div class="rounded-lg border border-border-subtle {surfaceClass} {paddingClass} {className}">
  <slot />
</div>
