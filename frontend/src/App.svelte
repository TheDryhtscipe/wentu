<script>
  import { onMount } from 'svelte';
  import Home from './pages/Home.svelte';
  import CreateWentu from './pages/CreateWentu.svelte';
  import ViewWentu from './pages/ViewWentu.svelte';
  import './app.css';

  let currentPage = 'home';
  let params = {};

  function parseUrl() {
    const path = window.location.pathname;
    const searchParams = new URLSearchParams(window.location.search);

    if (path === '/' || path === '') {
      currentPage = 'home';
      params = {};
    } else if (path === '/create') {
      currentPage = 'create';
      params = {};
    } else if (path.startsWith('/wentu/')) {
      currentPage = 'view';
      const slug = path.substring(7); // Remove '/wentu/'
      params = {
        slug,
        creatorName: searchParams.get('creatorName') || '',
        creatorKey: searchParams.get('creatorKey') || '',
        creatorParticipantId: searchParams.get('creatorParticipantId') || '',
        creatorParticipantKey: searchParams.get('creatorParticipantKey') || '',
      };
    } else {
      // Unknown route, default to home
      currentPage = 'home';
      params = {};
    }
  }

  function navigate(page, newParams = {}) {
    currentPage = page;
    params = newParams;

    // Update browser URL
    let url = '/';
    if (page === 'create') {
      url = '/create';
    } else if (page === 'view' && newParams.slug) {
      url = `/wentu/${newParams.slug}`;

      // Add creator credentials as query params if present
      const queryParams = new URLSearchParams();
      if (newParams.creatorName) queryParams.set('creatorName', newParams.creatorName);
      if (newParams.creatorKey) queryParams.set('creatorKey', newParams.creatorKey);
      if (newParams.creatorParticipantId) queryParams.set('creatorParticipantId', newParams.creatorParticipantId);
      if (newParams.creatorParticipantKey) queryParams.set('creatorParticipantKey', newParams.creatorParticipantKey);

      const queryString = queryParams.toString();
      if (queryString) {
        url += `?${queryString}`;
      }
    }

    window.history.pushState({}, '', url);
  }

  function handlePopState() {
    parseUrl();
  }

  onMount(() => {
    // Parse initial URL
    parseUrl();

    // Listen for browser back/forward buttons
    window.addEventListener('popstate', handlePopState);

    return () => {
      window.removeEventListener('popstate', handlePopState);
    };
  });
</script>

<div class="min-h-screen bg-dark-bg text-text-primary flex flex-col">
  <header class="bg-content-bg border-b border-accent/30 py-3 px-4 sm:py-4 sm:px-6">
    <button type="button" class="text-xl sm:text-2xl font-bold text-accent hover:underline" on:click={() => navigate('home')} aria-label="Go to home">
      Wentu
    </button>
    <p class="text-text-secondary text-xs sm:text-sm">When to meet with STV</p>
  </header>

  <main class="flex-1 p-4 sm:p-6">
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

  <footer class="bg-content-bg border-t border-accent/30 py-3 px-4 sm:py-4 sm:px-6 text-center text-text-secondary text-xs sm:text-sm">
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
