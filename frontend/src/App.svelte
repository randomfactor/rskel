<script>
  import { onMount } from 'svelte'
  import { authState, checkSession } from './lib/auth.svelte.js'
  import Home from './views/Home.svelte'
  import Login from './views/Login.svelte'
  import About from './views/About.svelte'

  let currentRoute = $state('home')

  function resolveRoute() {
    const hash = window.location.hash || '#/'
    if (hash === '#/login') {
      currentRoute = 'login'
      return
    }

    if (hash === '#/about') {
      currentRoute = 'about'
      return
    }

    currentRoute = 'home'
  }

  onMount(() => {
    resolveRoute()
    window.addEventListener('hashchange', resolveRoute)
    void checkSession()

    return () => {
      window.removeEventListener('hashchange', resolveRoute)
    }
  })
</script>

{#if authState.isLoading}
  <main class="loading-shell">
    <div class="spinner" aria-live="polite" aria-label="Loading session"></div>
  </main>
{:else if currentRoute === 'login'}
  <Login />
{:else if currentRoute === 'about'}
  <About />
{:else}
  <Home />
{/if}

<style>
  .loading-shell {
    min-height: 100vh;
    display: grid;
    place-items: center;
    background: linear-gradient(180deg, #f8fafc 0%, #eef2ff 100%);
  }

  .spinner {
    width: 52px;
    height: 52px;
    border: 4px solid #e2e8f0;
    border-top-color: #7c3aed;
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
