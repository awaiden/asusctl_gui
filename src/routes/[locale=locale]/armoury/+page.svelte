<script lang="ts">
  import { onMount } from "svelte";
  import { useDictionary } from "dialexjs/svelte";
  import PageHeader from "$lib/components/ui/PageHeader.svelte";
  import Loading from "$lib/components/ui/Loading.svelte";
  import ErrorBox from "$lib/components/ui/ErrorBox.svelte";
  import Unsupported from "$lib/components/ui/Unsupported.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import RawOutput from "$lib/components/ui/RawOutput.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import { armouryList, armourySet, isUnsupported, errMsg, type ArmouryList } from "$lib/api";
  import { toast } from "$lib/state.svelte";

  const pages = useDictionary("pages");
  let loading = $state(true);
  let busyName = $state<string | null>(null);
  let error = $state<string | null>(null);
  let unsupported = $state(false);
  let data = $state<ArmouryList | null>(null);

  async function load() {
    loading = true;
    try { data = await armouryList(); error = null; unsupported = false; }
    catch (e) { unsupported = isUnsupported(e); error = unsupported ? null : errMsg(e); }
    finally { loading = false; }
  }
  async function set(name: string, value: string) {
    busyName = name;
    try { await armourySet(name, value); toast(`${name} = ${value}`); await load(); }
    catch (e) { toast(errMsg(e), "error"); await load(); }
    finally { busyName = null; }
  }
  onMount(load);
</script>

<PageHeader title={$pages.armoury.title} onrefresh={load} busy={loading} />
{#if loading && !data}
  <Loading />
{:else if unsupported || (data && data.items.length === 0 && !data.raw.trim())}
  <Unsupported />
{:else if error}
  <ErrorBox message={error} onretry={load} />
{:else if data}
  <div class="space-y-4">
    {#each data.items as item (item.name)}
      <Card title={item.name.replace(/_/g, " ")}>
        {#if item.values.length}
          <Select
            ariaLabel={item.name}
            items={item.values.map((v) => ({ value: v, label: v }))}
            value={item.current ?? ""}
            disabled={busyName === item.name}
            onchange={(v) => set(item.name, v)}
          />
        {:else}
          <p class="text-sm text-[var(--ui-muted-foreground)]">{$pages.armoury.current} <span class="font-mono text-[var(--ui-foreground)]">{item.current ?? "?"}</span></p>
        {/if}
      </Card>
    {/each}
    {#if data.raw.trim()}
      <RawOutput text={data.raw} label={$pages.armoury.rawOutput} defaultOpen={data.items.length === 0} />
    {/if}
  </div>
{/if}
