<script lang="ts">
  import {
    BaseForm,
    FormInput,
    FormSwitch,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { backup } from './schema.svelte';

  interface Props {
    initialValue?: FormValue<typeof backup>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
  }

  let { initialValue, onsubmit, footer, isLoading }: Props = $props();

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
      <FormInput
        {...props}
        key="backup_storage_location"
        label="Backup Storage Location (TODO)"
        placeholder="Enter backup storage location"
        type="text"
      />
    {/if}
  {/snippet}
</BaseForm>
