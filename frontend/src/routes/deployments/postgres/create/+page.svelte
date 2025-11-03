<script lang="ts">
  import { CheckIcon } from '@lucide/svelte';
  import { Badge, Button, Card } from 'positron-components/components/ui';
  import type { PageServerData } from './$types';
  import {
    BaseForm,
    FormDialog,
    FormInput,
    type FormType
  } from 'positron-components/components/form';
  import {
    cancelDeployment,
    generalInformation,
    resources
  } from './schema.svelte';
  import { beforeNavigate, goto } from '$app/navigation';
  import type { BeforeNavigate } from '@sveltejs/kit';

  let { data }: { data: PageServerData } = $props();

  interface Stage {
    title: string;
    form: any;
    schema: any;
  }

  let stage = $state(0);
  let cancelOpen = $state(false);

  let stages: Stage[] = [
    {
      title: 'General Information',
      form: data.generalInformation,
      schema: generalInformation
    },
    { title: 'Resources', form: data.resources, schema: resources }
  ];
  let stage_data: (undefined | object)[] = $state(
    Array(stages.length).fill(undefined)
  );
  $inspect(stage_data).with(console.log);

  let attemptedNavigation: BeforeNavigate | undefined = undefined;
  let confirmed = false;
  beforeNavigate((nav) => {
    // allow navigation if it was already confirmed
    if (confirmed) {
      confirmed = false;
      return;
    }

    // don't show the dialog if the user is leaving the page because the browser built-in
    // dialog will be shown instead
    if (nav.type !== 'leave') {
      attemptedNavigation = nav;
      cancelOpen = true;
    }
    nav.cancel();
  });

  const cancelConfirm = () => {
    confirmed = true;
    goto(attemptedNavigation?.to?.url.pathname || '/deployments/postgres');

    return undefined;
  };
</script>

<div class="flex h-full items-center justify-center p-4">
  <Card.Root class="w-120">
    <Card.Header class="flex flex-col gap-4">
      <div class="flex gap-2">
        {#each stages as _, index}
          <Badge
            class={'flex size-6 rounded-full' +
              (stage > index ? ' p-0 cursor-pointer' : '')}
            variant={stage === index ? 'default' : 'outline'}
            onclick={() => {
              if (stage > index) {
                stage_data[stage] = stages[stage].form.data;
                stage = index;
                stages[stage].form.data = {
                  ...stages[stage].form.data,
                  ...stage_data[stage]
                };
              }
            }}
          >
            {#if stage > index}
              <CheckIcon />
            {:else}
              {index + 1}
            {/if}
          </Badge>
        {/each}
      </div>
      <Card.Title>{stages[stage].title}</Card.Title>
    </Card.Header>
    <Card.Content>
      <BaseForm
        form={stages[stage].form}
        schema={stages[stage].schema}
        onsubmit={(form: FormType<any>) => {
          if (stage < stages.length - 1) {
            stage_data[stage] = form.data;
            stage += 1;
          } else {
            // Final submission logic here
          }
          return undefined;
        }}
      >
        {#snippet children({ props })}
          {#if stage === 0}
            <FormInput
              {...props}
              key="name"
              label="Cluster Name"
              placeholder="Enter name"
            />
          {:else if stage === 1}
            <FormInput
              {...props}
              key="cpu"
              label="CPU Cores"
              type="number"
              placeholder="Enter number of CPU cores"
            />
            <FormInput
              {...props}
              key="memory"
              label="Memory (GB)"
              type="number"
              placeholder="Enter amount of memory"
            />
          {/if}
        {/snippet}
        {#snippet footer()}
          <Card.Footer class="w-full gap-2 px-0">
            <Button
              class="cursor-pointer"
              variant="outline"
              disabled={stage === 0}
              onclick={() => {
                if (stage > 0) {
                  stage_data[stage] = stages[stage].form.data;
                  stage -= 1;
                  stages[stage].form.data = {
                    ...stages[stage].form.data,
                    ...stage_data[stage]
                  };
                }
              }}
            >
              Previous
            </Button>
            <Button
              class="ml-auto cursor-pointer"
              variant="outline"
              onclick={() => {
                cancelOpen = true;
              }}
            >
              Cancel
            </Button>
            <Button class="cursor-pointer" type="submit">
              {#if stage === stages.length - 1}Create{:else}Next{/if}
            </Button>
          </Card.Footer>
        {/snippet}
      </BaseForm>
    </Card.Content>
  </Card.Root>
</div>
<FormDialog
  title="Cancel Deployment Creation"
  description="Are you sure you want to cancel creating this deployment? All progress will be lost."
  confirm="Cancel"
  onsubmit={cancelConfirm}
  bind:open={cancelOpen}
  form={data.cancelDeployment}
  schema={cancelDeployment}
/>
