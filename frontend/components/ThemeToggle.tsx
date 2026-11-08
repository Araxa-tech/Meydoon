"use client";

import { useEffect, useState } from "react";
import { Icon } from "./Icons";
import { useI18n } from "@/lib/i18n";

export function ThemeToggle() {
  const { t } = useI18n();
  const [dark, setDark] = useState(false);

  // The inline script in <head> has already applied the class before paint.
  useEffect(() => {
    setDark(document.documentElement.classList.contains("dark"));
  }, []);

  const toggle = () => {
    const next = !document.documentElement.classList.contains("dark");
    document.documentElement.classList.toggle("dark", next);
    try {
      localStorage.setItem("theme", next ? "dark" : "light");
    } catch {
      /* storage may be unavailable */
    }
    setDark(next);
  };

  return (
    <button
      type="button"
      onClick={toggle}
      aria-label={t("nav.themeToggle")}
      className="grid h-[42px] w-[42px] cursor-pointer place-items-center rounded-full border border-line-strong text-ink transition duration-200 hover:bg-wash"
    >
      <Icon name={dark ? "sun" : "moon"} size={18} />
    </button>
  );
}