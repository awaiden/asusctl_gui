# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Broader ASUS Linux / `asusctl` community: ASUS laptop owners running Linux with `asusd` who want a desktop GUI for hardware controls.

## Product Purpose

Full feature-parity visual desktop front-end for every `asusctl` CLI subcommand (System Info, Power Profiles, Keyboard & Aura, Battery, Fan Curves, Armoury firmware attributes, Screenpad backlight, and Slash LED bar) with raw CLI transparency so users can control hardware without memorizing CLI flags.

## Positioning

A lightweight Tauri 2 desktop GUI that drives the `asusctl` CLI directly (no D-Bus bindings), combines full subcommand parity with raw CLI output visibility, and degrades gracefully across heterogeneous ASUS hardware models (e.g. TUF vs. ROG laptops with different supported interfaces).

## Operating Context

Linux desktop running `asusd` with `asusctl` in `PATH`, packaged and run as a Tauri 2 desktop application (`bun run tauri dev` / `bun run tauri build`). Hardware capabilities vary significantly by laptop model (for example, ASUS TUF Gaming F15 FX506HF lacks `FanCurves`, `Slash`, `Screenpad`, and Aura zones).

## Capabilities and Constraints

- Always drive `/usr/bin/asusctl` directly via `std::process::Command` with typed, validated argument vectors — no direct D-Bus bindings, no shell execution, and no free-form argument passthrough.
- Detect supported interfaces via `asusctl info --show-supported` and gracefully hide or disable unsupported features instead of surfacing errors.
- Always provide a raw CLI output fallback whenever structured parsing fails.
- Keep all interactive controls keyboard-navigable and screen-reader accessible via `@ui-sdk/svelte` behaviors.
- Open-source project (`MIT` license).

## Brand Commitments

App name is **ASUS Control** (`asusctl_gui`).

## Evidence on Hand

Verified against real `asusctl v6.3.8` output on an ASUS TUF Gaming F15 (FX506HF). No external user testimonials, benchmarks, or marketing claims exist; future work must not fabricate them.

## Product Principles

- **CLI truth over guesswork**: Every action maps directly to a validated `asusctl` invocation, and raw CLI output stays inspectable when parsing falls short.
- **Hardware-aware restraint**: Capabilities not supported by the user's laptop are clearly marked unavailable rather than failing noisily.
- **Safe by construction**: Typed inputs, strict range/token validation, and zero shell interpolation protect the user's system.
- **Accessible desktop operation**: Full keyboard navigation, focus management, and semantic labeling across every hardware control.

## Accessibility & Inclusion

Full keyboard navigation and screen-reader accessibility for all controls, backed by `ui-sdk` WAI-ARIA behaviors.
