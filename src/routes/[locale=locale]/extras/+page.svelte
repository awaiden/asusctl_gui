<script lang="ts">
  import { onMount } from "svelte";
  import { useDictionary } from "dialexjs/svelte";
  import PageHeader from "$lib/components/ui/PageHeader.svelte";
  import Loading from "$lib/components/ui/Loading.svelte";
  import ErrorBox from "$lib/components/ui/ErrorBox.svelte";
  import Unsupported from "$lib/components/ui/Unsupported.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import Slider from "$lib/components/ui/Slider.svelte";
  import Toggle from "$lib/components/ui/Toggle.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { screenpad, slash, errMsg } from "$lib/api";
  import { app, loadInfo, toast } from "$lib/state.svelte";

  const pages = useDictionary("pages");
  const common = useDictionary("common");
  let loading = $state(true);
  let busy = $state(false);
  let spBright = $state(100);
  let spSync = $state(false);
  let slBright = $state(128);
  let slEnable = $state(true);

  async function load() {
    loading = true;
    await loadInfo();
    loading = false;
  }
  async function run(fn: () => Promise<string>, ok: string) {
    busy = true;
    try { await fn(); toast(ok); } catch (e) { toast(errMsg(e), "error"); }
    finally { busy = false; }
  }
  onMount(load);
</script>

<PageHeader title={$pages.extras.title} onrefresh={load} busy={loading} />
{#if loading && !app.info}
  <Loading />
{:else if app.error}
  <ErrorBox message={app.error} onretry={load} />
{:else if app.info && !app.info.supported.screenpad && !app.info.supported.slash}
  <Unsupported />
{:else if app.info}
  <div class="space-y-4">
    {#if app.info.supported.screenpad}
      <Card title={$pages.extras.screenpad}>
        <Slider label={$pages.extras.brightness} min={0} max={100} bind:value={spBright} />
        <div class="mt-3 flex items-center gap-3 text-sm">
          <Button variant="primary" size="sm" disabled={busy}
            onclick={() => run(() => screenpad({ brightness: spBright }), $pages.extras.toastScreenpadBrightness)}>{$common.actions.apply}</Button>
          <Toggle label={$pages.extras.syncWithMain} disabled={busy} bind:checked={spSync}
            onchange={(v) => run(() => screenpad({ sync: v }), v ? $pages.extras.toastScreenpadSyncOn : $pages.extras.toastScreenpadSyncOff)} />
        </div>
      </Card>
    {/if}
    {#if app.info.supported.slash}
      <Card title={$pages.extras.slash}>
        <Toggle label={$pages.extras.enabled} disabled={busy} bind:checked={slEnable}
          onchange={(v) => run(() => slash({ enable: v }), v ? $pages.extras.toastSlashEnabled : $pages.extras.toastSlashDisabled)} />
        <div class="mt-3"><Slider label={$pages.extras.brightness} min={0} max={255} bind:value={slBright} /></div>
        <Button variant="primary" size="sm" class="mt-3" disabled={busy}
          onclick={() => run(() => slash({ brightness: slBright }), $pages.extras.toastSlashBrightness)}>{$common.actions.apply}</Button>
      </Card>
    {/if}
  </div>
{/if}
