import { getInfo, errMsg, type Info, type Supported } from "./api";

export const emptySupported: Supported = {
  interfaces: [],
  aura_modes: [],
  fan_curves: false,
  slash: false,
  screenpad: false,
  armoury: false,
};

export const app = $state<{ info: Info | null; loading: boolean; error: string | null }>({
  info: null,
  loading: true,
  error: null,
});

let inflight: Promise<void> | null = null;

export async function loadInfo() {
  if (inflight) return inflight;
  app.loading = true;
  inflight = (async () => {
    try {
      app.info = await getInfo();
      app.error = null;
    } catch (e) {
      app.error = errMsg(e);
    } finally {
      app.loading = false;
      inflight = null;
    }
  })();
  return inflight;
}

export type ToastKind = "success" | "error";
export const toasts = $state<{ list: { id: number; kind: ToastKind; text: string }[] }>({
  list: [],
});
let nextId = 1;

export function dismiss(id: number) {
  toasts.list = toasts.list.filter((t) => t.id !== id);
}
export function toast(text: string, kind: ToastKind = "success") {
  const id = nextId++;
  toasts.list.push({ id, kind, text });
  setTimeout(() => dismiss(id), 4000);
}
