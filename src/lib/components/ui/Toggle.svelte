<script lang="ts" module>
  import { tv } from "tailwind-variants";

  export const toggleVariants = tv({
    slots: {
      root: "flex cursor-pointer items-center justify-between gap-3 text-sm",
    },
    variants: {
      disabled: {
        true: {
          root: "cursor-not-allowed opacity-60",
        },
      },
    },
  });
</script>

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
  const styles = $derived(toggleVariants({ disabled }));
</script>

<label data-slot="label" class={styles.root()}>
  <span>{label}</span>
  <Switch
    bind:checked
    {disabled}
    onCheckedChange={(v) => onchange?.(v)}
  >
    <SwitchThumb />
  </Switch>
</label>
