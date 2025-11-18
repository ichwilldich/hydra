<script lang="ts">
  import {
    BaseForm,
    FormInput,
    FormSelect,
    FormSwitch,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { connection, CertSource, RefType } from './schema.svelte';

  interface Props {
    initialValue?: FormValue<typeof connection>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
  }

  let { initialValue, onsubmit, footer, isLoading }: Props = $props();

  let form: BaseForm<typeof connection> | undefined = $state();
  let ssl_enabled = $state(initialValue?.ssl_enabled ?? false);
  let ssl_ca_enabled = $state(initialValue?.ssl_ca_enabled ?? false);

  export const getValue = () => {
    return form?.getValue();
  };
</script>

<BaseForm
  schema={connection}
  {onsubmit}
  {footer}
  {initialValue}
  bind:this={form}
  bind:isLoading
>
  {#snippet children({ props })}
    <FormSwitch
      {...props}
      key="external_access"
      label="Enable External Access"
    />
    <FormSwitch
      {...props}
      key="ssl_enabled"
      label="Enable SSL"
      onCheckedChange={(v) => (ssl_enabled = v)}
    />
    {#if ssl_enabled}
      <FormSelect
        {...props}
        key="ssl_cert_source"
        label="Certificate Source"
        single
        data={Object.values(CertSource).map((v) => ({ label: v, value: v }))}
      />
      <!-- TODO: Add conditional fields for CertSource specific inputs if needed -->
      <!-- For now, just showing the source selector as a start -->
      
      <FormSelect
        {...props}
        key="ssl_key_source"
        label="Key Source"
        single
        data={Object.values(CertSource).map((v) => ({ label: v, value: v }))}
      />

      <FormSwitch
        {...props}
        key="ssl_ca_enabled"
        label="Enable Custom CA"
        onCheckedChange={(v) => (ssl_ca_enabled = v)}
      />
      {#if ssl_ca_enabled}
        <FormSelect
            {...props}
            key="ssl_ca_source"
            label="CA Source"
            single
            data={Object.values(CertSource).map((v) => ({ label: v, value: v }))}
        />
      {/if}
    {/if}
  {/snippet}
</BaseForm>
