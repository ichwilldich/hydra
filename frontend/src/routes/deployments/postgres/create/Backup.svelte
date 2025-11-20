<script lang="ts">
  import {
    BaseForm,
    FormInput,
    FormSelect,
    FormSwitch,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { backup } from './schema.svelte';
  import type { SystemInfo } from '$lib/backend/postgres.svelte';

  interface Props {
    initialValue?: FormValue<typeof backup>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
    sys_info?: SystemInfo;
  }

  let { initialValue, onsubmit, footer, isLoading, sys_info }: Props = $props();

  let form: BaseForm<typeof backup> | undefined = $state();
  let backups_enabled = $state(initialValue?.backups_enabled ?? false);

  export const getValue = () => {
    return form?.getValue();
  };
</script>

<BaseForm
  schema={backup}
  {onsubmit}
  {footer}
  {initialValue}
  bind:this={form}
  bind:isLoading
>
  {#snippet children({ props })}
    <FormSwitch
      {...props}
      key="backups_enabled"
      label="Enable Backups"
      onCheckedChange={(checked: boolean) => (backups_enabled = checked)}
    />
    {#if backups_enabled}
      <FormInput
        {...props}
        key="backup_schedule"
        label="Backup Schedule"
        placeholder="Enter backup schedule"
        type="text"
      />
      <FormInput
        {...props}
        key="backup_retention"
        label="Number of Backups to Retain"
        placeholder="Enter backup retention"
        type="number"
      />
      <FormSelect
        {...props}
        key="backup_storage_location"
        label="Backup Storage Location"
        data={sys_info?.backup_locations?.map((location) => ({
          label: location,
          value: location
        })) ?? []}
      />
    {/if}
  {/snippet}
</BaseForm>
