<!--
  Applies the `.input` marker class to keep the app-wide `select.input` chevron
  SVG rule (app.css) painting the purple caret. If that rule is ever removed,
  this marker becomes inert but harmless.
-->
<script>
  export let value = '';
  export let label = '';
  export let hint = '';
  export let error = '';
  export let required = false;
  export let disabled = false;
  export let id = '';
  let className = '';
  export { className as class };

  const autoId = (typeof crypto !== 'undefined' && crypto.randomUUID)
    ? `select-${crypto.randomUUID()}`
    : `select-${Math.random().toString(36).slice(2)}`;

  $: resolvedId = id || autoId;
  $: hintId = `${resolvedId}-hint`;
  $: errorId = `${resolvedId}-error`;
  $: describedBy = error ? errorId : (hint ? hintId : undefined);

  const base = 'input px-2 sm:px-3 py-2 bg-surface-card border rounded text-text-primary focus:border-focus-ring focus:outline-none text-sm sm:text-base';
  $: borderClass = error ? 'border-error' : 'border-border-subtle';
</script>

{#if label}
  <label for={resolvedId} class="block text-sm text-text-secondary mb-1">
    {label}{#if required}<span class="text-error ml-0.5" aria-hidden="true">*</span>{/if}
  </label>
{/if}

<select
  {...$$restProps}
  id={resolvedId}
  bind:value
  {disabled}
  {required}
  aria-invalid={error ? 'true' : undefined}
  aria-describedby={describedBy}
  class="{base} {borderClass} {className}"
  on:change
  on:focus
  on:blur
  on:keydown
>
  <slot />
</select>

{#if error}
  <p id={errorId} class="mt-1 text-xs text-error">{error}</p>
{:else if hint}
  <p id={hintId} class="mt-1 text-xs text-text-muted">{hint}</p>
{/if}
