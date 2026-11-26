<script lang="ts">
  import { dismiss, toasts } from "$lib/state.svelte";
  import { X } from "@lucide/svelte";
  import { tv } from "tailwind-variants";
  import Button from "$lib/components/Button.svelte";

  const toastVariants = tv({
    base: "flex items-center gap-3 rounded-[var(--ui-radius)] border px-3 py-2 text-sm shadow",
    variants: {
      kind: {
        success: "border-emerald-600/40 bg-emerald-50 text-emerald-900 dark:border-emerald-700/50 dark:bg-emerald-950/90 dark:text-emerald-100",
        error: "border-red-600/40 bg-red-50 text-red-900 dark:border-[var(--ui-destructive)]/50 dark:bg-red-950/90 dark:text-red-100",
      },
    },
    defaultVariants: {
      kind: "success",
    },
  });
</script>

<div class="fixed right-4 bottom-4 z-50 flex max-w-sm flex-col gap-2">
  {#each toasts.list as t (t.id)}
    <div
      class={toastVariants({ kind: t.kind })}
      data-type={t.kind}
      role="status"
    >
      <span>{t.text}</span>
      <Button variant="ghost" size="sm" onclick={() => dismiss(t.id)} ariaLabel="Dismiss">
        <X size={14} />
      </Button>
    </div>
  {/each}
</div>
