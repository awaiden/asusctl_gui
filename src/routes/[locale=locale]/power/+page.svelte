<script lang="ts">
  import { onMount } from "svelte";
  import { useDictionary } from "dialexjs/svelte";
  import PageHeader from "$lib/components/ui/PageHeader.svelte";
  import Loading from "$lib/components/ui/Loading.svelte";
  import ErrorBox from "$lib/components/ui/ErrorBox.svelte";
  import Unsupported from "$lib/components/ui/Unsupported.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import RawOutput from "$lib/components/ui/RawOutput.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { profileList, profileGet, profileSet, isUnsupported, errMsg, type Profile, type ProfileTarget, type ProfileGetOut } from "$lib/api";
  import { toast } from "$lib/state.svelte";

  const pages = useDictionary("pages");
  let loading = $state(true);
  let busy = $state(false);
  let error = $state<string | null>(null);
  let unsupported = $state(false);
  let profiles = $state<string[]>([]);
  let cur = $state<ProfileGetOut | null>(null);
  const valid = ["Quiet", "Balanced", "Performance"];

  function profileLabel(p: string) {
    return $pages.power.profiles[p as Profile] ?? p;
  }

  async function load() {
    loading = true;
    try {
      const [l, g] = await Promise.all([profileList(), profileGet()]);
      profiles = l.profiles.filter((p) => valid.includes(p));
      cur = g; error = null; unsupported = false;
    } catch (e) {
      unsupported = isUnsupported(e);
      error = unsupported ? null : errMsg(e);
    } finally { loading = false; }
  }
  async function set(p: string, t: ProfileTarget) {
    busy = true;
    try { await profileSet(p as Profile, t); toast($pages.power.toastSet(profileLabel(p), $pages.power.targets[t])); await load(); }
    catch (e) { toast(errMsg(e), "error"); }
    finally { busy = false; }
  }
  onMount(load);
</script>

<PageHeader title={$pages.power.title} onrefresh={load} busy={loading} />
{#if loading && !cur}
  <Loading />
{:else if unsupported}
  <Unsupported />
{:else if error}
  <ErrorBox message={error} onretry={load} />
{:else if cur}
  <div class="space-y-4">
    <Card title={$pages.power.currentState}>
      <dl class="grid grid-cols-1 gap-3 text-sm sm:grid-cols-3">
        <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2">
          <dt class="text-xs text-[var(--ui-muted-foreground)]">{$pages.power.active}</dt>
          <dd class="mt-0.5 font-medium text-[var(--ui-foreground)]">{cur.profile.active ? profileLabel(cur.profile.active) : "—"}</dd>
        </div>
        <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2">
          <dt class="text-xs text-[var(--ui-muted-foreground)]">{$pages.power.ac}</dt>
          <dd class="mt-0.5 font-medium text-[var(--ui-foreground)]">{cur.profile.ac ? profileLabel(cur.profile.ac) : "—"}</dd>
        </div>
        <div class="rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2">
          <dt class="text-xs text-[var(--ui-muted-foreground)]">{$pages.power.battery}</dt>
          <dd class="mt-0.5 font-medium text-[var(--ui-foreground)]">{cur.profile.battery ? profileLabel(cur.profile.battery) : "—"}</dd>
        </div>
      </dl>
    </Card>
    <Card title={$pages.power.setProfile}>
      <div class="space-y-2">
        {#each profiles as p (p)}
          <div class="flex flex-wrap items-center justify-between gap-3 rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] px-3 py-2 text-sm">
            <span class="font-medium text-[var(--ui-foreground)]">{profileLabel(p)}</span>
            <div class="inline-flex divide-x divide-[var(--ui-border)] rounded-[var(--ui-radius)] border border-[var(--ui-border)]">
              <Button variant={cur.profile.active === p ? "primary" : "secondary"} size="sm" grouped ariaPressed={cur.profile.active === p} disabled={busy} onclick={() => set(p, "both")}>{$pages.power.targets.both}</Button>
              <Button variant={cur.profile.ac === p ? "primary" : "secondary"} size="sm" grouped ariaPressed={cur.profile.ac === p} disabled={busy} onclick={() => set(p, "ac")}>{$pages.power.targets.ac}</Button>
              <Button variant={cur.profile.battery === p ? "primary" : "secondary"} size="sm" grouped ariaPressed={cur.profile.battery === p} disabled={busy} onclick={() => set(p, "battery")}>{$pages.power.targets.battery}</Button>
            </div>
          </div>
        {/each}
      </div>
    </Card>
    <RawOutput text={cur.raw} />
  </div>
{/if}
