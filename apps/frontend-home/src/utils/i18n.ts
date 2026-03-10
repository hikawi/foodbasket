export function getTranslations(locale: string | undefined) {
  return import(`../i18n/${locale || "en"}.json`);
}
