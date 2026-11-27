<script lang="ts">
  import { onMount } from "svelte";
  import { useDictionary } from "dialexjs/svelte";
  import PageHeader from "$lib/components/ui/PageHeader.svelte";
  import Loading from "$lib/components/ui/Loading.svelte";
  import ErrorBox from "$lib/components/ui/ErrorBox.svelte";
  import Unsupported from "$lib/components/ui/Unsupported.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import RawOutput from "$lib/components/ui/RawOutput.svelte";
  import Slider from "$lib/components/ui/Slider.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { batteryInfo, batterySetLimit, batteryOneShot, isUnsupported, errMsg, type BatteryOut } from "$lib/api";
  import { toast } from "$lib/state.svelte";

  const pages = useDictionary("pages");
  const common = useDictionary("common");
  let loading = $state(true);
  let busy = $state(false);
  let error = $state<string | null>(null);
  let unsupported = $state(false);
  let info = $state<BatteryOut | null>(null);
  let limit = $state(80);
  let customMode = $state(false);
  const presets = [60, 80, 100];

  async function load() {
    loading = true;
    try {
      info = await batteryInfo();
      if (info.limit !== null) {
        limit = Math.min(100, Math.max(20, info.limit));
        customMode = !presets.includes(limit);
      }
      error = null; unsupported = false;
    } catch (e) {
      unsupported = isUnsupported(e); error = unsupported ? null : errMsg(e);
    } finally { loading = false; }
  }
  async function apply() {
    busy = true;
    try { await batterySetLimit(limit); toast($pages.battery.toastLimitSet(limit)); await load(); }
    catch (e) { toast(errMsg(e), "error"); }
    finally { busy = false; }
  }
  async function oneShot() {
    busy = true;
    try { await batteryOneShot(); toast($pages.battery.toastOneShotStarted); }
    catch (e) { toast(errMsg(e), "error"); }
    finally { busy = false; }
  }
  onMount(load);
</script>

<PageHeader title={$pages.battery.title} onrefresh={load} busy={loading} />
{#if loading && !info}
  <Loading />
{:else if unsupported}
  <Unsupported />
{:else if error}
  <ErrorBox message={error} onretry={load} />
{:else if info}
  <div class="space-y-4">
    <Card title={$pages.battery.chargeLimit}>
      <div class="space-y-3">
        {#if info.limit !== null}
          <p class="text-sm text-[var(--ui-muted-foreground)]">
            {$pages.battery.currentLimit} <span class="font-medium text-[var(--ui-foreground)] tabular-nums">{info.limit}%</span>
          </p>
        {/if}
        {#if customMode}
          <Slider label={$pages.battery.targetLimit} min={20} max={100} bind:value={limit} />
        {/if}
        <div class="flex flex-wrap items-center gap-2 text-sm">
          <div class="inline-flex divide-x divide-[var(--ui-border)] rounded-[var(--ui-radius)] border border-[var(--ui-border)]">
            {#each presets as p (p)}
              <Button
                variant={!customMode && limit === p ? "primary" : "secondary"}
                size="sm"
                grouped
                ariaPressed={!customMode && limit === p}
                disabled={busy}
                onclick={() => { customMode = false; limit = p; }}
              >{p}%</Button>
            {/each}
            <Button
              variant={customMode ? "primary" : "secondary"}
              size="sm"
              grouped
              ariaPressed={customMode}
              disabled={busy}
              onclick={() => { customMode = true; }}
            >{$pages.battery.custom}</Button>
          </div>
          <div class="ml-auto flex gap-2">
            <Button variant="primary" size="sm" disabled={busy || info.limit === limit} onclick={apply}>{$common.actions.apply}</Button>
            <Button variant="secondary" size="sm" disabled={busy} onclick={oneShot}>{$pages.battery.chargeToFullOnce}</Button>
          </div>
        </div>
      </div>
    </Card>
    <RawOutput text={info.raw} />
  </div>
{/if}
