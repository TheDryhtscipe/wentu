<script>
  export let value = '';
  export let label = '';
  export let hint = '';
  export let error = '';
  export let required = false;
  export let placeholder = '';
  export let disabled = false;
  export let rows = 3;
  export let id = '';
  let className = '';
  export { className as class };

  const autoId = (typeof crypto !== 'undefined' && crypto.randomUUID)
    ? `textarea-${crypto.randomUUID()}`
    : `textarea-${Math.random().toString(36).slice(2)}`;

  $: resolvedId = id || autoId;
  $: hintId = `${resolvedId}-hint`;
  $: errorId = `${resolvedId}-error`;
  $: describedBy = error ? errorId : (hint ? hintId : undefined);

  const base = 'px-2 sm:px-3 py-2 bg-surface-card border rounded text-text-primary placeholder-text-secondary focus:border-focus-ring focus:outline-none text-sm sm:text-base resize-y';
  $: borderClass = error ? 'border-error' : 'border-border-subtle';

  function handleInput(event) {
    value = event.target.value;
  }
</script>

{#if label}
  <label for={resolvedId} class="block text-sm text-text-secondary mb-1">
    {label}{#if required}<span class="text-error ml-0.5" aria-hidden="true">*</span>{/if}
  </label>
{/if}

<textarea
  {...$$restProps}
  id={resolvedId}
  {value}
  {placeholder}
  {disabled}
  {required}
  {rows}
  aria-invalid={error ? 'true' : undefined}
  aria-describedby={describedBy}
  class="{base} {borderClass} {className}"
  on:input={handleInput}
  on:input
  on:change
  on:keydown
  on:focus
  on:blur
></textarea>

{#if error}
  <p id={errorId} class="mt-1 text-xs text-error">{error}</p>
{:else if hint}
  <p id={hintId} class="mt-1 text-xs text-text-muted">{hint}</p>
{/if}
