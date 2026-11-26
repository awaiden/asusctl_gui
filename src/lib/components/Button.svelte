<script lang="ts" module>
  import { tv, type VariantProps } from "tailwind-variants";

  export const buttonVariants = tv({
    base: "inline-flex cursor-pointer items-center justify-center gap-1.5 rounded-[var(--ui-radius)] border font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--ui-ring)] disabled:pointer-events-none disabled:opacity-50",
    variants: {
      variant: {
        primary: "border-transparent bg-[var(--ui-primary)] text-[var(--ui-primary-foreground)] hover:bg-[var(--ui-primary)]/90",
        secondary: "border-[var(--ui-border)] bg-[var(--ui-muted)] text-[var(--ui-foreground)] hover:bg-[var(--ui-background)]",
        ghost: "border-transparent bg-transparent text-[var(--ui-muted-foreground)] hover:bg-[var(--ui-muted)] hover:text-[var(--ui-foreground)]",
      },
      size: {
        sm: "h-7 px-2.5 text-xs",
        md: "h-8 px-3 text-sm",
        lg: "h-10 px-4 text-sm",
      },
    },
    defaultVariants: {
      variant: "primary",
      size: "md",
    },
  });

  export type ButtonVariants = VariantProps<typeof buttonVariants>;
</script>

<script lang="ts">
  import type { Snippet } from "svelte";
  import { Button as UiButton } from "@ui-sdk/svelte/button";

  interface Props extends ButtonVariants {
    disabled?: boolean;
    type?: "button" | "submit" | "reset";
    ariaLabel?: string;
    class?: string;
    onclick?: (event: MouseEvent) => void | Promise<unknown>;
    children?: Snippet;
  }

  let {
    variant = "primary",
    size = "md",
    disabled = false,
    type = "button",
    ariaLabel,
    class: className = "",
    onclick,
    children,
  }: Props = $props();
</script>

<UiButton
  {type}
  {disabled}
  aria-label={ariaLabel}
  data-variant={variant}
  data-size={size}
  class={buttonVariants({ variant, size, class: className })}
  {onclick}
>
  {@render children?.()}
</UiButton>
