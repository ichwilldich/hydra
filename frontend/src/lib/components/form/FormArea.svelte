<script lang="ts" generics="S extends FormRecord = FormRecord">
  import { type FormPath, type SuperForm } from 'sveltekit-superforms';
  import type { HTMLTextareaAttributes } from 'svelte/elements';
  import type { WithElementRef } from 'bits-ui';
  import { type FormRecord } from 'positron-components/components/form';
  import { Form, Textarea } from 'positron-components/components/ui';

  type InputProps = WithElementRef<Omit<HTMLTextareaAttributes, 'type'>>;

  interface Props {
    formData: SuperForm<S>;
    key: FormPath<S>;
    label: string;
    disabled?: boolean;
  }

  let {
    formData: form,
    key,
    label,
    disabled,
    ...restProps
  }: InputProps & Props = $props();

  let formData = $derived(form.form as any);
</script>

<Form.Field {form} name={key} class="gap-1/2 grid">
  <Form.Control>
    {#snippet children({ props })}
      <Form.Label>{label}</Form.Label>
      <Textarea
        {disabled}
        {...props}
        {...restProps}
        bind:value={$formData[key]}
      />
    {/snippet}
  </Form.Control>
  <Form.FieldErrors />
</Form.Field>
