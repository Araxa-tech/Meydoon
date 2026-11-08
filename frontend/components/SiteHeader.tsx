import { useI18n } from "@/lib/i18n";
import { useState } from "react";
import { LanguageToggle } from "./LanguageToggle";
import { ThemeToggle } from "./ThemeToggle";
import { Icon } from "./Icons";
import { useRouter } from "next/navigation";

const navLink =
  "relative after:absolute after:inset-x-0 after:-bottom-[7px] after:h-0.5 after:origin-left after:scale-x-0 after:bg-ink after:transition-transform after:duration-200 after:content-[''] hover:after:scale-x-100 rtl:after:origin-right";
const btnBase =
  "inline-flex cursor-pointer items-center justify-center gap-[.55rem] rounded-full px-[1.2rem] py-[.78rem] text-sm font-bold transition duration-200";
const btnDark = `${btnBase} bg-ink text-page hover:-translate-y-0.5 hover:bg-ink-2`;
const btnGhost = `${btnBase} hover:bg-wash`;

export function SiteHeader() {
  const router = useRouter();
  const { t, locale } = useI18n();
  const [menuOpen, setMenuOpen] = useState(false);

  return (
      <header className="relative z-50 border-b border-line bg-page">
        <div className="mx-auto flex h-[76px] max-w-[1320px] items-center justify-between gap-3 px-5 lg:px-8">
          <a
            href="#"
            aria-label={t("brand.home")}
            className="inline-flex items-center gap-[.55rem] font-display text-[1.3rem] font-extrabold tracking-[-.04em]"
          >
            <span className="grid h-[31px] w-[31px] place-items-center rounded-full bg-[#1d211c] text-base text-[#d9ff6b] dark:bg-[#d9ff6b] dark:text-[#1d211c]">
              {locale === "fa" ? "م" : "m"}
            </span>
            {t("brand.name")}
          </a>

          <nav
            className="hidden items-center gap-8 text-sm font-semibold lg:flex"
            aria-label={t("nav.primary")}
          >
            <a href="#jobs" className={navLink}>
              {t("nav.findWork")}
            </a>
            <a href="#engineers" className={navLink}>
              {t("nav.findTalent")}
            </a>
            <a href="#how-it-works" className={navLink}>
              {t("nav.howItWorks")}
            </a>
            <a href="#resources" className={navLink}>
              {t("nav.resources")}
            </a>
          </nav>

          <div className="hidden items-center gap-2 lg:flex">
            <LanguageToggle />
            <ThemeToggle />
            <button className={btnGhost} onClick={()=>{router.replace("accounts/login")}}>{t("nav.login")}</button>
            <button className={btnDark}>
              {t("nav.postProject")} <Icon name="arrow" size={17} />
            </button>
          </div>

          <div className="flex items-center gap-2 lg:hidden">
            <LanguageToggle />
            <ThemeToggle />
            <button
              className="grid h-[42px] w-[42px] cursor-pointer place-items-center rounded-full border border-line-strong text-ink"
              onClick={() => setMenuOpen(!menuOpen)}
              aria-label={t("nav.toggleNav")}
              aria-expanded={menuOpen}
            >
              <Icon name={menuOpen ? "close" : "menu"} />
            </button>
          </div>
        </div>

        {menuOpen && (
          <div className="absolute inset-x-0 top-[76px] flex flex-col gap-4 border-b border-line bg-page p-5 font-bold shadow-[0_20px_30px_rgba(0,0,0,.08)] lg:hidden">
            <a href="#jobs" onClick={() => setMenuOpen(false)}>
              {t("nav.findWork")}
            </a>
            <a href="#engineers" onClick={() => setMenuOpen(false)}>
              {t("nav.findTalent")}
            </a>
            <a href="#how-it-works" onClick={() => setMenuOpen(false)}>
              {t("nav.howItWorks")}
            </a>
            <button className={`${btnDark} mt-3`}>
              {t("nav.postProject")} <Icon name="arrow" size={17} />
            </button>
          </div>
        )}
      </header>
    )
}
