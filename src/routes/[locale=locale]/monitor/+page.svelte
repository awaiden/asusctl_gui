<script lang="ts">
  import { onMount } from "svelte";
  import { useDictionary } from "dialexjs/svelte";
  import PageHeader from "$lib/components/ui/PageHeader.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import Speedometer from "$lib/components/Speedometer.svelte";
  import Sparkline from "$lib/components/Sparkline.svelte";
  import { getSystemMetrics, type SystemMetrics } from "$lib/api";
  import { Pause, Play } from "@lucide/svelte";

  const pages = useDictionary("pages");

  const MAX_POINTS = 60;

  let metrics = $state<SystemMetrics | null>(null);
  let live = $state(true);
  let intervalMs = $state(1000);
  let inFlight = false;

  let cpuHistory = $state<number[]>([]);
  let memHistory = $state<number[]>([]);
  let netRxHistory = $state<number[]>([]);
  let netTxHistory = $state<number[]>([]);
  let diskReadHistory = $state<number[]>([]);
  let diskWriteHistory = $state<number[]>([]);

  function pushPoint(arr: number[], val: number): number[] {
    const next = [...arr, val];
    return next.length > MAX_POINTS ? next.slice(next.length - MAX_POINTS) : next;
  }

  async function poll() {
    if (inFlight) return;
    inFlight = true;
    try {
      const m = await getSystemMetrics();
      metrics = m;
      cpuHistory = pushPoint(cpuHistory, m.cpu_usage_pct);
      memHistory = pushPoint(memHistory, m.mem_pct);
      netRxHistory = pushPoint(netRxHistory, m.net_rx_bytes_per_sec);
      netTxHistory = pushPoint(netTxHistory, m.net_tx_bytes_per_sec);
      diskReadHistory = pushPoint(diskReadHistory, m.disk_read_bytes_per_sec);
      diskWriteHistory = pushPoint(diskWriteHistory, m.disk_write_bytes_per_sec);
    } catch {
      // keep last known metrics
    } finally {
      inFlight = false;
    }
  }

  onMount(() => {
    poll();
  });

  $effect(() => {
    if (!live) return;
    const id = setInterval(poll, intervalMs);
    return () => clearInterval(id);
  });

  function formatGiB(bytes: number): string {
    return (bytes / (1024 * 1024 * 1024)).toFixed(1);
  }

  function formatRate(bytesPerSec: number): string {
    if (bytesPerSec >= 1024 * 1024) {
      return `${(bytesPerSec / (1024 * 1024)).toFixed(1)} MB/s`;
    }
    if (bytesPerSec >= 1024) {
      return `${(bytesPerSec / 1024).toFixed(1)} KB/s`;
    }
    return `${Math.round(bytesPerSec)} B/s`;
  }

  function formatUptime(secs: number): string {
    const d = Math.floor(secs / 86400);
    const h = Math.floor((secs % 86400) / 3600);
    const m = Math.floor((secs % 3600) / 60);
    const s = secs % 60;
    const parts: string[] = [];
    if (d > 0) parts.push(`${d}d`);
    if (h > 0 || d > 0) parts.push(`${h}h`);
    parts.push(`${m}m`);
    parts.push(`${s}s`);
    return parts.join(" ");
  }

  function barColor(pct: number): string {
    if (pct >= 85) return "bg-[var(--ui-destructive)]";
    if (pct >= 65) return "bg-[var(--ui-warning)]";
    return "bg-[var(--ui-primary)]";
  }

  const cpuPeak = $derived(cpuHistory.length > 0 ? Math.max(...cpuHistory) : 0);
</script>

<div class="mb-4 flex flex-wrap items-center justify-between gap-3">
  <PageHeader title={$pages.monitor.title} />
  <div class="inline-flex h-8 items-stretch overflow-hidden rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)]">
    <span class="inline-flex items-center gap-1.5 border-r border-[var(--ui-border)] px-2.5 text-xs font-medium text-[var(--ui-foreground)]">
      <span class={`h-2 w-2 rounded-full ${live ? "animate-pulse bg-emerald-500" : "bg-[var(--ui-muted-foreground)]"}`}></span>
      {live ? $pages.monitor.live : $pages.monitor.paused}
    </span>
    <div class="inline-flex items-stretch border-r border-[var(--ui-border)]">
      {#each [1000, 2000, 5000] as ms (ms)}
        <button
          type="button"
          onclick={() => (intervalMs = ms)}
          class={`inline-flex cursor-pointer items-center px-2 font-mono text-xs transition-colors ${intervalMs === ms ? "bg-[var(--ui-primary)] text-[var(--ui-primary-foreground)]" : "text-[var(--ui-muted-foreground)] hover:text-[var(--ui-foreground)]"}`}
        >
          {ms / 1000}s
        </button>
      {/each}
    </div>
    <button
      type="button"
      onclick={() => { live = !live; }}
      class="inline-flex cursor-pointer items-center gap-1.5 bg-[var(--ui-muted)] px-3 text-xs font-medium text-[var(--ui-foreground)] transition-colors hover:bg-[var(--ui-background)]"
    >
      {#if live}
        <Pause size={14} />
        <span>{$pages.monitor.pause}</span>
      {:else}
        <Play size={14} />
        <span>{$pages.monitor.resume}</span>
      {/if}
    </button>
  </div>
</div>

{#if metrics}
  <div class="space-y-4">
    <!-- CPU & Memory -->
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      <Card title={$pages.monitor.cpu}>
        <div class="space-y-3">
          <div class="flex items-baseline justify-between">
            <span class="text-3xl font-semibold tabular-nums text-[var(--ui-foreground)]">
              {metrics.cpu_usage_pct.toFixed(1)}%
            </span>
            <span class="text-xs font-mono tabular-nums text-[var(--ui-muted-foreground)]">
              {$pages.monitor.peak}: {cpuPeak.toFixed(1)}%
            </span>
          </div>
          <Sparkline values={cpuHistory} max={100} color="var(--ui-primary)" ariaLabel={$pages.monitor.cpu} />
          <div class="grid grid-cols-2 gap-2 text-xs">
            <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-2.5 py-1.5">
              <div class="text-[var(--ui-muted-foreground)]">{$pages.monitor.frequency}</div>
              <div class="mt-0.5 font-mono font-medium tabular-nums text-[var(--ui-foreground)]">
                {metrics.cpu_freq_mhz !== null ? `${(metrics.cpu_freq_mhz / 1000).toFixed(2)} GHz` : "—"}
              </div>
            </div>
            <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-2.5 py-1.5">
              <div class="text-[var(--ui-muted-foreground)]">{$pages.monitor.loadAvg}</div>
              <div class="mt-0.5 font-mono font-medium tabular-nums text-[var(--ui-foreground)]">
                {metrics.load_avg.map((v) => v.toFixed(2)).join(" · ")}
              </div>
            </div>
          </div>
        </div>
      </Card>

      <Card title={$pages.monitor.memory}>
        <div class="space-y-3">
          <div class="flex items-baseline justify-between">
            <span class="text-3xl font-semibold tabular-nums text-[var(--ui-foreground)]">
              {metrics.mem_pct.toFixed(1)}%
            </span>
            <span class="text-xs font-mono tabular-nums text-[var(--ui-muted-foreground)]">
              {formatGiB(metrics.mem_used_bytes)} / {formatGiB(metrics.mem_total_bytes)} GiB
            </span>
          </div>
          <Sparkline values={memHistory} max={100} color="var(--ui-fan)" ariaLabel={$pages.monitor.memory} />
          <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-2.5 py-1.5 text-xs">
            <div class="flex items-center justify-between">
              <span class="text-[var(--ui-muted-foreground)]">{$pages.monitor.swap}</span>
              <span class="font-mono tabular-nums text-[var(--ui-foreground)]">
                {formatGiB(metrics.swap_used_bytes)} / {formatGiB(metrics.swap_total_bytes)} GiB ({metrics.swap_pct.toFixed(0)}%)
              </span>
            </div>
            <div class="mt-1.5 h-1.5 w-full overflow-hidden rounded-full bg-[var(--ui-muted)]">
              <div
                class="h-full rounded-full bg-[var(--ui-fan)] transition-[width] duration-300"
                style={`width: ${Math.min(100, Math.max(0, metrics.swap_pct))}%`}
              ></div>
            </div>
          </div>
        </div>
      </Card>
    </div>

    <!-- Per-Core CPU Usage -->
    {#if metrics.cpu_per_core_pct.length > 0}
      <Card title={$pages.monitor.perCoreUsage}>
        <div class="grid grid-cols-2 gap-2.5 sm:grid-cols-4">
          {#each metrics.cpu_per_core_pct as pct, idx (idx)}
            <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-2.5 py-2">
              <div class="flex items-center justify-between text-xs">
                <span class="text-[var(--ui-muted-foreground)]">{$pages.monitor.core(idx)}</span>
                <span class="font-mono font-medium tabular-nums text-[var(--ui-foreground)]">{pct.toFixed(0)}%</span>
              </div>
              <div class="mt-1.5 h-1.5 w-full overflow-hidden rounded-full bg-[var(--ui-muted)]">
                <div
                  class={`h-full rounded-full transition-[width] duration-300 ${barColor(pct)}`}
                  style={`width: ${Math.min(100, Math.max(0, pct))}%`}
                ></div>
              </div>
            </div>
          {/each}
        </div>
      </Card>
    {/if}

    <!-- Thermals & Fans -->
    <Card title={$pages.monitor.thermalsFans}>
      <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
        <Speedometer label="CPU" temp={metrics.cpu_temp_c} maxTemp={105} rpm={metrics.cpu_fan_rpm} maxRpm={6000} />
        <Speedometer label="GPU" temp={metrics.gpu_temp_c} maxTemp={100} rpm={metrics.gpu_fan_rpm} maxRpm={6000} />
      </div>
      {#if metrics.nvme_temp_c !== null}
        <div class="mt-3 flex items-center justify-between rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2 text-xs">
          <span class="text-[var(--ui-muted-foreground)]">{$pages.monitor.nvme}</span>
          <span class="font-mono font-medium tabular-nums text-[var(--ui-foreground)]">{metrics.nvme_temp_c}°C</span>
        </div>
      {/if}
    </Card>

    <!-- Network, Disk I/O & Power -->
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      <Card title={`${$pages.monitor.networkIo} & ${$pages.monitor.diskIo}`}>
        <div class="space-y-3 text-xs">
          <div>
            <div class="mb-1 flex items-center justify-between">
              <span class="font-medium text-[var(--ui-muted-foreground)]">{$pages.monitor.networkIo}</span>
              <span class="font-mono tabular-nums text-[var(--ui-foreground)]">
                ↓ {formatRate(metrics.net_rx_bytes_per_sec)} · ↑ {formatRate(metrics.net_tx_bytes_per_sec)}
              </span>
            </div>
            <Sparkline values={netRxHistory} color="var(--ui-primary)" height={44} ariaLabel={$pages.monitor.networkIo} />
          </div>
          <div>
            <div class="mb-1 flex items-center justify-between">
              <span class="font-medium text-[var(--ui-muted-foreground)]">{$pages.monitor.diskIo}</span>
              <span class="font-mono tabular-nums text-[var(--ui-foreground)]">
                R {formatRate(metrics.disk_read_bytes_per_sec)} · W {formatRate(metrics.disk_write_bytes_per_sec)}
              </span>
            </div>
            <Sparkline values={diskReadHistory} color="var(--ui-fan)" height={44} ariaLabel={$pages.monitor.diskIo} />
          </div>
        </div>
      </Card>

      <Card title={$pages.monitor.powerBattery}>
        <div class="space-y-3 text-xs">
          {#if metrics.battery_pct !== null}
            <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2">
              <div class="flex items-center justify-between">
                <span class="font-medium text-[var(--ui-foreground)]">
                  {metrics.battery_pct}% {metrics.battery_status ? `· ${metrics.battery_status}` : ""}
                </span>
                {#if metrics.ac_online !== null}
                  <span class={metrics.ac_online ? "font-medium text-emerald-600 dark:text-emerald-400" : "text-[var(--ui-muted-foreground)]"}>
                    {metrics.ac_online ? $pages.monitor.acOnline : $pages.monitor.onBattery}
                  </span>
                {/if}
              </div>
              <div class="mt-2 h-2 w-full overflow-hidden rounded-full bg-[var(--ui-muted)]">
                <div
                  class="h-full rounded-full bg-[var(--ui-primary)] transition-[width] duration-300"
                  style={`width: ${Math.min(100, Math.max(0, metrics.battery_pct))}%`}
                ></div>
              </div>
            </div>
          {/if}
          <div class="grid grid-cols-2 gap-2">
            {#if metrics.battery_power_w !== null}
              <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-2.5 py-1.5">
                <div class="text-[var(--ui-muted-foreground)]">{$pages.monitor.powerDraw}</div>
                <div class="mt-0.5 font-mono font-medium tabular-nums text-[var(--ui-foreground)]">
                  {metrics.battery_power_w.toFixed(1)} W
                </div>
              </div>
            {/if}
            <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-2.5 py-1.5">
              <div class="text-[var(--ui-muted-foreground)]">{$pages.monitor.uptime}</div>
              <div class="mt-0.5 font-mono font-medium tabular-nums text-[var(--ui-foreground)]">
                {formatUptime(metrics.uptime_secs)}
              </div>
            </div>
          </div>
        </div>
      </Card>
    </div>
  </div>
{/if}
