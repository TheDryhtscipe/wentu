<!--
  The `surface` prop selects the background wash + matching border in one place,
  so consumers don't fight CSS declaration order against Tailwind utilities.
  Pass a custom `class` only for layout (margin/spacing); colour washes belong
  in `surface`.
-->
<script>
  export let elevated = false;
  export let padded = true;
  export let element = undefined;
  /** @type {'default' | 'success' | 'accent'} */
  export let surface = 'default';
  let className = '';
  export { className as class };

  $: backgroundClass = (() => {
    if (surface === 'success') return 'bg-success/10';
    if (surface === 'accent') return 'bg-accent/10';
    return elevated ? 'bg-surface-elevated' : 'bg-surface-card';
  })();

  $: borderClass = (() => {
    if (surface === 'success') return 'border-success/50';
    if (surface === 'accent') return 'border-accent/30';
    return 'border-border-subtle';
  })();

  $: paddingClass = padded ? 'p-3 sm:p-4' : '';
</script>

<div
  {...$$restProps}
  bind:this={element}
  class="rounded-lg border {borderClass} {backgroundClass} {paddingClass} {className}"
>
  <slot />
</div>
