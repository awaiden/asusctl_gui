import common from "./i18n/common.content";
import pages from "./i18n/pages.content";

export const dialex = {
  dictionaries: [common, pages],
  config: {
    defaultLocale: "en",
    locales: ["en", "tr", "de", "fr", "es", "ru", "zh", "ja"],
  },
};

type DictShape<D> = D extends import("dialexjs").DictionaryDefinition<
  string,
  infer T extends Record<string, any>
>
  ? T
  : never;

declare module "dialexjs/svelte" {
  interface DictionaryRegistry {
    common: DictShape<typeof common>;
    pages: DictShape<typeof pages>;
  }
}
