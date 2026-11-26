<script lang="ts">
  import { onMount } from "svelte";
  import { useDictionary } from "dialexjs/svelte";
  import PageHeader from "$lib/components/PageHeader.svelte";
  import Loading from "$lib/components/Loading.svelte";
  import ErrorBox from "$lib/components/ErrorBox.svelte";
  import Card from "$lib/components/Card.svelte";
  import RawOutput from "$lib/components/RawOutput.svelte";
  import Speedometer from "$lib/components/Speedometer.svelte";
  import { getThermals, type ThermalMetrics } from "$lib/api";
  import { app, loadInfo } from "$lib/state.svelte";

  const pages = useDictionary("pages");
  const common = useDictionary("common");

  let thermals = $state<ThermalMetrics>({
    cpu_temp_c: null,
    gpu_temp_c: null,
    cpu_fan_rpm: null,
    gpu_fan_rpm: null,
  });

  async function refreshAll() {
    await Promise.all([loadInfo(), getThermals().then((t) => (thermals = t)).catch(() => {})]);
  }

  const deviceInfo = $derived.by(() => {
    const raw = app.info?.raw ?? "";
    const version = raw.match(/Software version:\s*(.+)/i)?.[1]?.trim() ?? raw.match(/asusctl\s+v?([\d.]+)/i)?.[1]?.trim();
    const family = raw.match(/Product family:\s*(.+)/i)?.[1]?.trim();
    const board = raw.match(/Board name:\s*(.+)/i)?.[1]?.trim();
    return { version, family, board };
  });

  onMount(() => {
    if (!app.info && !app.loading) loadInfo();
    getThermals().then((t) => (thermals = t)).catch(() => {});
    const id = setInterval(() => {
      getThermals().then((t) => (thermals = t)).catch(() => {});
    }, 2500);
    return () => clearInterval(id);
  });
</script>

<PageHeader title={$pages.system.title} onrefresh={refreshAll} busy={app.loading} />
{#if app.loading && !app.info}
  <Loading />
{:else if app.error}
  <ErrorBox message={app.error} />
{:else if app.info}
  <Card title={$pages.system.deviceSoftware}>
    <dl class="grid grid-cols-1 gap-3 text-sm sm:grid-cols-3">
      <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2">
        <dt class="text-xs text-[var(--ui-muted-foreground)]">{$pages.system.productFamily}</dt>
        <dd class="mt-0.5 font-medium text-[var(--ui-foreground)]">{deviceInfo.family ?? "—"}</dd>
      </div>
      <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2">
        <dt class="text-xs text-[var(--ui-muted-foreground)]">{$pages.system.boardName}</dt>
        <dd class="mt-0.5 font-mono font-medium text-[var(--ui-foreground)]">{deviceInfo.board ?? "—"}</dd>
      </div>
      <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2">
        <dt class="text-xs text-[var(--ui-muted-foreground)]">{$pages.system.asusctlVersion}</dt>
        <dd class="mt-0.5 font-mono font-medium text-[var(--ui-foreground)] tabular-nums">{deviceInfo.version ?? "—"}</dd>
      </div>
    </dl>
  </Card>
  <Card title={$pages.system.thermalsFans}>
    <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
      <Speedometer label="CPU" temp={thermals.cpu_temp_c} maxTemp={105} rpm={thermals.cpu_fan_rpm} maxRpm={6000} />
      <Speedometer label="GPU" temp={thermals.gpu_temp_c} maxTemp={100} rpm={thermals.gpu_fan_rpm} maxRpm={6000} />
    </div>
  </Card>
  <Card title={$pages.system.supportedFeatures}>
    <dl class="grid grid-cols-1 gap-x-6 gap-y-3 text-sm sm:grid-cols-2">
      <div class="sm:col-span-2">
        <dt class="text-xs font-medium text-[var(--ui-muted-foreground)]">{$pages.system.interfaces}</dt>
        <dd class="mt-0.5 font-mono text-xs text-[var(--ui-foreground)]">
          {app.info.supported.interfaces.join(", ") || $common.status.none}
        </dd>
      </div>
      <div class="sm:col-span-2">
        <dt class="text-xs font-medium text-[var(--ui-muted-foreground)]">{$pages.system.auraModes}</dt>
        <dd class="mt-0.5 text-[var(--ui-foreground)]">
          {app.info.supported.aura_modes.join(", ") || $common.status.none}
        </dd>
      </div>
      {#each [
        { label: $pages.system.fanCurves, ok: app.info.supported.fan_curves },
        { label: $pages.system.slashLedBar, ok: app.info.supported.slash },
        { label: $pages.system.screenpad, ok: app.info.supported.screenpad },
        { label: $pages.system.armouryAttributes, ok: app.info.supported.armoury },
      ] as feat (feat.label)}
        <div class="flex items-center justify-between rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2">
          <dt class="text-[var(--ui-foreground)]">{feat.label}</dt>
          <dd class={feat.ok ? "text-xs font-medium text-emerald-600 dark:text-emerald-400" : "text-xs text-[var(--ui-muted-foreground)]"}>
            {feat.ok ? $common.status.supported : $common.status.unavailable}
          </dd>
        </div>
      {/each}
    </dl>
  </Card>
  <div class="space-y-3">
    <RawOutput text={app.info.raw} label="asusctl info" defaultOpen={true} />
    {#if app.info.supported_raw}
      <RawOutput text={app.info.supported_raw} label="asusctl info --show-supported" />
    {/if}
  </div>
{/if}
