<script lang="ts" module>
  import { tv } from "tailwind-variants";

  export const pageHeaderVariants = tv({
    slots: {
      root: "mb-5 flex items-center justify-between",
      title: "text-xl font-semibold tracking-tight text-[var(--ui-foreground)]",
      icon: "",
    },
    variants: {
      busy: {
        true: {
          icon: "animate-spin",
        },
      },
    },
  });
</script>

<script lang="ts">
  import { RefreshCw } from "@lucide/svelte";
  import { useDictionary } from "dialexjs/svelte";
  import Button from "./Button.svelte";

  interface Props {
    title: string;
    onrefresh?: () => void;
    busy?: boolean;
  }

  let { title, onrefresh, busy = false }: Props = $props();
  const common = useDictionary("common");
  const styles = $derived(pageHeaderVariants({ busy }));
</script>

<header class={styles.root()}>
  <h1 class={styles.title()}>{title}</h1>
  {#if onrefresh}
    <Button variant="ghost" size="sm" onclick={onrefresh} disabled={busy} ariaLabel={$common.status.refresh}>
      <RefreshCw size={16} class={styles.icon()} />
    </Button>
  {/if}
</header>
