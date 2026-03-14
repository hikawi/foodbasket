import en from "../i18n/en.json";
import ja from "../i18n/ja.json";

const translations = { en, ja };

export type Language = keyof typeof translations;

export function getTranslations(locale: Language) {
  return translations[locale];
}
