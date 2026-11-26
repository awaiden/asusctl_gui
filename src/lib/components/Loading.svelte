<script lang="ts">
  import { LoaderCircle } from "@lucide/svelte";
  import { useDictionary } from "dialexjs/svelte";

  interface Props {
    label?: string;
  }

  let { label }: Props = $props();
  const common = useDictionary("common");
</script>

<div class="flex items-center gap-2 py-2 text-sm text-[var(--ui-muted-foreground)]" role="status" aria-live="polite">
  <LoaderCircle size={16} class="animate-spin" />
  <span>{label ?? $common.status.loading}</span>
</div>
