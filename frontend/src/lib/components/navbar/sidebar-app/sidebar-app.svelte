<script lang="ts">
  import NavGroup from './nav-group.svelte';
  import NavUser from './nav-user.svelte';
  import Header from './header.svelte';
  import { Sidebar } from 'positron-components/components/ui';
  import { Gauge, DatabaseIcon } from '@lucide/svelte';
  import { onMount } from 'svelte';
  import { user_info, type UserInfo } from '$lib/backend/user.svelte';

  let top_nav = [
    {
      title: 'Dashboard',
      url: '/',
      icon: Gauge
    }
  ];

  let deployments = [
    {
      title: 'Postgres',
      url: '/deployments/postgres',
      icon: DatabaseIcon
    }
  ];

  let user: UserInfo | undefined = $state();
  onMount(() => {
    user_info().then((data) => {
      user = data;
    });
  });
</script>

<Sidebar.Root collapsible="icon">
  <Sidebar.Header>
    <Header />
  </Sidebar.Header>
  <Sidebar.Content>
    <NavGroup items={top_nav} />
    <NavGroup items={deployments} title="Deployments" />
  </Sidebar.Content>
  <Sidebar.Footer>
    <NavUser {user} />
  </Sidebar.Footer>
  <Sidebar.Rail />
</Sidebar.Root>
