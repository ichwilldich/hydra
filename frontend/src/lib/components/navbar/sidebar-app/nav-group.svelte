<script lang="ts">
  import { Sidebar } from 'positron-components/components/ui';
  import type { Component } from 'svelte';

  interface Item {
    title: string;
    url: string;
    icon: Component;
  }

  interface Props {
    items: Item[];
    title?: string;
  }

  let { items, title }: Props = $props();
</script>

{#if !title}
  {@render menu({ class: 'px-2' })}
{:else}
  <Sidebar.Group>
    <Sidebar.GroupLabel>{title}</Sidebar.GroupLabel>
    {@render menu()}
  </Sidebar.Group>
{/if}

{#snippet menu({ class: className }: { class?: string } = {})}
  <Sidebar.Menu class={className}>
    {#each items as item (item.title)}
      <Sidebar.MenuItem>
        <a href={item.url}>
          <Sidebar.MenuButton tooltipContent={item.title}>
            {#if item.icon}
              <item.icon />
            {/if}
            <span>{item.title}</span>
          </Sidebar.MenuButton>
        </a>
      </Sidebar.MenuItem>
    {/each}
  </Sidebar.Menu>
{/snippet}
