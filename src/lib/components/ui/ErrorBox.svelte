<script lang="ts" module>
  import { tv } from "tailwind-variants";

  export const errorBoxVariants = tv({
    slots: {
      root: "flex items-center gap-2.5 rounded-[var(--ui-radius)] border border-[var(--ui-destructive)]/50 bg-[var(--ui-destructive)]/10 p-3 text-sm text-red-700 dark:text-red-200",
      icon: "shrink-0 text-[var(--ui-destructive)]",
      message: "min-w-0 flex-1 break-words",
      retry: "ml-auto shrink-0",
    },
  });
</script>

<script lang="ts">
  import { AlertCircle } from "@lucide/svelte";
  import { useDictionary } from "dialexjs/svelte";
  import Button from "./Button.svelte";

  interface Props {
    message: string;
    onretry?: () => void;
  }

  let { message, onretry }: Props = $props();
  const common = useDictionary("common");
  const styles = errorBoxVariants();
</script>

<div class={styles.root()} role="alert">
  <AlertCircle size={16} class={styles.icon()} />
  <span class={styles.message()}>{message}</span>
  {#if onretry}
    <Button variant="secondary" size="sm" class={styles.retry()} onclick={onretry}>{$common.actions.retry}</Button>
  {/if}
</div>
