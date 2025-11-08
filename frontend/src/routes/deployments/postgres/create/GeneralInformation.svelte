<script lang="ts">
  import {
    BaseForm,
    FormInput,
    FormSelect,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { generalInformation, PostgresVersion } from './schema.svelte';
  import { ConnectorType, type SystemInfo } from '$lib/backend/postgres.svelte';

  interface Props {
    initialValue?: FormValue<typeof generalInformation>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
    sys_info?: SystemInfo;
  }

  let { initialValue, onsubmit, footer, isLoading, sys_info }: Props = $props();

  let form: BaseForm<typeof generalInformation> | undefined = $state();

  export const getValue = () => {
    return form?.getValue();
  };
</script>

<BaseForm
  schema={generalInformation}
  {onsubmit}
  {footer}
  {initialValue}
  bind:this={form}
  bind:isLoading
>
  {#snippet children({ props })}
    <FormInput
      {...props}
      key="name"
      label="Cluster Name"
      placeholder="Enter name"
    />
    {#if sys_info?.connector === ConnectorType.Kubernetes}
      <FormSelect
        {...props}
        key="namespace"
        label="Kubernetes Namespace"
        data={sys_info.namespaces.map((ns) => ({ label: ns, value: ns }))}
      />
    {/if}
    <FormSelect
      {...props}
      key="version"
      label="PostgreSQL Version"
      single
      data={Object.values(PostgresVersion).map((version) => ({
        label: version,
        value: version
      }))}
    />
    <FormInput
      {...props}
      key="replicas"
      label="Number of Replicas"
      type="number"
    />
  {/snippet}
</BaseForm>
