<script lang="ts">
  import {
    BaseForm,
    type FormRecord
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { summary } from './schema.svelte';
  import {
    ConnectorType,
    type CreateDeployment,
    type SystemInfo
  } from '$lib/backend/postgres.svelte';

  interface Props {
    initialValue: CreateDeployment;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
    sys_info?: SystemInfo;
  }

  let { footer, initialValue, onsubmit, isLoading, sys_info }: Props = $props();

  export const getValue = () => {
    return initialValue;
  };
</script>

<BaseForm
  {footer}
  schema={summary}
  {isLoading}
  onsubmit={() => onsubmit(initialValue as unknown as FormRecord)}
>
  <div class="space-y-4">
    <div>
      <h3 class="font-semibold">General Information</h3>
      <div class="ml-4 grid grid-cols-2 gap-x-4 text-sm">
        <span class="text-muted-foreground">Name:</span>
        <span>{initialValue?.name}</span>
        {#if sys_info?.connector === ConnectorType.Kubernetes}
          <span class="text-muted-foreground">Namespace:</span>
          <span>{initialValue?.namespace}</span>
        {/if}
        <span class="text-muted-foreground">Version:</span>
        <span>{initialValue?.version}</span>
        <span class="text-muted-foreground">Replicas:</span>
        <span>{initialValue?.replicas}</span>
      </div>
    </div>

    <div>
      <h3 class="font-semibold">Resources (per replica)</h3>
      <div class="ml-4 grid grid-cols-2 gap-x-4 text-sm">
        <span class="text-muted-foreground">Memory Request:</span>
        <span>{initialValue?.resources.memory_request_mb} MB</span>
        <span class="text-muted-foreground">Memory Limit:</span>
        <span>{initialValue?.resources.memory_limit_mb} MB</span>
        <span class="text-muted-foreground">CPU Request:</span>
        <span>{initialValue?.resources.cpu_request_millicores}m</span>
        <span class="text-muted-foreground">CPU Limit:</span>
        <span>{initialValue?.resources.cpu_limit_millicores}m</span>
        {#if sys_info?.connector === ConnectorType.Kubernetes}
          <span class="text-muted-foreground">Storage Size:</span>
          <span>{initialValue?.resources.storage_mb} MB</span>
        {/if}
      </div>
    </div>

    <div>
      <h3 class="font-semibold">Connection</h3>
      <div class="ml-4 grid grid-cols-2 gap-x-4 text-sm">
        <span class="text-muted-foreground">External Access:</span>
        <span
          >{initialValue?.connection.external_access
            ? 'Enabled'
            : 'Disabled'}</span
        >
        <span class="text-muted-foreground">SSL:</span>
        <span
          >{initialValue?.connection.ssl_enabled ? 'Enabled' : 'Disabled'}</span
        >
        {#if initialValue?.connection.ssl_enabled}
          <span class="text-muted-foreground pl-2">Cert Source:</span>
          <span>{initialValue?.connection.ssl_cert.type}</span>
          <span class="text-muted-foreground pl-2">Key Source:</span>
          <span>{initialValue?.connection.ssl_key.type}</span>
          <span class="text-muted-foreground">Custom CA:</span>
          <span
            >{initialValue?.connection.ca_enabled
              ? 'Enabled'
              : 'Disabled'}</span
          >
          {#if initialValue?.connection.ca_enabled}
            <span class="text-muted-foreground pl-2">CA Source:</span>
            <span>{initialValue?.connection.ssl_ca.type}</span>
          {/if}
        {/if}
      </div>
    </div>

    <div>
      <h3 class="font-semibold">Backup Configuration</h3>
      <div class="ml-4 grid grid-cols-2 gap-x-4 text-sm">
        <span class="text-muted-foreground">Backups:</span>
        <span>{initialValue?.backup.enabled ? 'Enabled' : 'Disabled'}</span>
        {#if initialValue?.backup.enabled}
          <span class="text-muted-foreground">Schedule:</span>
          <span>{initialValue?.backup.schedule}</span>
          <span class="text-muted-foreground">Retention:</span>
          <span>{initialValue?.backup.retention_days} days</span>
          <span class="text-muted-foreground">Location:</span>
          <span>{initialValue?.backup.storage_location || 'Default'}</span>
        {/if}
      </div>
    </div>

    <div>
      <h3 class="font-semibold">Monitoring</h3>
      <div class="ml-4 grid grid-cols-2 gap-x-4 text-sm">
        <span class="text-muted-foreground">Monitoring:</span>
        <span>{initialValue?.monitoring.enabled ? 'Enabled' : 'Disabled'}</span>
        {#if initialValue?.monitoring.enabled}
          {#if sys_info?.connector === ConnectorType.Kubernetes}
            <span class="text-muted-foreground">Deploy Resources:</span>
            <span
              >{initialValue?.monitoring.deploy_monitoring ? 'Yes' : 'No'}</span
            >
          {/if}
          <span class="text-muted-foreground">External Access:</span>
          <span
            >{initialValue?.monitoring.external_access
              ? 'Enabled'
              : 'Disabled'}</span
          >
        {/if}
      </div>
    </div>

    <div>
      <h3 class="font-semibold">Advanced</h3>
      <div class="ml-4 grid grid-cols-2 gap-x-4 text-sm">
        <span class="text-muted-foreground">Allow Alter System:</span>
        <span>{initialValue?.advanced.allow_alter_system ? 'Yes' : 'No'}</span>
        {#if initialValue?.advanced.extra_params && Object.keys(initialValue.advanced.extra_params).length > 0}
          <span class="text-muted-foreground">Extra Params:</span>
          <span class="truncate">
            {Object.entries(initialValue.advanced.extra_params)
              .map(([k, v]) => `${k}=${v}`)
              .join(', ')}
          </span>
        {/if}
      </div>
    </div>
  </div>
</BaseForm>
