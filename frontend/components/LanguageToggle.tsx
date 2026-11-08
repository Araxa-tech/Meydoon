"use client";

import { useI18n } from "@/lib/i18n";

export function LanguageToggle() {
  const { locale, setLocale, t } = useI18n();

  return (
    <button
      type="button"
      onClick={() => setLocale(locale === "en" ? "fa" : "en")}
      aria-label={t("nav.langToggle")}
      className="inline-flex h-[42px] cursor-pointer items-center rounded-full border border-line-strong px-3 text-xs font-extrabold text-ink transition duration-200 hover:bg-wash"
    >
      {locale === "en" ? "فارسی" : "EN"}
    </button>
  );
}