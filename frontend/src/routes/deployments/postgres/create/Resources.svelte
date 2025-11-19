<script lang="ts">
  import {
    BaseForm,
    FormInput,
    FormSelect,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { resources, units } from './schema.svelte';
  import { ConnectorType, type SystemInfo } from '$lib/backend/postgres.svelte';

  interface Props {
    initialValue?: FormValue<typeof resources>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
    sys_info?: SystemInfo;
  }

  let { initialValue, onsubmit, footer, isLoading, sys_info }: Props = $props();

  let form: BaseForm<typeof resources> | undefined = $state();

  export const getValue = () => {
    return form?.getValue();
  };
</script>

<BaseForm
  schema={resources}
  {onsubmit}
  {footer}
  {initialValue}
  bind:this={form}
  bind:isLoading
>
  {#snippet children({ props })}
    <div class="flex w-full gap-2">
      <FormInput
        {...props}
        class="w-89"
        key="memory_request_size"
        label="Memory Request"
        placeholder="Enter amount of memory"
        type="number"
      />
      <FormSelect
        {...props}
        class="w-16"
        key="memory_request_size_unit"
        label="Unit"
        single={true}
        data={Object.keys(units).map((unit) => ({ value: unit, label: unit }))}
      />
    </div>
    <div class="flex w-full gap-2">
      <FormInput
        {...props}
        class="w-89"
        key="memory_limit_size"
        label="Memory Limit"
        placeholder="Enter amount of memory"
        type="number"
      />
      <FormSelect
        {...props}
        class="w-16"
        key="memory_limit_size_unit"
        label="Unit"
        single={true}
        data={Object.keys(units).map((unit) => ({ value: unit, label: unit }))}
      />
    </div>
    {#if sys_info?.connector === ConnectorType.Kubernetes}
      <div class="flex w-full justify-between gap-2">
        <FormInput
          {...props}
          class="w-89"
          key="storage_size"
          label="Disk Storage"
          placeholder="Enter amount of storage"
          type="number"
        />
        <FormSelect
          {...props}
          class="w-16"
          key="storage_size_unit"
          label="Unit"
          single={true}
          data={Object.keys(units).map((unit) => ({
            value: unit,
            label: unit
          }))}
        />
      </div>
    {/if}
    <FormInput
      {...props}
      key="cpu_request"
      label="CPU Request (in millicores - 1000m = 1 Core)"
      placeholder="Enter amount of CPU"
      type="number"
    />
    <FormInput
      {...props}
      key="cpu_limit"
      label="CPU Limit (in millicores - 1000m = 1 Core)"
      placeholder="Enter amount of CPU"
      type="number"
    />
  {/snippet}
</BaseForm>
