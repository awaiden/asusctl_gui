<script lang="ts">
  interface Props {
    label: string;
    temp: number | null;
    maxTemp?: number;
    rpm: number | null;
    maxRpm?: number;
  }

  let { label, temp, maxTemp = 105, rpm, maxRpm = 6000 }: Props = $props();

  const tempPct = $derived(temp === null ? 0 : Math.min(1, Math.max(0, temp / maxTemp)));
  const rpmPct = $derived(rpm === null ? 0 : Math.min(1, Math.max(0, rpm / maxRpm)));

  const outerLen = 138.23; // pi * 44
  const innerLen = 100.53; // pi * 32
  const tempOffset = $derived(outerLen * (1 - tempPct));
  const rpmOffset = $derived(innerLen * (1 - rpmPct));

  const tempColor = $derived.by(() => {
    if (temp === null) return "var(--ui-border)";
    if (temp >= 90) return "var(--ui-destructive)";
    if (temp >= 78) return "#f59e0b";
    return "var(--ui-primary)";
  });

  function tickCoords(pct: number, r1: number, r2: number) {
    const angle = Math.PI * (1 - pct);
    const cx = 55;
    const cy = 54;
    return {
      x1: cx + r1 * Math.cos(angle),
      y1: cy - r1 * Math.sin(angle),
      x2: cx + r2 * Math.cos(angle),
      y2: cy - r2 * Math.sin(angle),
    };
  }

  const ticks = Array.from({ length: 11 }, (_, i) => {
    const pct = i / 10;
    const color = pct >= 0.9 ? "var(--ui-destructive)" : pct >= 0.75 ? "#f59e0b" : "var(--ui-border)";
    return { pct, color, ...tickCoords(pct, 49, 53) };
  });
</script>

<div class="flex flex-col items-center rounded-[var(--ui-radius)] border border-[var(--ui-border)] bg-[var(--ui-background)] p-4">
  <svg viewBox="0 0 110 64" class="w-40 overflow-visible" aria-hidden="true">
    <!-- Outer radial scale ticks -->
    {#each ticks as t (t.pct)}
      <line
        x1={t.x1}
        y1={t.y1}
        x2={t.x2}
        y2={t.y2}
        stroke={t.color}
        stroke-width={t.pct >= 0.75 ? "1.8" : "1.2"}
        stroke-linecap="round"
      />
    {/each}
    <!-- Outer arc: Temperature -->
    <path
      d="M 11 54 A 44 44 0 0 1 99 54"
      fill="none"
      stroke="var(--ui-border)"
      stroke-width="7"
      stroke-linecap="round"
    />
    <path
      d="M 11 54 A 44 44 0 0 1 99 54"
      fill="none"
      stroke={tempColor}
      stroke-width="7"
      stroke-linecap="round"
      stroke-dasharray={outerLen}
      stroke-dashoffset={tempOffset}
      class="transition-[stroke-dashoffset,stroke] duration-300"
    />
    <!-- Inner arc: Fan RPM -->
    <path
      d="M 23 54 A 32 32 0 0 1 87 54"
      fill="none"
      stroke="var(--ui-border)"
      stroke-width="5"
      stroke-linecap="round"
    />
    <path
      d="M 23 54 A 32 32 0 0 1 87 54"
      fill="none"
      stroke="#38bdf8"
      stroke-width="5"
      stroke-linecap="round"
      stroke-dasharray={innerLen}
      stroke-dashoffset={rpmOffset}
      class="transition-[stroke-dashoffset] duration-300"
    />
  </svg>
  <div class="mt-2 flex flex-col items-center gap-0.5 text-center tabular-nums">
    <span class="text-xl leading-tight font-semibold text-[var(--ui-foreground)]">
      {temp === null ? "—" : `${temp}°C`}
    </span>
    <span class="text-xs font-medium text-sky-600 dark:text-sky-400">
      {rpm === null ? "— RPM" : `${rpm} RPM`}
    </span>
  </div>
  <div class="mt-1.5 text-xs font-medium text-[var(--ui-muted-foreground)]">{label}</div>
</div>
