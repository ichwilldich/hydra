<script lang="ts">
  import { ArrowLeft, ArrowRight, Ban, CheckIcon, Plus } from '@lucide/svelte';
  import {
    Badge,
    Button,
    Card,
    Spinner
  } from 'positron-components/components/ui';
  import {
    BaseForm,
    FormDialog,
    type FormRecord
  } from 'positron-components/components/form';
  import { cancelDeployment, reformatData } from './schema.svelte';
  import { beforeNavigate, goto } from '$app/navigation';
  import type { BeforeNavigate } from '@sveltejs/kit';
  import type {
    Component,
    ComponentProps,
    Snippet,
    SvelteComponent
  } from 'svelte';
  import GeneralInformation from './GeneralInformation.svelte';
  import Resources from './Resources.svelte';
  import {
    create_deployment,
    system_info,
    type SystemInfo
  } from '$lib/backend/postgres.svelte';
  import Summary from './Summary.svelte';

  interface StageProps {
    initialValue?: any;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
  }

  type StageComponent = Component<
    StageProps,
    { getValue: () => object | undefined }
  >;

  interface Stage {
    title: string;
    content: StageComponent;
    data: object;
  }

  let stage = $state(0);
  let cancelOpen = $state(false);
  let form: undefined | SvelteComponent = $state();
  let isLoading = $state(true);
  let sys_info: SystemInfo | undefined = $state();
  system_info().then((info) => {
    sys_info = info;
    isLoading = false;
  });

  let stages: Stage[] = [
    {
      title: 'General Information',
      content: GeneralInformation,
      data: {}
    },
    {
      title: 'Resources',
      content: Resources,
      data: {}
    },
    {
      title: 'Summary',
      content: Summary,
      data: {}
    }
  ];

  const gotoStep = (step: number) => {
    stages[stage].data = form?.getValue() || {};
    stage = step;
  };

  const submit = async (form: FormRecord) => {
    stages[stage].data = form;
    if (stage < stages.length - 2) {
      stage += 1;
    } else if (stage === stages.length - 2) {
      stage += 1;
      let rawData = stages.reduce((acc, s) => ({ ...acc, ...s.data }), {});
      let data = reformatData(rawData);
      stages[stage].data = data;
    } else {
      let res = await create_deployment(form as any);
      if (res) {
        return { error: 'Error creating deployment.' };
      } else {
        setTimeout(() => {
          confirmed = true;
          goto('/deployments/postgres');
        });
      }
    }
    return undefined;
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
        bind:this={form}
        initialValue={current.data}
        onsubmit={submit}
        bind:isLoading
      >
        {#snippet footer({ isLoading })}
          <Card.Footer class="w-full gap-2 px-0">
            <Button
              class="cursor-pointer"
              variant="outline"
              disabled={stage === 0 || isLoading}
              onclick={() => {
                if (stage > 0) {
                  gotoStep(stage - 1);
                }
              }}
            >
              <ArrowLeft />
              Previous
            </Button>
            <Button
              class="ml-auto cursor-pointer"
              variant="outline"
              disabled={isLoading}
              onclick={() => {
                cancelOpen = true;
              }}
            >
              <Ban />
              Cancel
            </Button>
            <Button class="cursor-pointer" type="submit" disabled={isLoading}>
              {#if stage === stages.length - 1}
                Create
                {#if isLoading}
                  <Spinner />
                {:else}
                  <Plus />
                {/if}
              {:else if stage === stages.length - 2}
                Summary
                <ArrowRight />
              {:else}
                Next
                <ArrowRight />
              {/if}
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
  schema={cancelDeployment}
/>
