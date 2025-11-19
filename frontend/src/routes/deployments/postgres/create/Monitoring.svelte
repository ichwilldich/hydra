<script lang="ts">
  import {
    BaseForm,
    FormSwitch,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { monitoring } from './schema.svelte';
  import { ConnectorType, type SystemInfo } from '$lib/backend/postgres.svelte';

  interface Props {
    initialValue?: FormValue<typeof monitoring>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
    sys_info?: SystemInfo;
  }

  let { initialValue, onsubmit, footer, isLoading, sys_info }: Props = $props();

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
      onCheckedChange={(v: boolean) => (monitoring_enabled = v)}
    />
    {#if monitoring_enabled}
      {#if sys_info?.connector === ConnectorType.Kubernetes}
        <FormSwitch
          {...props}
          key="deploy_monitoring_resources"
          label="Deploy Monitoring Resources"
        />
      {/if}
      <FormSwitch
        {...props}
        key="monitoring_external_access"
        label="Enable External Access for Monitoring"
      />
      {#if sys_info?.connector === ConnectorType.Docker}
        <p class="text-muted-foreground text-sm">
          Note: This only changes the IP bind, it does not manage any firewall.
        </p>
      {/if}
    {/if}
  {/snippet}
</BaseForm>
