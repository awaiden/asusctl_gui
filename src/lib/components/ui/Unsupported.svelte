<script lang="ts" module>
  import { tv } from "tailwind-variants";

  export const unsupportedVariants = tv({
    slots: {
      root: "flex items-center gap-2.5 rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-muted)] p-4 text-sm text-[var(--ui-muted-foreground)]",
      icon: "shrink-0",
    },
  });
</script>

<script lang="ts">
  import { Info } from "@lucide/svelte";
  import { useDictionary } from "dialexjs/svelte";

  interface Props {
    message?: string;
  }

  let { message }: Props = $props();
  const common = useDictionary("common");
  const styles = unsupportedVariants();
</script>

<div class={styles.root()}>
  <Info size={16} class={styles.icon()} />
  <span>{message ?? $common.status.unsupported}</span>
</div>
