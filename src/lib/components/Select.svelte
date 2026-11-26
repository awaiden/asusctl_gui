<script lang="ts">
  import { Field, FieldLabel } from "@ui-sdk/svelte/field";
  import {
    Select as UiSelect,
    SelectContent,
    SelectItem,
    SelectItemIndicator,
    SelectTrigger,
    SelectValue,
  } from "@ui-sdk/svelte/select";

  export interface SelectOption {
    value: string;
    label: string;
    disabled?: boolean;
  }

  interface Props {
    value?: string;
    items: SelectOption[];
    label?: string;
    ariaLabel?: string;
    placeholder?: string;
    disabled?: boolean;
    onchange?: (v: string) => void;
  }

  let {
    value = $bindable(""),
    items,
    label,
    ariaLabel,
    placeholder = "Select an option",
    disabled = false,
    onchange,
  }: Props = $props();
</script>

<Field>
  {#if label}
    <FieldLabel>{label}</FieldLabel>
  {/if}
  <UiSelect
    bind:value
    {items}
    {placeholder}
    {disabled}
    onValueChange={(v) => {
      if (typeof v === "string") onchange?.(v);
    }}
  >
    <SelectTrigger
      aria-label={label ? undefined : ariaLabel}
      class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 text-sm text-[var(--ui-foreground)] transition-colors hover:border-[var(--ui-input)]"
    >
      <SelectValue />
    </SelectTrigger>
    <SelectContent class="z-50 rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] p-1 shadow-xl ring-1 ring-black/10 dark:ring-white/10">
      {#each items as item (item.value)}
        <SelectItem value={item.value} disabled={item.disabled}>
          <span>{item.label}</span>
          <SelectItemIndicator />
        </SelectItem>
      {/each}
    </SelectContent>
  </UiSelect>
</Field>
