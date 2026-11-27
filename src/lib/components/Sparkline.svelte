<script lang="ts">
  interface Props {
    values: number[];
    max?: number;
    color?: string;
    height?: number;
    ariaLabel?: string;
  }

  let {
    values,
    max,
    color = "var(--ui-primary)",
    height = 56,
    ariaLabel = "Sparkline chart",
  }: Props = $props();

  const width = 240;

  const effectiveMax = $derived.by(() => {
    if (max !== undefined && max > 0) return max;
    const m = values.length > 0 ? Math.max(...values) : 0;
    return m > 0 ? m * 1.15 : 1;
  });

  const points = $derived.by(() => {
    if (values.length === 0) return "";
    if (values.length === 1) {
      const y = height - Math.min(1, Math.max(0, values[0] / effectiveMax)) * (height - 4) - 2;
      return `0,${y.toFixed(1)} ${width},${y.toFixed(1)}`;
    }
    const step = width / (values.length - 1);
    return values
      .map((v, i) => {
        const x = i * step;
        const pct = Math.min(1, Math.max(0, v / effectiveMax));
        const y = height - pct * (height - 4) - 2;
        return `${x.toFixed(1)},${y.toFixed(1)}`;
      })
      .join(" ");
  });

  const areaPoints = $derived(
    points ? `0,${height} ${points} ${width},${height}` : "",
  );
</script>

<svg
  viewBox={`0 0 ${width} ${height}`}
  preserveAspectRatio="none"
  class="block w-full overflow-visible"
  style={`height: ${height}px`}
  role="img"
  aria-label={ariaLabel}
>
  {#each [0.25, 0.5, 0.75] as g (g)}
    <line
      x1="0"
      y1={height * g}
      x2={width}
      y2={height * g}
      stroke="var(--ui-border)"
      stroke-width="0.75"
      stroke-dasharray="3 3"
      opacity="0.6"
    />
  {/each}
  {#if areaPoints}
    <polygon points={areaPoints} fill={color} opacity="0.14" />
  {/if}
  {#if points}
    <polyline
      {points}
      fill="none"
      stroke={color}
      stroke-width="1.75"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  {/if}
</svg>
