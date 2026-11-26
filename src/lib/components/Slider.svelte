<script lang="ts">
  import { Field, FieldLabel } from "@ui-sdk/svelte/field";
  import {
    Slider as UiSlider,
    SliderControl,
    SliderIndicator,
    SliderThumb,
    SliderTrack,
    SliderValue,
  } from "@ui-sdk/svelte/slider";

  interface Props {
    value?: number;
    min?: number;
    max?: number;
    step?: number;
    label: string;
    disabled?: boolean;
    onchange?: (v: number) => void;
  }

  let {
    value = $bindable(0),
    min = 0,
    max = 100,
    step = 1,
    label,
    disabled = false,
    onchange,
  }: Props = $props();
</script>

<Field>
  <UiSlider
    bind:value
    {min}
    {max}
    {step}
    {disabled}
    onValueCommit={(v) => {
      if (typeof v === "number") onchange?.(v);
    }}
  >
    <div data-slot="row" class="mb-1 flex items-center justify-between text-sm">
      <FieldLabel>{label}</FieldLabel>
      <SliderValue class="text-[var(--ui-muted-foreground)] tabular-nums" />
    </div>
    <SliderControl>
      <SliderTrack class="h-2 rounded-full bg-zinc-300 dark:bg-zinc-700/90">
        <SliderIndicator class="rounded-full bg-[var(--ui-primary)]" />
        <SliderThumb aria-label={label} />
      </SliderTrack>
    </SliderControl>
  </UiSlider>
</Field>
