<script lang="ts">
  import { onMount } from "svelte";
  import { useDictionary } from "dialexjs/svelte";
  import PageHeader from "$lib/components/ui/PageHeader.svelte";
  import Loading from "$lib/components/ui/Loading.svelte";
  import ErrorBox from "$lib/components/ui/ErrorBox.svelte";
  import Unsupported from "$lib/components/ui/Unsupported.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import RawOutput from "$lib/components/ui/RawOutput.svelte";
  import Toggle from "$lib/components/ui/Toggle.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import NumberInput from "$lib/components/ui/NumberInput.svelte";
  import { fanCurveGet, fanCurveSet, fanCurveEnable, fanCurveReset, isUnsupported, errMsg,
    type Profile, type Fan, type FanPoint } from "$lib/api";
  import { app, loadInfo, toast } from "$lib/state.svelte";

  const pages = useDictionary("pages");
  const common = useDictionary("common");

  interface CurvePoint extends FanPoint { id: number }

  const profiles: Profile[] = ["Quiet", "Balanced", "Performance"];
  const fans: Fan[] = ["cpu", "gpu", "mid"];
  const defaults = (): CurvePoint[] =>
    [30, 40, 50, 60, 70, 80, 90, 100].map((t, id) => ({ id, temp: t, pct: Math.max(10, t - 20) }));

  let loading = $state(true);
  let busy = $state(false);
  let error = $state<string | null>(null);
  let unsupported = $state(false);
  let raw = $state("");
  let profile = $state<Profile>("Balanced");
  let fan = $state<Fan>("cpu");
  let points = $state<CurvePoint[]>(defaults());
  let enabled = $state(true);

  async function load() {
    loading = true;
    try {
      if (!app.info) await loadInfo();
      if (app.info && !app.info.supported.fan_curves) { unsupported = true; return; }
      raw = await fanCurveGet(); error = null; unsupported = false;
    } catch (e) {
      unsupported = isUnsupported(e); error = unsupported ? null : errMsg(e);
    } finally { loading = false; }
  }
  const clamp = (n: number, lo: number, hi: number) => Math.min(hi, Math.max(lo, Math.round(Number(n) || 0)));
  async function act(fn: () => Promise<string>, ok: string, reload = true) {
    busy = true;
    try { await fn(); toast(ok); if (reload) raw = await fanCurveGet(); }
    catch (e) { toast(errMsg(e), "error"); }
    finally { busy = false; }
  }
  function save() {
    const pts = points.map((p) => ({ temp: clamp(p.temp, 0, 100), pct: clamp(p.pct, 0, 100) }));
    return act(() => fanCurveSet(profile, fan, pts), $pages.fans.toastSaved($pages.power.profiles[profile], fan));
  }
  onMount(load);
</script>

<PageHeader title={$pages.fans.title} onrefresh={load} busy={loading} />
{#if loading && !raw}
  <Loading />
{:else if unsupported}
  <Unsupported />
{:else if error}
  <ErrorBox message={error} onretry={load} />
{:else}
  <div class="space-y-4">
    <Card title={$pages.fans.editCurve}>
    <div class="mb-4 grid grid-cols-1 gap-3 text-sm sm:grid-cols-2">
      <Select label={$pages.fans.profile} items={profiles.map((p) => ({ value: p, label: $pages.power.profiles[p] }))} bind:value={profile} />
      <Select label={$pages.fans.fan} items={fans.map((f) => ({ value: f, label: f.toUpperCase() }))} bind:value={fan} />
    </div>
    <div class="mb-2 grid grid-cols-[1fr_1fr] gap-3 text-xs font-medium text-[var(--ui-muted-foreground)]">
      <span>{$pages.fans.temperature}</span>
      <span>{$pages.fans.fanSpeed}</span>
    </div>
    <div class="space-y-2">
      {#each points as p (p.id)}
        <div class="grid grid-cols-[1fr_1fr] gap-3 text-sm">
          <NumberInput min={0} max={100} bind:value={p.temp} ariaLabel={$pages.fans.tempPointAria(p.id + 1)} />
          <NumberInput min={0} max={100} bind:value={p.pct} ariaLabel={$pages.fans.fanPctPointAria(p.id + 1)} />
        </div>
      {/each}
    </div>
    <div class="mt-4 flex flex-wrap items-center gap-3 text-sm">
      <Button variant="primary" size="sm" disabled={busy} onclick={save}>{$pages.fans.saveCurve}</Button>
      <Toggle label={$pages.fans.enabled} disabled={busy} bind:checked={enabled}
        onchange={(v) => act(() => fanCurveEnable(profile, fan, v), v ? $pages.fans.toastEnabled($pages.power.profiles[profile], fan) : $pages.fans.toastDisabled($pages.power.profiles[profile], fan))} />
      <Button variant="secondary" size="sm" disabled={busy} onclick={() => act(fanCurveReset, $pages.fans.toastReset)}>{$common.actions.resetAll}</Button>
    </div>
  </Card>
  <RawOutput text={raw} />
  </div>
{/if}
