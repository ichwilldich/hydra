<script lang="ts">
  import {
    BaseForm,
    FormInput,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { generalInformation } from './schema.svelte';

  interface Props {
    initialValue?: FormValue<typeof generalInformation>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
  }

  let { initialValue, onsubmit, footer, isLoading }: Props = $props();

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
  {/snippet}
</BaseForm>
