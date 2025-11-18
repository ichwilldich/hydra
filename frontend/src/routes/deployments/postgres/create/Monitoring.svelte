<script lang="ts">
  import {
    BaseForm,
    FormSwitch,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { monitoring } from './schema.svelte';

  interface Props {
    initialValue?: FormValue<typeof monitoring>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
  }

  let { initialValue, onsubmit, footer, isLoading }: Props = $props();

  let form: BaseForm<typeof monitoring> | undefined = $state();
  let monitoring_enabled = $state(initialValue?.monitoring_enabled ?? false);

  export const getValue = () => {
    return form?.getValue();
  };
</script>

<BaseForm
  schema={monitoring}
  {onsubmit}
  {footer}
  {initialValue}
  bind:this={form}
  bind:isLoading
>
  {#snippet children({ props })}
    <FormSwitch
      {...props}
      key="monitoring_enabled"
      label="Enable Monitoring"
      onCheckedChange={(v) => (monitoring_enabled = v)}
    />
    {#if monitoring_enabled}
      <FormSwitch
        {...props}
        key="deploy_monitoring_resources"
        label="Deploy Monitoring Resources"
      />
      <FormSwitch
        {...props}
        key="external_access"
        label="Enable External Access for Monitoring"
      />
    {/if}
  {/snippet}
</BaseForm>
