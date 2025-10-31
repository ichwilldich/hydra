<script lang="ts">
  import '../app.css';
  import AppSidebar from '$lib/components/navbar/sidebar-app/sidebar-app.svelte';
  import {
    ModeWatcher,
    Sidebar,
    Toaster
  } from 'positron-components/components/ui';
  import { page } from '$app/state';
  import { onMount } from 'svelte';
  import { test_token } from '$lib/backend/auth.svelte';
  import { goto } from '$app/navigation';

  interface Props {
    children?: import('svelte').Snippet;
  }

  let { children }: Props = $props();

  onMount(() => {
    test_token().then((valid) => {
      console.log('Token valid:', valid);
      // can also be undefined if there was an error
      if (valid === false) {
        goto('/login');
      }
    });
  });

  const noLayout = ['/login', '/oauth', '/oauth/logout'];
</script>

<ModeWatcher />
<Toaster position="top-right" richColors closeButton />

{#if !noLayout.includes(page.url.pathname)}
  <Sidebar.Provider class="min-h-screen">
    <AppSidebar />
    <Sidebar.Trigger class="absolute top-3 left-3 flex md:hidden" />
    <main class="min-h-screen min-w-0 flex-1">
      {@render children?.()}
    </main>
  </Sidebar.Provider>
{:else}
  {@render children?.()}
{/if}
