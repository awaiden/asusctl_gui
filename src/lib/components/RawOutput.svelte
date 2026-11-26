<script lang="ts">
  import { ChevronDown } from "@lucide/svelte";
  import { useDictionary } from "dialexjs/svelte";
  import {
    Collapsible,
    CollapsiblePanel,
    CollapsibleTrigger,
  } from "@ui-sdk/svelte/collapsible";

  interface Props {
    text: string;
    label?: string;
    defaultOpen?: boolean;
  }

  let { text, label, defaultOpen = false }: Props = $props();
  const common = useDictionary("common");
</script>

<Collapsible {defaultOpen} class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-muted)]">
  <CollapsibleTrigger class="group flex w-full items-center justify-between px-3 py-2 text-left text-xs font-medium text-[var(--ui-muted-foreground)] transition-colors hover:text-[var(--ui-foreground)]">
    <span>{label ?? $common.status.rawOutput}</span>
    <ChevronDown size={14} class="transition-transform group-data-[open]:rotate-180" />
  </CollapsibleTrigger>
  <CollapsiblePanel>
    <pre class="overflow-x-auto border-t border-[var(--ui-border)] p-3 font-mono text-xs whitespace-pre-wrap text-[var(--ui-foreground)]">{text}</pre>
  </CollapsiblePanel>
</Collapsible>
