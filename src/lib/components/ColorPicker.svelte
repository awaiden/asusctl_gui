<script lang="ts">
  import { Field, FieldLabel } from "@ui-sdk/svelte/field";
  import {
    ColorPicker as UiColorPicker,
    ColorPickerArea,
    ColorPickerAreaThumb,
    ColorPickerInput,
    ColorPickerSlider,
    ColorPickerSliderThumb,
    ColorPickerSwatch,
  } from "@ui-sdk/svelte/color-picker";

  interface Props {
    value?: string;
    label?: string;
    ariaLabel?: string;
    disabled?: boolean;
    onchange?: (v: string) => void;
  }

  let {
    value = $bindable("#6366f1"),
    label,
    ariaLabel,
    disabled = false,
    onchange,
  }: Props = $props();
</script>

<Field>
  {#if label}
    <FieldLabel>{label}</FieldLabel>
  {/if}
  <UiColorPicker
    bind:value
    {disabled}
    aria-label={label ? undefined : ariaLabel}
    onValueChange={(v) => onchange?.(v)}
  >
    <ColorPickerArea>
      <ColorPickerAreaThumb />
    </ColorPickerArea>
    <ColorPickerSlider channel="hue">
      <ColorPickerSliderThumb />
    </ColorPickerSlider>
    <div data-slot="row" class="flex items-center gap-2">
      <ColorPickerSwatch />
      <ColorPickerInput aria-label={label ? `${label} hex` : "Color value"} />
    </div>
  </UiColorPicker>
</Field>
