<script>
  import { onMount } from 'svelte';
  import Home from './pages/Home.svelte';
  import CreateWentu from './pages/CreateWentu.svelte';
  import ViewWentu from './pages/ViewWentu.svelte';
  import './app.css';

  let currentPage = 'home';
  let params = {};

  function navigate(page, newParams = {}) {
    currentPage = page;
    params = newParams;
  }

  onMount(() => {
    // Could add router here for real URL handling
  });
</script>

<div class="min-h-screen bg-dark-bg text-text-primary flex flex-col">
  <header class="bg-content-bg border-b border-accent/30 py-4 px-6">
    <h1 class="text-2xl font-bold text-accent cursor-pointer" on:click={() => navigate('home')}>
      Wentu
    </h1>
    <p class="text-text-secondary text-sm">When to meet with STV</p>
  </header>

  <main class="flex-1 p-6">
    {#if currentPage === 'home'}
      <Home on:navigate={(e) => navigate(e.detail.page, e.detail.params)} />
    {:else if currentPage === 'create'}
      <CreateWentu on:navigate={(e) => navigate(e.detail.page, e.detail.params)} />
    {:else if currentPage === 'view'}
      <ViewWentu
        slug={params.slug}
        creatorName={params.creatorName ?? ''}
        creatorKey={params.creatorKey ?? ''}
        creatorParticipantId={params.creatorParticipantId ?? ''}
        creatorParticipantKey={params.creatorParticipantKey ?? ''}
        on:navigate={(e) => navigate(e.detail.page, e.detail.params)}
      />
    {/if}
  </main>

  <footer class="bg-content-bg border-t border-accent/30 py-4 px-6 text-center text-text-secondary text-sm">
    <p>No accounts, no persistence. Everything expires.</p>
  </footer>
</div>

<style global>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }
</style>
