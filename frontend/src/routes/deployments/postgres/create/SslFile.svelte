<script lang="ts">
  import {
    FormFile,
    FormInput,
    FormSelect,
    type FormPath,
    type FormPathLeaves,
    type FormValue,
    type SuperForm
  } from 'positron-components/components/form';
  import { CertSource, connection, RefType } from './schema.svelte';
  import { ConnectorType, type SystemInfo } from '$lib/backend/postgres.svelte';
  import FormArea from '$lib/components/form/FormArea.svelte';

  type Value = FormValue<typeof connection>;
  type Key = FormPath<Value>;

  interface Props {
    formData: SuperForm<Value>;
    disabled: boolean;
    initialValue?: Value;
    selectKey: Key;
    textKey: Key;
    fileKey: FormPathLeaves<Value, File>;
    refTypeKey: Key;
    refNameKey: Key;
    refKeyKey: Key;
    hostFilePathKey: Key;
    key?: boolean;
    sys_info?: SystemInfo;
  }

  let {
    selectKey,
    textKey,
    fileKey,
    refTypeKey,
    refNameKey,
    refKeyKey,
    hostFilePathKey,
    initialValue,
    key,
    sys_info,
    ...props
  }: Props = $props();

  //@ts-ignore
  let src = $state(initialValue?.[selectKey]?.[0] as CertSource | undefined);
</script>

<FormSelect
  {...props}
  key={selectKey}
  label={key ? 'Key Source' : 'Certificate Source'}
  single
  data={Object.values(CertSource)
    .filter((v) => {
      if (sys_info?.connector === ConnectorType.Docker) {
        return v !== CertSource.Reference && v !== CertSource.Auto;
      } else if (sys_info?.connector === ConnectorType.Kubernetes) {
        return v !== CertSource.HostFilePath;
      }
      return true;
    })
    .map((v) => ({ label: v, value: v }))}
  onSelectChange={(v) => (src = v[0])}
/>
{#if src === CertSource.Text}
  <FormArea
    {...props}
    key={textKey}
    label={key ? 'Key Content' : 'Certificate Content'}
    placeholder={key
      ? '-----BEGIN PRIVATE KEY-----...'
      : '-----BEGIN CERTIFICATE-----...'}
  />
{:else if src === CertSource.File}
  <FormFile
    {...props}
    key={fileKey}
    label={key ? 'Key File' : 'Certificate File'}
  />
{:else if src === CertSource.Reference}
  <FormSelect
    {...props}
    key={refTypeKey}
    label="Reference Type"
    single
    data={Object.values(RefType).map((v) => ({ label: v, value: v }))}
  />
  <FormInput
    {...props}
    key={refNameKey}
    label="Reference Name"
    placeholder={key ? 'my-key-secret' : 'my-cert-secret'}
  />
  <FormInput
    {...props}
    key={refKeyKey}
    label="Reference Key"
    placeholder={key ? 'tls.key' : 'tls.crt'}
  />
{:else if src === CertSource.HostFilePath}
  <FormInput
    {...props}
    key={hostFilePathKey}
    label="Host File Path"
    placeholder={key ? '/path/to/key.key' : '/path/to/cert.crt'}
  />
{/if}
