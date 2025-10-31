<script lang="ts">
  import { Sidebar } from 'positron-components/components/ui';
  import { Database } from '@lucide/svelte';

  // @ts-ignore this is injected at build time via Vite's define option
  let version = __version__;
  let sidebar = Sidebar.useSidebar();
</script>

<Sidebar.Menu>
  <Sidebar.MenuItem>
    <Sidebar.MenuButton
      size="lg"
      class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
      onclick={sidebar.toggle}
    >
      <div
        class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
      >
        <Database class="size-4" />
      </div>
      <div class="grid flex-1 text-left text-sm leading-tight">
        <span class="truncate font-medium"> Hydra </span>
        <span class="truncate text-xs">{version}</span>
      </div>
    </Sidebar.MenuButton>
  </Sidebar.MenuItem>
</Sidebar.Menu>
