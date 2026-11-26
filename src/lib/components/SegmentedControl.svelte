<script lang="ts">
  import { Field, FieldLabel } from "@ui-sdk/svelte/field";
  import {
    SegmentedControl as UiSegmentedControl,
    SegmentedControlIndicator,
    SegmentedControlItem,
  } from "@ui-sdk/svelte/segmented-control";

  export interface SegmentedOption {
    value: string;
    label: string;
    disabled?: boolean;
  }

  interface Props {
    value?: string;
    items: SegmentedOption[];
    label?: string;
    ariaLabel?: string;
    disabled?: boolean;
    onchange?: (v: string) => void;
  }

  let {
    value = $bindable(""),
    items,
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
  <UiSegmentedControl
    bind:value
    {disabled}
    aria-label={label ? undefined : ariaLabel}
    class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] p-0.5"
    onValueChange={(v) => {
      if (typeof v === "string") onchange?.(v);
    }}
  >
    <SegmentedControlIndicator class="rounded-[calc(var(--ui-radius)-2px)] border border-[var(--ui-border)] bg-[var(--ui-muted)] shadow-sm" />
    {#each items as item (item.value)}
      <SegmentedControlItem value={item.value} disabled={item.disabled} class="rounded-[calc(var(--ui-radius)-2px)] border-0">
        {item.label}
      </SegmentedControlItem>
    {/each}
  </UiSegmentedControl>
</Field>
