<script lang="ts" module>
  import { tv } from "tailwind-variants";

  export const sliderVariants = tv({
    slots: {
      row: "mb-1 flex items-center justify-between text-sm",
      value: "text-[var(--ui-muted-foreground)] tabular-nums",
      track: "h-2 rounded-full bg-zinc-300 dark:bg-zinc-700/90",
      indicator: "rounded-full bg-[var(--ui-primary)]",
    },
  });
</script>

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
  const styles = sliderVariants();
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
    <div data-slot="row" class={styles.row()}>
      <FieldLabel>{label}</FieldLabel>
      <SliderValue class={styles.value()} />
    </div>
    <SliderControl>
      <SliderTrack class={styles.track()}>
        <SliderIndicator class={styles.indicator()} />
        <SliderThumb aria-label={label} />
      </SliderTrack>
    </SliderControl>
  </UiSlider>
</Field>
