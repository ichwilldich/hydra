<script lang="ts">
  import { BaseForm, FormInput } from 'positron-components/components/form';
  import type { ComponentProps, Snippet } from 'svelte';

  interface Props {
    form: any;
    schema: any;
    onsubmit: ComponentProps<typeof BaseForm>['onsubmit'];
    footer: Snippet;
  }

  let { form, schema, onsubmit, footer }: Props = $props();
</script>

<BaseForm {form} {schema} {onsubmit} {footer}>
  {#snippet children({ props })}
    <FormInput
      {...props}
      key="name"
      label="Cluster Name"
      placeholder="Enter name"
    />
  {/snippet}
</BaseForm>
