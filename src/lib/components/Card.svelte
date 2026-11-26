<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title?: string;
    children: Snippet;
  }

  let { title, children }: Props = $props();
</script>

<section class="mb-4 rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-muted)] p-4">
  {#if title}<h2 class="mb-3 text-sm font-medium text-[var(--ui-muted-foreground)]">{title}</h2>{/if}
  {@render children()}
</section>
