<script lang="ts">
  import { RefreshCw } from "@lucide/svelte";
  import { useDictionary } from "dialexjs/svelte";
  import Button from "$lib/components/Button.svelte";

  interface Props {
    title: string;
    onrefresh?: () => void;
    busy?: boolean;
  }

  let { title, onrefresh, busy = false }: Props = $props();
  const common = useDictionary("common");
</script>

<header class="mb-4 flex items-center justify-between">
  <h1 class="text-xl font-semibold text-[var(--ui-foreground)]">{title}</h1>
  {#if onrefresh}
    <Button variant="ghost" size="sm" onclick={onrefresh} disabled={busy} ariaLabel={$common.status.refresh}>
      <RefreshCw size={16} class={busy ? "animate-spin" : ""} />
    </Button>
  {/if}
</header>
