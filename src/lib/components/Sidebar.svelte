<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { useDictionary } from "dialexjs/svelte";
  import { DEFAULT_LOCALE, isLocale, LOCALE_OPTIONS, localizedPath, type Locale } from "$lib/i18n/locales";
  import { app, emptySupported } from "$lib/state.svelte";
  import { setTheme, theme, type ThemeMode } from "$lib/theme.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import { Battery, CircleHelp, Fan, Globe, Info, Keyboard, Monitor, Moon, Sun, Wrench, Zap } from "@lucide/svelte";
  import { tv } from "tailwind-variants";

  const common = useDictionary("common");
  const currentLocale = $derived<Locale>(isLocale(page.params.locale) ? page.params.locale : DEFAULT_LOCALE);

  const navItem = tv({
    base: "flex items-center gap-2 rounded-[var(--ui-radius)] border px-2.5 py-1.5 text-sm no-underline transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--ui-ring)]",
    variants: {
      state: {
        active: "border-[var(--ui-border)] bg-[var(--ui-background)] font-medium text-[var(--ui-foreground)]",
        idle: "border-transparent text-[var(--ui-muted-foreground)] hover:bg-[var(--ui-background)] hover:text-[var(--ui-foreground)]",
        disabled: "cursor-not-allowed border-transparent text-[var(--ui-muted-foreground)]/65",
      },
    },
    defaultVariants: {
      state: "idle",
    },
  });

  const themeBtn = tv({
    base: "flex cursor-pointer items-center justify-center py-1.5 transition-colors focus-visible:relative focus-visible:z-10 focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-[var(--ui-ring)]",
    variants: {
      active: {
        true: "bg-[var(--ui-primary)] text-[var(--ui-primary-foreground)]",
        false: "text-[var(--ui-muted-foreground)] hover:text-[var(--ui-foreground)]",
      },
    },
  });

  const sup = $derived(app.info?.supported ?? emptySupported);
  const items = $derived([
    { href: `/${currentLocale}`, label: $common.nav.systemInfo, icon: Info, ok: true },
    { href: `/${currentLocale}/power`, label: $common.nav.powerProfiles, icon: Zap, ok: true },
    { href: `/${currentLocale}/keyboard`, label: $common.nav.keyboardAura, icon: Keyboard, ok: true },
    { href: `/${currentLocale}/battery`, label: $common.nav.battery, icon: Battery, ok: true },
    { href: `/${currentLocale}/fans`, label: $common.nav.fanCurves, icon: Fan, ok: sup.fan_curves },
    { href: `/${currentLocale}/armoury`, label: $common.nav.armoury, icon: Wrench, ok: sup.armoury },
    { href: `/${currentLocale}/extras`, label: $common.nav.extras, icon: Monitor, ok: sup.screenpad || sup.slash },
    { href: `/${currentLocale}/about`, label: $common.nav.about, icon: CircleHelp, ok: true },
  ]);

  function onLocaleChange(next: string) {
    if (isLocale(next)) {
      goto(localizedPath(next, page.url.pathname));
    }
  }

  const deviceMeta = $derived.by(() => {
    const raw = app.info?.raw ?? "";
    const family = raw.match(/Product family:\s*(.+)/i)?.[1]?.trim();
    const board = raw.match(/Board name:\s*(.+)/i)?.[1]?.trim();
    return { family, board };
  });
</script>

<nav aria-label={$common.nav.primaryAria} class="flex w-56 shrink-0 flex-col border-r border-[var(--ui-border)] bg-[var(--ui-muted)] p-3">
  <div class="mb-3 flex items-center gap-2 px-2 text-lg font-semibold text-[var(--ui-foreground)]">
    <img src="/icon.svg" alt="" class="h-6 w-6 shrink-0" />
    <span>ASUS Control</span>
  </div>
  <div class="flex flex-1 flex-col gap-1">
    {#each items as it (it.href)}
      {#if it.ok}
        <a
          href={it.href}
          aria-current={page.url.pathname === it.href ? "page" : undefined}
          class={navItem({ state: page.url.pathname === it.href ? "active" : "idle" })}
        >
          <it.icon size={16} class="shrink-0" /><span class="truncate">{it.label}</span>
        </a>
      {:else}
        <span
          class={navItem({ state: "disabled" })}
          title={$common.status.unsupported}
        >
          <it.icon size={16} class="shrink-0" /><span class="truncate">{it.label}</span>
        </span>
      {/if}
    {/each}
  </div>
  <div class="mt-3 space-y-2 border-t border-[var(--ui-border)] px-2 pt-3">
    <div
      role="group"
      aria-label={$common.theme.label}
      class="grid grid-cols-3 divide-x divide-[var(--ui-border)] overflow-hidden rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)]"
    >
      {#each [
        { mode: "system" as ThemeMode, label: $common.theme.system, icon: Monitor },
        { mode: "light" as ThemeMode, label: $common.theme.light, icon: Sun },
        { mode: "dark" as ThemeMode, label: $common.theme.dark, icon: Moon },
      ] as opt (opt.mode)}
        <button
          type="button"
          title={opt.label}
          aria-label={opt.label}
          aria-pressed={theme.mode === opt.mode}
          onclick={() => setTheme(opt.mode)}
          class={themeBtn({ active: theme.mode === opt.mode })}
        >
          <opt.icon size={14} />
        </button>
      {/each}
    </div>
    <div class="flex items-center gap-2 text-xs text-[var(--ui-muted-foreground)]">
      <Globe size={14} class="shrink-0" />
      <div class="min-w-0 flex-1">
        <Select
          ariaLabel={$common.nav.language}
          items={[...LOCALE_OPTIONS]}
          value={currentLocale}
          onchange={onLocaleChange}
        />
      </div>
    </div>
  </div>
  {#if deviceMeta.family || deviceMeta.board}
    <div class="mt-3 border-t border-[var(--ui-border)] px-2 pt-3 text-xs text-[var(--ui-muted-foreground)]">
      {#if deviceMeta.family}<div class="truncate font-medium text-[var(--ui-foreground)]">{deviceMeta.family}</div>{/if}
      {#if deviceMeta.board}<div class="truncate font-mono">{deviceMeta.board}</div>{/if}
    </div>
  {/if}
</nav>
