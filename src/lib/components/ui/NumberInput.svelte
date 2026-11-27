<script lang="ts" module>
  import { tv } from "tailwind-variants";

  export const numberInputVariants = tv({
    slots: {
      control: "tabular-nums",
    },
  });
</script>

<script lang="ts">
  import { Minus, Plus } from "@lucide/svelte";
  import { useDictionary } from "dialexjs/svelte";
  import { Field, FieldLabel } from "@ui-sdk/svelte/field";
  import {
    NumberInput as UiNumberInput,
    NumberInputControl,
    NumberInputDecrement,
    NumberInputGroup,
    NumberInputIncrement,
  } from "@ui-sdk/svelte/number-input";

  interface Props {
    value?: number | null;
    min?: number;
    max?: number;
    step?: number;
    label?: string;
    ariaLabel?: string;
    disabled?: boolean;
    onchange?: (v: number | null) => void;
  }

  let {
    value = $bindable<number | null>(0),
    min,
    max,
    step = 1,
    label,
    ariaLabel,
    disabled = false,
    onchange,
  }: Props = $props();
  const common = useDictionary("common");
  const styles = numberInputVariants();
</script>

<Field>
  {#if label}
    <FieldLabel>{label}</FieldLabel>
  {/if}
  <UiNumberInput
    bind:value
    {min}
    {max}
    {step}
    {disabled}
    onValueCommit={(v) => onchange?.(v)}
  >
    <NumberInputGroup>
      <NumberInputDecrement aria-label={$common.actions.decrease}><Minus size={14} /></NumberInputDecrement>
      <NumberInputControl aria-label={label ? undefined : ariaLabel} class={styles.control()} />
      <NumberInputIncrement aria-label={$common.actions.increase}><Plus size={14} /></NumberInputIncrement>
    </NumberInputGroup>
  </UiNumberInput>
</Field>
