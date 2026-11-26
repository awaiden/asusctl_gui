<script lang="ts">
  import { onMount } from "svelte";
  import { useDictionary } from "dialexjs/svelte";
  import PageHeader from "$lib/components/PageHeader.svelte";
  import Loading from "$lib/components/Loading.svelte";
  import ErrorBox from "$lib/components/ErrorBox.svelte";
  import Unsupported from "$lib/components/Unsupported.svelte";
  import Card from "$lib/components/Card.svelte";
  import RawOutput from "$lib/components/RawOutput.svelte";
  import Select from "$lib/components/Select.svelte";
  import { armouryList, armourySet, isUnsupported, errMsg, type ArmouryList } from "$lib/api";
  import { toast } from "$lib/state.svelte";

  const pages = useDictionary("pages");
  let loading = $state(true);
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
    try { await armourySet(name, value); toast(`${name} = ${value}`); await load(); }
    catch (e) { toast(errMsg(e), "error"); await load(); }
  }
  onMount(load);
</script>

<PageHeader title={$pages.armoury.title} onrefresh={load} busy={loading} />
{#if loading && !data}
  <Loading />
{:else if unsupported || (data && data.items.length === 0 && !data.raw.trim())}
  <Unsupported />
{:else if error}
  <ErrorBox message={error} />
{:else if data}
  {#each data.items as item (item.name)}
    <Card title={item.name.replace(/_/g, " ")}>
      {#if item.values.length}
        <Select
          ariaLabel={item.name}
          items={item.values.map((v) => ({ value: v, label: v }))}
          value={item.current ?? ""}
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
{/if}
