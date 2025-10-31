<script lang="ts">
  import {
    Avatar,
    DropdownMenu,
    Sidebar,
    Skeleton
  } from 'positron-components/components/ui';
  import BellIcon from '@lucide/svelte/icons/bell';
  import ChevronsUpDownIcon from '@lucide/svelte/icons/chevrons-up-down';
  import LogOutIcon from '@lucide/svelte/icons/log-out';
  import { Settings } from '@lucide/svelte';
  import { logout } from '$lib/backend/auth.svelte';
  import { goto } from '$app/navigation';
  import type { UserInfo } from '$lib/backend/user.svelte';

  interface Props {
    user?: UserInfo;
  }

  let { user }: Props = $props();
  let name_short = $derived(user?.name.slice(0, 2).toUpperCase());

  let sidebar = Sidebar.useSidebar();

  const logout_user = async () => {
    await logout();
    goto('/login');
  };
</script>

<Sidebar.Menu>
  <Sidebar.MenuItem>
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        {#snippet child({ props }: { props: Record<string, unknown> })}
          <Sidebar.MenuButton
            size="lg"
            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            {...props}
          >
            {@render user_snippet()}
            <ChevronsUpDownIcon class="ml-auto size-4" />
          </Sidebar.MenuButton>
        {/snippet}
      </DropdownMenu.Trigger>
      <DropdownMenu.Content
        class="w-(--bits-dropdown-menu-anchor-width) min-w-56 rounded-lg"
        side={sidebar.isMobile ? 'bottom' : 'right'}
        align="end"
        sideOffset={4}
      >
        <DropdownMenu.Label class="p-0 font-normal">
          <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
            {@render user_snippet()}
          </div>
        </DropdownMenu.Label>
        <DropdownMenu.Separator />
        <DropdownMenu.Group>
          <DropdownMenu.Item onclick={() => goto('/account')}>
            <Settings />
            Account
          </DropdownMenu.Item>
          <DropdownMenu.Item>
            <BellIcon />
            Notifications
          </DropdownMenu.Item>
        </DropdownMenu.Group>
        <DropdownMenu.Separator />
        <DropdownMenu.Item onclick={logout_user}>
          <LogOutIcon />
          Log out
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </Sidebar.MenuItem>
</Sidebar.Menu>

{#snippet user_snippet()}
  <Avatar.Root class="size-8 rounded-lg">
    <Avatar.Image src={''} alt={user?.name ?? '?'} />
    <Avatar.Fallback class="rounded-lg">{name_short}</Avatar.Fallback>
  </Avatar.Root>
  <div class="grid flex-1 text-left text-sm leading-tight">
    {#if user}
      <span class="truncate font-medium">{user.name}</span>
      <span class="truncate text-xs">{user.email}</span>
    {:else}
      <Skeleton class="h-4 w-20" />
      <Skeleton class="mt-1 h-3 w-24" />
    {/if}
  </div>
{/snippet}
