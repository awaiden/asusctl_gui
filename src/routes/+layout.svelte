<script lang="ts">
  import "../app.css";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import ToastHost from "$lib/components/ToastHost.svelte";
  import { page } from "$app/state";
  import { provideDialex } from "dialexjs/svelte";
  import { dialex } from "$lib/dialex.generated";
  import { detectPreferredLocale, isLocale, persistLocale } from "$lib/i18n/locales";
  import { loadInfo } from "$lib/state.svelte";
  import { initTheme } from "$lib/theme.svelte";
  import { onMount } from "svelte";

  let { children } = $props();
  const initialLocale = isLocale(page.params.locale) ? page.params.locale : detectPreferredLocale();
  const dx = provideDialex({ ...dialex }, { initialLocale });

  $effect(() => {
    const param = page.params.locale;
    if (isLocale(param)) {
      dx.setLocale(param);
      persistLocale(param);
      if (typeof document !== "undefined") {
        document.documentElement.lang = param;
      }
    }
  });

  onMount(() => {
    loadInfo();
    return initTheme();
  });
</script>

<div class="flex h-screen">
  <Sidebar />
  <main class="flex-1 overflow-y-auto p-6">
    <div class="mx-auto max-w-4xl">
      {@render children()}
    </div>
  </main>
</div>
<ToastHost />
