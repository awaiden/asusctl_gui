<script lang="ts">
  import { Info } from "@lucide/svelte";
  import { useDictionary } from "dialexjs/svelte";

  interface Props {
    message?: string;
  }

  let { message }: Props = $props();
  const common = useDictionary("common");
</script>

<div class="flex items-center gap-2.5 rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-muted)] p-4 text-sm text-[var(--ui-muted-foreground)]">
  <Info size={16} class="shrink-0" />
  <span>{message ?? $common.status.unsupported}</span>
</div>
