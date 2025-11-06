<script lang="ts">
  import {
    BaseForm,
    type FormRecord
  } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';
  import { summary } from './schema.svelte';

  interface Props {
    initialValue?: FormRecord;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet<[{ isLoading: boolean }]>;
    isLoading: boolean;
  }

  let { footer, initialValue, onsubmit, isLoading }: Props = $props();

  export const getValue = () => {
    return initialValue;
  };
</script>

<BaseForm
  {footer}
  schema={summary}
  {isLoading}
  onsubmit={() => onsubmit(initialValue as FormRecord)}
>
  <p>General:</p>
  <div class="ml-4">
    <p>
      <span class="text-muted-foreground">Name:</span>
      {initialValue?.name}
    </p>
  </div>
  <p>Resources:</p>
  <div class="ml-4">
    <p>
      <span class="text-muted-foreground">Storage Size:</span>
      {initialValue?.storage_mb}MB
    </p>
  </div>
  <div></div>
</BaseForm>
