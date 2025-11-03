<script lang="ts">
  import { CheckIcon } from '@lucide/svelte';
  import { Badge, Button, Card } from 'positron-components/components/ui';
  import type { PageServerData } from './$types';
  import {
    BaseForm,
    FormDialog,
    type FormType
  } from 'positron-components/components/form';
  import {
    cancelDeployment,
    generalInformation,
    resources
  } from './schema.svelte';
  import { beforeNavigate, goto } from '$app/navigation';
  import type { BeforeNavigate } from '@sveltejs/kit';
  import type { Component, ComponentProps, Snippet } from 'svelte';
  import GeneralInformation from './GeneralInformation.svelte';
  import Resources from './Resources.svelte';

  let { data }: { data: PageServerData } = $props();

  interface StageProps {
    form: any;
    schema: any;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet;
  }

  interface Stage {
    title: string;
    form: any;
    schema: any;
    content?: Component<StageProps>;
  }

  let stage = $state(0);
  let cancelOpen = $state(false);

  let stages: Stage[] = [
    {
      title: 'General Information',
      form: data.generalInformation,
      schema: generalInformation,
      content: GeneralInformation
    },
    {
      title: 'Resources',
      form: data.resources,
      schema: resources,
      content: Resources
    }
  ];
  let stage_data: (undefined | object)[] = $state(
    Array(stages.length).fill(undefined)
  );
  $inspect(stage_data).with(console.log);

  const gotoStep = (step: number) => {
    console.log(stages[stage].form.data);
    stage_data[stage] = stages[stage].form.data;
    stage = step;
    stages[stage].form.data = {
      ...stages[stage].form.data,
      ...stage_data[stage]
    };
  };

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
              (stage > index ? ' cursor-pointer p-0' : '')}
            variant={stage === index ? 'default' : 'outline'}
            onclick={() => {
              if (stage > index) {
                gotoStep(index);
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
      {@const current = stages[stage]}
      <current.content
        form={current.form}
        schema={current.schema}
        onsubmit={(form: FormType<any>) => {
          if (stage < stages.length - 1) {
            stage_data[stage] = form.data;
            stage += 1;
            stages[stage].form.data = {
              ...stages[stage].form.data,
              ...stage_data[stage]
            };
          } else {
            // Final submission logic here
          }
          return undefined;
        }}
      >
        {#snippet footer()}
          <Card.Footer class="w-full gap-2 px-0">
            <Button
              class="cursor-pointer"
              variant="outline"
              disabled={stage === 0}
              onclick={() => {
                if (stage > 0) {
                  gotoStep(stage - 1);
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
      </current.content>
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
