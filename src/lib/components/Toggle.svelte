<script lang="ts">
  import { Switch, SwitchThumb } from "@ui-sdk/svelte/switch";

  interface Props {
    checked?: boolean;
    label: string;
    disabled?: boolean;
    onchange?: (v: boolean) => void;
  }

  let {
    checked = $bindable(false),
    label,
    disabled = false,
    onchange,
  }: Props = $props();
</script>

<label data-slot="label" class="flex cursor-pointer items-center justify-between gap-3 text-sm">
  <span>{label}</span>
  <Switch
    bind:checked
    {disabled}
    onCheckedChange={(v) => onchange?.(v)}
  >
    <SwitchThumb />
  </Switch>
</label>
