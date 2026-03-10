import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";

import "./styles.css";
import { createI18n } from "vue-i18n";

import enUS from "./i18n/en.json";
import jaJP from "./i18n/ja.json";

type I18nSchema = typeof enUS;
const i18n = createI18n<[I18nSchema], "en-US" | "ja-JP">({
  legacy: false,
  availableLocales: ["en-US", "ja-JP"],
  fallbackLocale: "en-US",
  formatFallbackMessages: true,
  messages: {
    "en-US": enUS,
    "ja-JP": jaJP,
  },
  numberFormats: {
    "en-US": {
      currency: {
        style: "currency",
        currency: "USD",
        notation: "standard",
        currencyDisplay: "symbol",
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
      },
      decimal: {
        style: "decimal",
        minimumFractionDigits: 0,
        maximumFractionDigits: 2,
      },
      percent: {
        style: "percent",
        useGrouping: false,
      },
    },
    "ja-JP": {
      currency: {
        style: "currency",
        currency: "USD",
        notation: "standard",
        currencyDisplay: "symbol",
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
      },
      decimal: {
        style: "decimal",
        minimumSignificantDigits: 3,
        maximumSignificantDigits: 5,
      },
      percent: {
        style: "percent",
        useGrouping: false,
      },
    },
  },
});

const app = createApp(App);

app.use(createPinia());
app.use(i18n);
app.use(router);

app.mount("#app");
