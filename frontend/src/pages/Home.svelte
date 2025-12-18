<script>
  import { createEventDispatcher } from 'svelte';
  import { Lightbulb } from 'lucide-svelte';
  import TrackedWentusList from '../components/TrackedWentusList.svelte';

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

<div class="max-w-2xl mx-auto px-4 sm:px-0">
  <div class="text-center mb-8 sm:mb-12">
    <h2 class="text-2xl sm:text-3xl md:text-4xl font-bold text-accent mb-3 sm:mb-4">Schedule your meeting</h2>
    <p class="text-text-secondary text-base sm:text-lg">with ranked choice voting</p>
  </div>

  <div class="grid gap-6 sm:gap-8 md:grid-cols-2">
    <!-- Create Section -->
    <div class="card flex flex-col">
      <h3 class="text-lg sm:text-xl font-bold text-accent mb-3 sm:mb-4">Create new</h3>
      <p class="text-text-secondary text-sm sm:text-base mb-4 sm:mb-6">Set a title, pick date options, and share the code with others.</p>
      <button class="btn-primary w-full mt-auto" on:click={goCreate}>
        Create Wentu
      </button>
    </div>

    <!-- Join Section -->
    <div class="card flex flex-col">
      <h3 class="text-lg sm:text-xl font-bold text-accent mb-3 sm:mb-4">Join existing</h3>
      <p class="text-text-secondary text-sm sm:text-base mb-3 sm:mb-4">Enter a wentu code to vote.</p>
      <input
        class="input w-full mb-3 sm:mb-4"
        type="text"
        placeholder="Enter wentu code"
        bind:value={slug}
        on:keydown={handleKeydown}
        aria-label="Wentu code"
      />
      {#if error}
        <p class="text-error text-sm mb-3 sm:mb-4">{error}</p>
      {/if}
      <button class="btn-primary w-full mt-auto" on:click={goView}>
        Join
      </button>
    </div>
  </div>

  <div class="mt-8 sm:mt-12 text-center text-text-secondary text-xs sm:text-sm px-4">
    <p class="flex items-center justify-center gap-2 flex-wrap">
      <Lightbulb size={16} class="text-accent flex-shrink-0" />
      <span>No account needed. Create your wentu, share the code, decide when to meet.</span>
    </p>
    <p class="mt-2">All data expires 7 days after the voting deadline.</p>
  </div>

  <!-- Tracked Wentus Section -->
  <div class="mt-8 sm:mt-12">
    <TrackedWentusList on:navigate={(e) => dispatch('navigate', e.detail)} />
  </div>
</div>
