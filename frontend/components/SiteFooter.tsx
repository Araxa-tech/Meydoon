"use client";

import { useI18n } from "@/lib/i18n";

export default function SiteFooter() {
   const { t, locale } = useI18n();
   const footerColumns = [
     [
       t("footer.forTalent"),
       t("footer.browseJobs"),
       t("footer.browseProjects"),
       t("footer.careerResources"),
     ],
     [
       t("footer.forCompanies"),
       t("footer.findEngineers"),
       t("footer.postProject"),
       t("footer.hiringGuide"),
     ],
     [t("footer.brand"), t("footer.about"), t("footer.contact"), t("footer.privacy")],
   ];
   return (
      <footer id="resources" className="bg-night-2">
        <div className="mx-auto grid max-w-[1320px] gap-12 px-5 py-14 md:grid-cols-[1.5fr_1fr_1fr_1fr] lg:px-8">
          <div>
            <a
              href="#"
              aria-label={t("brand.home")}
              className="inline-flex items-center gap-[.55rem] font-display text-[1.3rem] font-extrabold tracking-[-.04em] text-on-night"
            >
              <span className="grid h-[31px] w-[31px] place-items-center rounded-full bg-lime text-base text-[#1d211c]">
                {locale === "fa" ? "م" : "m"}
              </span>
              {t("brand.name")}
            </a>
            <p className="mt-5 max-w-xs text-sm leading-6 text-on-night-muted">
              {t("footer.tagline")}
            </p>
          </div>

          {footerColumns.map(([title, ...links]) => (
            <div key={title}>
              <h3 className="mb-5 font-display text-sm font-bold text-on-night">
                {title}
              </h3>
              <div className="flex flex-col gap-3">
                {links.map((link) => (
                  <a
                    className="text-sm text-on-night-muted transition duration-200 hover:text-on-night"
                    href="#"
                    key={link}
                  >
                    {link}
                  </a>
                ))}
              </div>
            </div>
          ))}
        </div>
        <div className="mx-auto flex max-w-[1320px] flex-col gap-3 border-t border-on-night-line px-5 py-6 text-xs text-on-night-muted md:flex-row md:items-center md:justify-between lg:px-8">
          <span>{t("footer.copyright")}</span>
          <span>{t("footer.madeBy")}</span>
        </div>
      </footer>
   ) 
}
