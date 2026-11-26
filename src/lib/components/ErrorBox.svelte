<script lang="ts">
  import { AlertCircle } from "@lucide/svelte";

  interface Props {
    message: string;
  }

  let { message }: Props = $props();
</script>

<div class="flex items-start gap-2.5 rounded-[var(--ui-radius)] border border-[var(--ui-destructive)]/50 bg-[var(--ui-destructive)]/10 p-3 text-sm text-red-700 dark:text-red-200" role="alert">
  <AlertCircle size={16} class="mt-0.5 shrink-0 text-[var(--ui-destructive)]" />
  <span class="min-w-0 break-words">{message}</span>
</div>
