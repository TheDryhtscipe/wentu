<!--
  Note: the `class` prop is concatenated after internal classes, but
  Tailwind JIT orders rules by utility group in the emitted CSS, not by
  string position. To override a specific internal utility, consumers
  may need `tailwind-merge` or a more specific class.
-->
<script>
  export let type = 'text';
  export let value = '';
  export let label = '';
  export let hint = '';
  export let error = '';
  export let required = false;
  export let placeholder = '';
  export let disabled = false;
  export let id = '';
  let className = '';
  export { className as class };

  // Auto-generate id if not provided.
  const autoId = (typeof crypto !== 'undefined' && crypto.randomUUID)
    ? `input-${crypto.randomUUID()}`
    : `input-${Math.random().toString(36).slice(2)}`;

  // Derive hint/error ids from the resolved id so caller-provided `id`
  // props produce matching `<id>-hint` / `<id>-error` targets.
  $: resolvedId = id || autoId;
  $: hintId = `${resolvedId}-hint`;
  $: errorId = `${resolvedId}-error`;

  $: describedBy = error ? errorId : (hint ? hintId : undefined);

  const base = 'px-2 sm:px-3 py-2 bg-surface-card border rounded text-text-primary placeholder-text-secondary focus:border-focus-ring focus:outline-none text-sm sm:text-base';
  $: borderClass = error ? 'border-error' : 'border-border-subtle';

  // Svelte does not support dynamic `type` combined with `bind:value` on a single
  // <input>. We handle the common text-like types with a shared block and emit
  // input/change events so callers can opt out of `bind:value` via event-driven
  // flows if they need a less-common type.
  function handleInput(event) {
    value = event.target.value;
  }
</script>

{#if label}
  <label for={resolvedId} class="block text-sm text-text-secondary mb-1">
    {label}{#if required}<span class="text-error ml-0.5" aria-hidden="true">*</span>{/if}
  </label>
{/if}

<input
  {...$$restProps}
  {type}
  id={resolvedId}
  {value}
  {placeholder}
  {disabled}
  {required}
  aria-invalid={error ? 'true' : undefined}
  aria-describedby={describedBy}
  class="{base} {borderClass} {className}"
  on:input={handleInput}
  on:input
  on:change
  on:keydown
  on:focus
  on:blur
/>

{#if error}
  <p id={errorId} class="mt-1 text-xs text-error">{error}</p>
{:else if hint}
  <p id={hintId} class="mt-1 text-xs text-text-muted">{hint}</p>
{/if}
