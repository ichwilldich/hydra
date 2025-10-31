<script lang="ts" module>
  import { Gauge, DatabaseIcon } from '@lucide/svelte';
  // This is sample data.
  const user = {
    name: 'shadcn',
    email: 'm@example.com'
  };
</script>

<script lang="ts">
  import NavGroup from './nav-group.svelte';
  import NavUser from './nav-user.svelte';
  import Header from './header.svelte';
  import { Sidebar } from 'positron-components/components/ui';

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
    <NavUser {...user} />
  </Sidebar.Footer>
  <Sidebar.Rail />
</Sidebar.Root>
