<script lang="ts">
  import { useDictionary } from "dialexjs/svelte";
  import PageHeader from "$lib/components/ui/PageHeader.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import { ExternalLink } from "@lucide/svelte";

  const pages = useDictionary("pages");
</script>

<PageHeader title={$pages.about.title} />
<Card title="ASUS Control">
  <dl class="grid grid-cols-1 gap-3 text-sm sm:grid-cols-2">
    <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2">
      <dt class="text-xs text-[var(--ui-muted-foreground)]">{$pages.about.version}</dt>
      <dd class="mt-0.5 font-mono font-medium text-[var(--ui-foreground)]">0.1.1</dd>
    </div>
    <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2">
      <dt class="text-xs text-[var(--ui-muted-foreground)]">{$pages.about.license}</dt>
      <dd class="mt-0.5 font-medium text-[var(--ui-foreground)]">MIT</dd>
    </div>
    <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2 sm:col-span-2">
      <dt class="text-xs text-[var(--ui-muted-foreground)]">{$pages.about.repository}</dt>
      <dd class="mt-0.5">
        <a
          href="https://github.com/awaiden/asusctl_gui"
          target="_blank"
          rel="noopener noreferrer"
          class="inline-flex items-center gap-1.5 rounded-sm font-medium text-[var(--ui-primary)] hover:underline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--ui-ring)]"
        >
          <span>github.com/awaiden/asusctl_gui</span>
          <ExternalLink size={14} />
        </a>
      </dd>
    </div>
  </dl>
  <p class="mt-4 text-xs text-[var(--ui-muted-foreground)]">
    {$pages.about.description}
  </p>
</Card>
