<script lang="ts" module>
  import { tv } from "tailwind-variants";

  export const rawOutputVariants = tv({
    slots: {
      root: "rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-muted)]",
      trigger:
        "group flex w-full cursor-pointer items-center justify-between rounded-[var(--ui-radius)] px-3 py-2 text-left text-xs font-medium text-[var(--ui-muted-foreground)] transition-colors hover:text-[var(--ui-foreground)] focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-[var(--ui-ring)]",
      chevron: "shrink-0 transition-transform group-data-[open]:rotate-180",
      content:
        "overflow-x-auto border-t border-[var(--ui-border)] p-3 font-mono text-xs whitespace-pre-wrap text-[var(--ui-foreground)]",
    },
  });
</script>

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
  const styles = rawOutputVariants();
</script>

<Collapsible {defaultOpen} class={styles.root()}>
  <CollapsibleTrigger class={styles.trigger()}>
    <span>{label ?? $common.status.rawOutput}</span>
    <ChevronDown size={14} class={styles.chevron()} />
  </CollapsibleTrigger>
  <CollapsiblePanel>
    <pre class={styles.content()}>{text}</pre>
  </CollapsiblePanel>
</Collapsible>
