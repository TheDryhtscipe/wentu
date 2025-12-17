<script>
  import { createEventDispatcher } from 'svelte';
  import { Lightbulb } from 'lucide-svelte';

  const dispatch = createEventDispatcher();

  let slug = '';
  let error = '';

  function goCreate() {
    dispatch('navigate', { page: 'create' });
  }

  function goView() {
    if (!slug.trim()) {
      error = 'Please enter a wentu code';
      return;
    }
    dispatch('navigate', { page: 'view', params: { slug: slug.trim() } });
  }

  function handleKeydown(e) {
    if (e.key === 'Enter') {
      goView();
    }
  }
</script>

<div class="max-w-2xl mx-auto">
  <div class="text-center mb-12">
    <h2 class="text-4xl font-bold text-accent mb-4">Schedule your meeting</h2>
    <p class="text-text-secondary text-lg">with ranked choice voting</p>
  </div>

  <div class="grid gap-8 md:grid-cols-2">
    <!-- Create Section -->
    <div class="card">
      <h3 class="text-xl font-bold text-accent mb-4">Create new</h3>
      <p class="text-text-secondary mb-6">Set a title, pick date options, and share the code with others.</p>
      <button class="btn-primary w-full" on:click={goCreate}>
        Create Wentu
      </button>
    </div>

    <!-- Join Section -->
    <div class="card">
      <h3 class="text-xl font-bold text-accent mb-4">Join existing</h3>
      <p class="text-text-secondary mb-4">Enter a wentu code to vote.</p>
      <input
        class="input w-full mb-4"
        type="text"
        placeholder="Enter wentu code"
        bind:value={slug}
        on:keydown={handleKeydown}
        aria-label="Wentu code"
      />
      {#if error}
        <p class="text-error text-sm mb-4">{error}</p>
      {/if}
      <button class="btn-primary w-full" on:click={goView}>
        Join
      </button>
    </div>
  </div>

  <div class="mt-12 text-center text-text-secondary text-sm">
    <p class="flex items-center justify-center gap-2">
      <Lightbulb size={16} class="text-accent" />
      No account needed. Create your wentu, share the code, decide when to meet.
    </p>
    <p>All data expires after 7 days by default.</p>
  </div>
</div>
