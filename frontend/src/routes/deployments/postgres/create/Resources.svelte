<script lang="ts">
  import {
    BaseForm,
    FormInput,
    FormSelect,
    type FormValue
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { resources, units } from './schema.svelte';

  interface Props {
    initialValue?: FormValue<typeof resources>;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
  }

  let { initialValue, onsubmit, footer, isLoading }: Props = $props();

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
        class="w-83"
        key="storage_size"
        label="Storage Size"
        placeholder="Enter storage in MB"
        type="number"
      />
      <FormSelect
        {...props}
        key="storage_size_unit"
        label="Storage Unit"
        single={true}
        data={Object.keys(units).map((unit) => ({ value: unit, label: unit }))}
      />
    </div>
  {/snippet}
</BaseForm>
