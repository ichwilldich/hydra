<script lang="ts">
  import {
    create_deployment,
    delete_deployment,
    list_deployments,
    type DeploymentInfo
  } from '$lib/backend/postgres.svelte';
  import { Plus } from '@lucide/svelte';
  import { Button, Input } from 'positron-components/components/ui';

  let name = $state('');
  let deployments: DeploymentInfo[] = $state([]);

  const reload = () => {
    list_deployments().then((data) => {
      if (!data) return;
      deployments = data;
    });
  };
  reload();

  const create = async () => {
    await create_deployment({
      name,
      storage_mb: 1024
    });

    reload();
  };

  const remove = async (uuid: string) => {
    await delete_deployment(uuid);
    reload();
  };
</script>

<div class="flex w-full p-4">
  <Button class="ml-auto" href="/deployments/postgres/create">
    <Plus />
    Create
  </Button>
</div>
