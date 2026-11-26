import { defineConfig } from "dialexjs";

export default defineConfig({
  defaultLocale: "en",
  locales: ["en", "tr", "de", "fr", "es", "ru", "zh", "ja"],
  include: "src/**/*.content.ts",
  output: "src/lib/dialex.generated.ts",
});
