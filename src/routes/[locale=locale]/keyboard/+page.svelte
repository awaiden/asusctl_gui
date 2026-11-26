<script lang="ts">
  import { onMount } from "svelte";
  import { useDictionary } from "dialexjs/svelte";
  import PageHeader from "$lib/components/PageHeader.svelte";
  import Loading from "$lib/components/Loading.svelte";
  import ErrorBox from "$lib/components/ErrorBox.svelte";
  import Unsupported from "$lib/components/Unsupported.svelte";
  import Card from "$lib/components/Card.svelte";
  import RawOutput from "$lib/components/RawOutput.svelte";
  import Toggle from "$lib/components/Toggle.svelte";
  import Button from "$lib/components/Button.svelte";
  import Select from "$lib/components/Select.svelte";
  import SegmentedControl from "$lib/components/SegmentedControl.svelte";
  import ColorPicker from "$lib/components/ColorPicker.svelte";
  import { ledsGet, ledsSet, ledsNext, ledsPrev, auraEffect, auraPower, isUnsupported, errMsg,
    type Brightness, type AuraEffect, type Speed, type Direction, type PowerZone, type LedsOut } from "$lib/api";
  import { app, loadInfo, toast } from "$lib/state.svelte";

  const pages = useDictionary("pages");
  const common = useDictionary("common");

  const effects: AuraEffect[] = ["static","breathe","rainbow-cycle","rainbow-wave","stars","rain","highlight","laser","ripple","pulse","comet","flash"];
  const speeds: Speed[] = ["low","med","high"];
  const dirs: Direction[] = ["up","down","left","right"];
  const levels: Brightness[] = ["off","low","med","high"];
  const zones: PowerZone[] = ["keyboard","logo","lightbar","lid","rear-glow","ally"];
  const noColourEffects = new Set<AuraEffect>(["rainbow-cycle", "rainbow-wave", "rain"]);
  const twoColourEffects = new Set<AuraEffect>(["breathe", "stars"]);

  let loading = $state(true);
  let error = $state<string | null>(null);
  let unsupported = $state(false);
  let leds = $state<LedsOut | null>(null);
  let effect = $state<AuraEffect>("static");
  let colour = $state("#6366f1");
  let colour2 = $state("#38bdf8");
  let speed = $state<Speed>("med");
  let direction = $state<Direction>("right");
  let zone = $state<PowerZone>("keyboard");
  let boot = $state(true), awake = $state(true), sleep = $state(true), shutdown = $state(true);

  // Only offer effects reported as supported (fall back to all if list unknown).
  const available = $derived.by(() => {
    const m = (app.info?.supported.aura_modes ?? []).map((s) => s.toLowerCase().replace(/[\s_]/g, "-"));
    return m.length ? effects.filter((e) => m.includes(e)) : effects;
  });
  const hex = (c: string) => c.replace("#", "").toUpperCase();
  const cap = (s: string) => s.charAt(0).toUpperCase() + s.slice(1);
  const hasZones = $derived((app.info?.supported.interfaces ?? []).length > 0 && (app.info?.supported.aura_modes.length ?? 0) > 0);
  const needsColour = $derived(!noColourEffects.has(effect));
  const needsColour2 = $derived(twoColourEffects.has(effect));
  const needsDirection = $derived(effect === "rainbow-wave");

  const previewStyle = $derived.by(() => {
    const b = leds?.brightness?.toLowerCase();
    const opacity = b === "off" ? 0.1 : b === "low" ? 0.45 : b === "med" ? 0.75 : 1;
    if (noColourEffects.has(effect)) {
      return `opacity: ${opacity}; background: linear-gradient(90deg, #ef4444, #f59e0b, #10b981, #38bdf8, #6366f1, #ec4899);`;
    }
    if (twoColourEffects.has(effect)) {
      return `opacity: ${opacity}; background: linear-gradient(90deg, ${colour}, ${colour2});`;
    }
    return `opacity: ${opacity}; background: ${colour};`;
  });

  async function load() {
    loading = true;
    try {
      if (!app.info) await loadInfo();
      leds = await ledsGet(); error = null; unsupported = false;
    } catch (e) {
      unsupported = isUnsupported(e); error = unsupported ? null : errMsg(e);
    } finally { loading = false; }
  }
  async function run(fn: () => Promise<string>, ok: string) {
    try { await fn(); toast(ok); leds = await ledsGet().catch(() => leds); }
    catch (e) { toast(errMsg(e), "error"); }
  }
  onMount(load);
</script>

<PageHeader title={$pages.keyboard.title} onrefresh={load} busy={loading} />
{#if loading && !leds}
  <Loading />
{:else if unsupported}
  <Unsupported />
{:else if error}
  <ErrorBox message={error} />
{:else}
  <div class="space-y-4">
    <!-- Backlight & Live Preview -->
    <Card title={$pages.keyboard.backlight}>
      <div class="mb-4 overflow-hidden rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] p-3">
        <div class="mb-2 flex items-center justify-between text-xs text-[var(--ui-muted-foreground)]">
          <span>{$pages.keyboard.preview} · <span class="font-medium text-[var(--ui-foreground)]">{effect}</span></span>
          <span>{$pages.keyboard.level}: <span class="font-medium text-[var(--ui-foreground)]">{leds?.brightness ? ($pages.keyboard.levels[leds.brightness.toLowerCase() as Brightness] ?? cap(leds.brightness)) : $common.status.unknown}</span></span>
        </div>
        <div class="h-2.5 w-full rounded-full transition-all duration-300" style={previewStyle}></div>
      </div>

      <div class="flex flex-wrap items-center justify-between gap-3 text-sm">
        <div class="inline-flex divide-x divide-[var(--ui-border)] overflow-hidden rounded-[var(--ui-radius)] border border-[var(--ui-border)]">
          {#each levels as b (b)}
            <Button
              variant={leds?.brightness?.toLowerCase() === b ? "primary" : "secondary"}
              size="sm"
              class="rounded-none border-0"
              onclick={() => run(() => ledsSet(b), $pages.keyboard.toastBrightness($pages.keyboard.levels[b]))}
            >{$pages.keyboard.levels[b]}</Button>
          {/each}
        </div>
        <div class="flex gap-2">
          <Button variant="secondary" size="sm" onclick={() => run(ledsPrev, $pages.keyboard.toastPrevBrightness)}>{$common.actions.prev}</Button>
          <Button variant="secondary" size="sm" onclick={() => run(ledsNext, $pages.keyboard.toastNextBrightness)}>{$common.actions.next}</Button>
        </div>
      </div>
    </Card>

    <!-- Aura Studio -->
    <Card title={$pages.keyboard.auraEffect}>
      {#if available.length === 0}
        <Unsupported />
      {:else}
        <div class="grid grid-cols-1 gap-6 text-sm md:grid-cols-2">
          <div class="space-y-4">
            <Select label={$pages.keyboard.mode} items={available.map((e) => ({ value: e, label: e }))} bind:value={effect} />
            <SegmentedControl label={$pages.keyboard.speed} items={speeds.map((s) => ({ value: s, label: $pages.keyboard.speeds[s] }))} bind:value={speed} />
            {#if needsDirection}
              <SegmentedControl label={$pages.keyboard.direction} items={dirs.map((d) => ({ value: d, label: $pages.keyboard.directions[d] }))} bind:value={direction} />
            {/if}
          </div>

          <div class="flex flex-col justify-between gap-4">
            {#if needsColour || needsColour2}
              <div class="flex flex-wrap gap-6">
                {#if needsColour}
                  <ColorPicker label={$pages.keyboard.primaryColour} bind:value={colour} />
                {/if}
                {#if needsColour2}
                  <ColorPicker label={$pages.keyboard.secondaryColour} bind:value={colour2} />
                {/if}
              </div>
            {:else}
              <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] p-4 text-xs text-[var(--ui-muted-foreground)]">
                {$pages.keyboard.spectrumNote}
              </div>
            {/if}
          </div>
        </div>

        <div class="mt-5 flex flex-wrap items-center justify-between gap-2 border-t border-[var(--ui-border)] pt-4">
          <Button variant="primary" size="sm"
            onclick={() => run(() => auraEffect({ effect, colour: hex(colour), colour2: hex(colour2), speed, direction }), $pages.keyboard.toastEffectApplied(effect))}>{$pages.keyboard.applyEffect}</Button>
          <div class="flex gap-2">
            <Button variant="secondary" size="sm" onclick={() => run(() => auraEffect({ switch: "prev" }), $pages.keyboard.toastPrevEffect)}>{$pages.keyboard.prevMode}</Button>
            <Button variant="secondary" size="sm" onclick={() => run(() => auraEffect({ switch: "next" }), $pages.keyboard.toastNextEffect)}>{$pages.keyboard.nextMode}</Button>
          </div>
        </div>
      {/if}
    </Card>

    {#if hasZones}
      <Card title={$pages.keyboard.powerZones}>
        <div class="space-y-3 text-sm">
          <Select label={$pages.keyboard.zone} items={zones.map((z) => ({ value: z, label: $pages.keyboard.zones[z] }))} bind:value={zone} />
          <div class="grid grid-cols-2 gap-3 sm:grid-cols-4">
            <Toggle label={$pages.keyboard.boot} bind:checked={boot} />
            <Toggle label={$pages.keyboard.awake} bind:checked={awake} />
            <Toggle label={$pages.keyboard.sleep} bind:checked={sleep} />
            <Toggle label={$pages.keyboard.shutdown} bind:checked={shutdown} />
          </div>
          <Button variant="primary" size="sm"
            onclick={() => run(() => auraPower(zone, boot, awake, sleep, shutdown), $pages.keyboard.toastZoneUpdated($pages.keyboard.zones[zone]))}>{$pages.keyboard.applyZone}</Button>
        </div>
      </Card>
    {/if}

    {#if leds}<RawOutput text={leds.raw} label={$pages.keyboard.ledsOutput} />{/if}
  </div>
{/if}
