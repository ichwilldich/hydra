<script lang="ts">
  import {
    BaseForm,
    FormSwitch,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { connection, CertSource } from './schema.svelte';
  import SslFile from './SslFile.svelte';
  import { ConnectorType, type SystemInfo } from '$lib/backend/postgres.svelte';

  interface Props {
    initialValue: FormValue<typeof connection>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
    sys_info?: SystemInfo;
  }

  let { initialValue, onsubmit, footer, isLoading, sys_info }: Props = $props();

  let effectiveInitialValue = $derived.by(() => {
    let val = { ...initialValue };
    if (sys_info?.connector === ConnectorType.Docker) {
      if (!val.ssl_cert_source) val.ssl_cert_source = [CertSource.File];
      if (!val.ssl_key_source) val.ssl_key_source = [CertSource.File];
      if (!val.ssl_ca_source) val.ssl_ca_source = [CertSource.File];
    }
    return val;
  });

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
  initialValue={effectiveInitialValue}
  bind:this={form}
  bind:isLoading
  enctype="multipart/form-data"
>
  {#snippet children({ props })}
    <FormSwitch
      {...props}
      key="external_access"
      label="Enable External Access"
    />
    {#if sys_info?.connector === ConnectorType.Docker}
      <p class="text-muted-foreground text-sm">
        Note: This only changes the IP bind, it does not open firewall ports or
        route traffic.
      </p>
    {/if}
    <FormSwitch
      {...props}
      key="ssl_enabled"
      label="Enable SSL"
      onCheckedChange={(v) => (ssl_enabled = v)}
    />
    {#if ssl_enabled}
      <SslFile
        {...props}
        selectKey="ssl_cert_source"
        textKey="ssl_cert_text"
        fileKey="ssl_cert_file"
        refTypeKey="ssl_cert_ref_type"
        refNameKey="ssl_cert_ref_name"
        refKeyKey="ssl_cert_ref_key"
        hostFilePathKey="ssl_cert_host_file_path"
        initialValue={effectiveInitialValue}
        {sys_info}
      />
      <SslFile
        {...props}
        selectKey="ssl_key_source"
        textKey="ssl_key_text"
        fileKey="ssl_key_file"
        refTypeKey="ssl_key_ref_type"
        refNameKey="ssl_key_ref_name"
        refKeyKey="ssl_key_ref_key"
        hostFilePathKey="ssl_key_host_file_path"
        key={true}
        initialValue={effectiveInitialValue}
        {sys_info}
      />
      <FormSwitch
        {...props}
        key="ssl_ca_enabled"
        label="Enable Custom CA"
        onCheckedChange={(v) => (ssl_ca_enabled = v)}
      />
      {#if ssl_ca_enabled}
        <SslFile
          {...props}
          selectKey="ssl_ca_source"
          textKey="ssl_ca_text"
          fileKey="ssl_ca_file"
          refTypeKey="ssl_ca_ref_type"
          refNameKey="ssl_ca_ref_name"
          refKeyKey="ssl_ca_ref_key"
          hostFilePathKey="ssl_ca_host_file_path"
          initialValue={effectiveInitialValue}
          {sys_info}
        />
      {/if}
    {/if}
  {/snippet}
</BaseForm>
