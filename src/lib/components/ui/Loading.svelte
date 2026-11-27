<script lang="ts" module>
  import { tv } from "tailwind-variants";

  export const loadingVariants = tv({
    slots: {
      root: "flex items-center gap-2 py-2 text-sm text-[var(--ui-muted-foreground)]",
      spinner: "animate-spin",
    },
  });
</script>

<script lang="ts">
  import { LoaderCircle } from "@lucide/svelte";
  import { useDictionary } from "dialexjs/svelte";

  interface Props {
    label?: string;
  }

  let { label }: Props = $props();
  const common = useDictionary("common");
  const styles = loadingVariants();
</script>

<div class={styles.root()} role="status" aria-live="polite">
  <LoaderCircle size={16} class={styles.spinner()} />
  <span>{label ?? $common.status.loading}</span>
</div>
