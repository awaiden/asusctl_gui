<script lang="ts" module>
  import { tv } from "tailwind-variants";

  export const cardVariants = tv({
    slots: {
      root: "rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-muted)] p-4",
      title: "mb-3 text-sm font-medium text-[var(--ui-muted-foreground)]",
    },
  });
</script>

<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title?: string;
    class?: string;
    children: Snippet;
  }

  let { title, class: className = "", children }: Props = $props();
  const styles = cardVariants();
</script>

<section class={styles.root({ class: className })}>
  {#if title}<h2 class={styles.title()}>{title}</h2>{/if}
  {@render children()}
</section>
