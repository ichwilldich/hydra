<script lang="ts">
  import {
    delete_deployment,
    list_deployments,
    type DeploymentInfo
  } from '$lib/backend/postgres.svelte';
  import { Plus } from '@lucide/svelte';
  import { FormDialog } from 'positron-components/components/form';
  import { Button, ScrollArea } from 'positron-components/components/ui';
  import { deleteDeployment } from './schema.svelte';

  let deployments: DeploymentInfo[] = $state([]);
  let deleteOpen = $state(false);
  let toDeleteName = $state('');
  let toDeleteUUID = $state('');

  const reload = () => {
    list_deployments().then((data) => {
      if (!data) return;
      deployments = data;
    });
  };
  reload();

  const deleteConfirm = async () => {
    if (toDeleteUUID) {
      let res = await delete_deployment(toDeleteUUID);
      if (res) {
        return { error: 'Error deleting deployment.' };
      }
      reload();
    }
  };
</script>

<div class="flex flex-col gap-4">
  <div class="flex w-full px-4 pt-4">
    <Button class="ml-auto" href="/deployments/postgres/create">
      <Plus />
      Create
    </Button>
  </div>
  <ScrollArea.ScrollArea class="mx-4 mb-4 min-h-0 grow">
    <div
      class="grid size-full auto-rows-min grid-cols-[repeat(auto-fill,minmax(14rem,1fr))] gap-2"
    >
      {#each deployments as deployment}
        <div class="flex w-80 rounded-md border p-4">
          <div class="flex flex-col">
            <p class="font-bold">{deployment.name}</p>
            <p class="text-muted-foreground">Postgres Deployment</p>
          </div>
          <Button
            variant="destructive"
            class="ml-auto cursor-pointer"
            onclick={() => {
              toDeleteName = deployment.name;
              toDeleteUUID = deployment.uuid;
              deleteOpen = true;
            }}
          >
            Delete
          </Button>
        </div>
      {:else}
        <div
          class="col-span-full flex flex-col items-center justify-center gap-2 py-8"
        >
          <p class="text-muted-foreground">No deployments found.</p>
        </div>
      {/each}
    </div>
  </ScrollArea.ScrollArea>
</div>
<FormDialog
  title="Delete Deployment"
  description={`Are you sure you want to delete the deployment "${toDeleteName}"? This action cannot be undone.`}
  confirm="Delete"
  confirmVariant="destructive"
  onsubmit={deleteConfirm}
  bind:open={deleteOpen}
  schema={deleteDeployment}
/>
