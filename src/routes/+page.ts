import { redirect } from "@sveltejs/kit";
import { detectPreferredLocale } from "$lib/i18n/locales";

export const load = () => {
  redirect(307, `/${detectPreferredLocale()}`);
};
