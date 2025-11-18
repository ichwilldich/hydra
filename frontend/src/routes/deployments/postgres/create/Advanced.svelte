<script lang="ts">
  import {
    BaseForm,
    FormInput,
    FormSwitch,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { advanced } from './schema.svelte';

  interface Props {
    initialValue?: FormValue<typeof advanced>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
  }

  let { initialValue, onsubmit, footer, isLoading }: Props = $props();

  let form: BaseForm<typeof advanced> | undefined = $state();

  export const getValue = () => {
    return form?.getValue();
  };
</script>

<BaseForm
  schema={advanced}
  {onsubmit}
  {footer}
  {initialValue}
  bind:this={form}
  bind:isLoading
>
  {#snippet children({ props })}
    <FormSwitch
      {...props}
      key="allow_alter_system"
      label="Allow ALTER SYSTEM"
    />
    <FormInput
      {...props}
      key="extra_database_parameters"
      label="Extra Database Parameters"
      placeholder="key=value,key2=value2"
    />
  {/snippet}
</BaseForm>
