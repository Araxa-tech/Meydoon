"use client";

import { Hero } from "@/components/Hero";
import { Icon } from "@/components/Icons";
import { SiteHeader } from "@/components/SiteHeader";
import { useI18n } from "@/lib/i18n";
import { useState } from "react";

/* Shared button recipes */
const btnBase =
  "inline-flex cursor-pointer items-center justify-center gap-[.55rem] rounded-full px-[1.2rem] py-[.78rem] text-sm font-bold transition duration-200";


/* Eyebrow: colour is added per section (light / dark / lime) */
const eyebrow =
  "eyebrow mb-4 flex items-center gap-[.65rem] text-[.74rem] font-extrabold uppercase tracking-[.14em]";

export default function Home() {
  const { t, locale } = useI18n();

  const [query, setQuery] = useState("");
  const [location, setLocation] = useState("");
  const [saved, setSaved] = useState<number[]>([]);
  const [searchMessage, setSearchMessage] = useState("");

  const handleSearch = (event: React.FormEvent) => {
    event.preventDefault();
    const role = query.trim() || t("hero.roleDefault");
    const place = location.trim() || t("hero.placeDefault");
    setSearchMessage(t("hero.searchMsg", { role, place }));
    document.querySelector("#jobs")?.scrollIntoView({ behavior: "smooth" });
  };

  const jobs = [
    {
      company: "Lumon",
      initials: "LU",
      color: "#d9ff6b",
      title: t("job1.title"),
      location: t("job1.location"),
      mode: t("job1.mode"),
      salary: t("job1.salary"),
      tags: ["React", "TypeScript", "Design systems"],
      posted: t("job1.posted"),
    },
    {
      company: "Northstar AI",
      initials: "NA",
      color: "#ff8f70",
      title: t("job2.title"),
      location: t("job2.location"),
      mode: t("job2.mode"),
      salary: t("job2.salary"),
      tags: ["Python", "LLMs", "PyTorch"],
      posted: t("job2.posted"),
    },
    {
      company: "Commons",
      initials: "CO",
      color: "#a9c7ff",
      title: t("job3.title"),
      location: t("job3.location"),
      mode: t("job3.mode"),
      salary: t("job3.salary"),
      tags: ["Next.js", "Node.js", "Postgres"],
      posted: t("job3.posted"),
    },
  ];

  const engineers = [
    {
      name: "Maya Chen",
      role: t("eng1.role"),
      skills: "React · WebGL · TypeScript",
      monogram: "MC",
      color: "#ffb99f",
      available: true,
    },
    {
      name: "Noah Williams",
      role: t("eng2.role"),
      skills: "Python · PyTorch · NLP",
      monogram: "NW",
      color: "#b8d1ff",
      available: true,
    },
    {
      name: "Sofia Martins",
      role: t("eng3.role"),
      skills: "React · Node.js · Figma",
      monogram: "SM",
      color: "#d9ff6b",
      available: false,
    },
    {
      name: "Eli Thompson",
      role: t("eng4.role"),
      skills: "Go · AWS · Kubernetes",
      monogram: "ET",
      color: "#ffd36a",
      available: true,
    },
  ];

  const popularTags = [
    t("hero.tagFrontend"),
    t("hero.tagAi"),
    t("hero.tagProduct"),
    t("hero.tagRemote"),
  ];

  const steps = [
    { num: t("steps.1.num"), title: t("steps.1.title"), copy: t("steps.1.copy") },
    { num: t("steps.2.num"), title: t("steps.2.title"), copy: t("steps.2.copy") },
    { num: t("steps.3.num"), title: t("steps.3.title"), copy: t("steps.3.copy") },
  ];


  return (
    <div className="min-h-screen min-w-[320px] overflow-hidden bg-page font-body text-ink [-webkit-tap-highlight-color:transparent]">
      <div>
        <section className="relative bg-page">
          <div
            aria-hidden
            className="pointer-events-none absolute start-0 top-0 h-full w-[43%] bg-[radial-gradient(var(--dots)_1px,transparent_1px)] bg-[length:19px_19px] opacity-[.22] [mask-image:linear-gradient(to_right,black,transparent)] rtl:[mask-image:linear-gradient(to_left,black,transparent)]"
          />
          <div className="relative mx-auto grid max-w-[1320px] items-center gap-8 px-5 py-14 md:py-20 lg:grid-cols-[1.03fr_.97fr] lg:px-8 lg:py-24">
            <div className="relative z-10">
              <div className={`${eyebrow} text-muted`}>
                <span className="h-[3px] w-6 bg-coral" /> {t("hero.eyebrow")}
              </div>
              <h1 className="max-w-[720px] font-display text-[clamp(3.35rem,6.2vw,6rem)] font-bold leading-[.98] tracking-[-.07em]">
                {t("hero.title1")}
                <br />
                {t("hero.title2")}{" "}
                <em className="accent-italic font-normal text-coral">
                  {t("hero.titleEm")}
                </em>
              </h1>
              <p className="mt-6 max-w-[590px] text-lg leading-8 text-muted md:text-xl">
                {t("hero.lead")}
              </p>

              <form
                className="relative mt-[2.1rem] grid max-w-[720px] grid-cols-1 items-center gap-[.35rem] rounded-[18px] border border-line-strong bg-card p-[.55rem] shadow-[0_17px_40px_rgba(37,43,33,.12)] dark:shadow-[0_17px_40px_rgba(0,0,0,.45)] min-[701px]:grid-cols-[1fr_1px_.8fr_auto] min-[701px]:gap-0"
                onSubmit={handleSearch}
              >
                <label className="flex min-w-0 flex-col gap-[.35rem] border-b border-line px-[.7rem] pb-3 pt-[.55rem] min-[701px]:border-0 min-[701px]:px-[.9rem] min-[701px]:py-[.35rem]">
                  <span className="flex items-center gap-[.35rem] text-[.7rem] font-extrabold uppercase tracking-[.04em]">
                    <Icon name="search" size={17} /> {t("hero.labelLooking")}
                  </span>
                  <input
                    className="w-full bg-transparent text-[.92rem] text-ink-2 outline-none placeholder:text-muted-3"
                    value={query}
                    onChange={(e) => setQuery(e.target.value)}
                    placeholder={t("hero.placeholderRole")}
                  />
                </label>
                <div className="hidden h-10 bg-line-strong min-[701px]:block" />
                <label className="flex min-w-0 flex-col gap-[.35rem] border-b border-line px-[.7rem] pb-3 pt-[.55rem] min-[701px]:border-0 min-[701px]:px-[.9rem] min-[701px]:py-[.35rem]">
                  <span className="flex items-center gap-[.35rem] text-[.7rem] font-extrabold uppercase tracking-[.04em]">
                    <Icon name="location" size={17} /> {t("hero.labelWhere")}
                  </span>
                  <input
                    className="w-full bg-transparent text-[.92rem] text-ink-2 outline-none placeholder:text-muted-3"
                    value={location}
                    onChange={(e) => setLocation(e.target.value)}
                    placeholder={t("hero.placeholderWhere")}
                  />
                </label>
                <button
                  className="flex h-[54px] w-full cursor-pointer items-center justify-center gap-2 rounded-[13px] bg-coral px-5 font-extrabold text-[#171a16] transition duration-200 hover:-translate-y-0.5 hover:bg-coral-hover min-[701px]:w-auto"
                  type="submit"
                  aria-label={t("hero.searchAria")}
                >
                  <Icon name="search" size={22} />
                  <span>{t("hero.search")}</span>
                </button>
              </form>

              <div className="mt-5 flex flex-wrap items-center gap-2 text-sm text-muted">
                <span className="me-1 font-semibold text-ink">
                  {t("hero.popular")}
                </span>
                {popularTags.map((tag) => (
                  <button
                    key={tag}
                    className="cursor-pointer rounded-full border border-line-strong bg-card px-[.7rem] py-[.35rem] transition duration-200 hover:bg-lime hover:text-[#1d211c]"
                    onClick={() => setQuery(tag)}
                  >
                    {tag}
                  </button>
                ))}
              </div>
            </div>

            <Hero />
          </div>

          <div className="mx-auto flex max-w-[1320px] flex-wrap items-center justify-between gap-5 border-t border-line px-5 py-5 text-sm lg:px-8">
            <span className="font-semibold text-muted-3">
              {t("hero.trusted")}
            </span>
            {["Arcade", "Northstar", "COMMONS", "kinetic", "Daybreak"].map(
              (brand) => (
                <span
                  key={brand}
                  className="font-display font-extrabold tracking-[-.025em] text-ink-2"
                >
                  {brand}
                </span>
              ),
            )}
            <span className="hidden items-center gap-2 font-semibold text-muted md:flex">
              <span className="h-2 w-2 rounded-full bg-[#51b864] shadow-[0_0_0_4px_rgba(81,184,100,.13)]" />{" "}
              {t("hero.newRoles")}
            </span>
          </div>
        </section>

        {/* ---------------- Jobs ---------------- */}
        <section id="jobs" className="bg-page px-5 py-20 lg:px-8 lg:py-28">
          <div className="mx-auto max-w-[1320px]">
            <div className="mb-10 flex items-end justify-between gap-8 max-[700px]:flex-col max-[700px]:items-start">
              <div>
                <div className={`${eyebrow} text-muted`}>
                  <span className="h-[3px] w-6 bg-coral" /> {t("jobs.eyebrow")}
                </div>
                <h2 className="font-display text-[clamp(2.3rem,4vw,3.7rem)] font-bold leading-none tracking-[-.055em]">
                  {t("jobs.title")}
                </h2>
                <p className="mt-[.9rem] text-[1.05rem] text-muted-2">
                  {t("jobs.subtitle")}
                </p>
              </div>
              <a
                href="#"
                className="inline-flex shrink-0 items-center gap-2 border-b border-ink pb-[.3rem] text-[.9rem] font-extrabold"
              >
                {t("jobs.browseAll")} <Icon name="arrow" size={18} />
              </a>
            </div>

            {searchMessage && (
              <div className="mb-6 -mt-4 flex items-center gap-2 rounded-[10px] bg-[#e5f6c1] px-4 py-[.8rem] text-sm font-bold text-[#1d211c] dark:bg-[#2a3318] dark:text-[#d9ff6b]">
                <Icon name="check" size={18} /> {searchMessage}
              </div>
            )}

            <div className="overflow-hidden rounded-[20px] border border-line bg-card">
              {jobs.map((job, index) => (
                <article
                  className={`group flex items-start gap-[1.4rem] p-[1.7rem] transition-colors duration-200 hover:bg-page-soft max-[700px]:relative max-[700px]:gap-4 max-[700px]:p-5 ${
                    index > 0 ? "border-t border-line" : ""
                  }`}
                  key={job.company}
                >
                  <div
                    className="grid h-[60px] w-[60px] flex-[0_0_60px] place-items-center rounded-2xl border border-line-strong font-display font-extrabold tracking-[-.04em] text-[#1d211c] max-[700px]:h-12 max-[700px]:w-12 max-[700px]:flex-[0_0_48px] max-[700px]:rounded-[13px]"
                    style={{ backgroundColor: job.color }}
                  >
                    {job.initials}
                  </div>
                  <div className="min-w-0 flex-1">
                    <div className="mb-1 flex items-center gap-2 text-sm font-semibold text-muted-3">
                      {job.company}{" "}
                      {index === 0 && (
                        <span className="rounded-full bg-coral-soft px-[.55rem] py-1 text-[.62rem] uppercase tracking-[.08em] text-coral-text">
                          {t("jobs.featured")}
                        </span>
                      )}
                    </div>
                    <h3 className="font-display text-[1.3rem] font-bold tracking-[-.03em] max-[700px]:pe-8 max-[700px]:text-[1.08rem]">
                      {job.title}
                    </h3>
                    <div className="mt-[.7rem] flex flex-wrap items-center gap-x-[1.2rem] gap-y-[.6rem] text-[.82rem] text-muted-2 max-[700px]:flex-col max-[700px]:items-start max-[700px]:gap-[.45rem]">
                      <span className="inline-flex items-center gap-[.35rem]">
                        <Icon name="location" size={16} />
                        {job.location}
                      </span>
                      <span className="inline-flex items-center gap-[.35rem]">
                        <Icon name="briefcase" size={16} />
                        {job.mode}
                      </span>
                      <span>{job.salary}</span>
                    </div>
                    <div className="mt-5 flex flex-wrap gap-2">
                      {job.tags.map((tag) => (
                        <span
                          className="rounded-full bg-chip px-[.7rem] py-[.38rem] text-[.73rem] font-bold text-chip-text"
                          key={tag}
                        >
                          {tag}
                        </span>
                      ))}
                    </div>
                  </div>
                  <div className="flex items-center gap-4 self-stretch max-[700px]:absolute max-[700px]:end-4 max-[700px]:top-4">
                    <button
                      className={`grid h-[38px] w-[38px] cursor-pointer place-items-center rounded-full border transition duration-200 ${
                        saved.includes(index)
                          ? "border-coral bg-coral-soft text-coral"
                          : "border-line-strong text-muted-2"
                      }`}
                      onClick={() =>
                        setSaved((current) =>
                          current.includes(index)
                            ? current.filter((item) => item !== index)
                            : [...current, index],
                        )
                      }
                      aria-label={
                        saved.includes(index)
                          ? t("jobs.unsaveAria", { title: job.title })
                          : t("jobs.saveAria", { title: job.title })
                      }
                    >
                      <Icon
                        name="heart"
                        size={20}
                        filled={saved.includes(index)}
                      />
                    </button>
                    <span className="text-xs font-semibold text-muted-3 max-[700px]:hidden">
                      {job.posted}
                    </span>
                    <button
                      className="grid h-11 w-11 cursor-pointer place-items-center rounded-full bg-ink text-page transition-transform duration-200 group-hover:translate-x-[3px] rtl:group-hover:-translate-x-[3px] max-[700px]:hidden"
                      aria-label={t("jobs.viewAria", { title: job.title })}
                    >
                      <Icon name="arrow" size={19} />
                    </button>
                  </div>
                </article>
              ))}
            </div>
          </div>
        </section>

        {/* ---------------- How it works (dark band) ---------------- */}
        <section id="how-it-works" className="bg-night">
          <div className="mx-auto grid max-w-[1320px] gap-14 px-5 py-20 lg:grid-cols-[.8fr_1.2fr] lg:px-8 lg:py-28">
            <div>
              <div className={`${eyebrow} text-on-night-muted`}>
                <span className="h-[3px] w-6 bg-lime" /> {t("steps.eyebrow")}
              </div>
              <h2 className="font-display text-4xl font-bold leading-tight tracking-[-.04em] text-on-night md:text-5xl">
                {t("steps.title")}
              </h2>
              <p className="mt-5 max-w-md text-lg leading-8 text-on-night-muted">
                {t("steps.lead")}
              </p>
              <a
                href="#"
                className="mt-8 inline-flex items-center gap-2 border-b border-lime pb-1 font-bold text-lime"
              >
                {t("steps.link")} <Icon name="arrow" size={18} />
              </a>
            </div>
            <div className="grid grid-cols-1 rounded-[18px] border border-on-night-line min-[701px]:grid-cols-3">
              {steps.map((step, index) => (
                <div
                  className={`min-h-[190px] p-[1.6rem] min-[701px]:min-h-[275px] ${
                    index > 0
                      ? "border-t border-on-night-line min-[701px]:border-s min-[701px]:border-t-0"
                      : ""
                  }`}
                  key={step.num}
                >
                  <span className="font-display text-[.78rem] font-extrabold tracking-[.1em] text-lime">
                    {step.num}
                  </span>
                  <h3 className="mt-12 font-display text-[1.25rem] font-bold tracking-[-.03em] text-on-night min-[701px]:mt-[5.5rem]">
                    {step.title}
                  </h3>
                  <p className="mt-[.8rem] text-[.88rem] leading-[1.55] text-on-night-muted">
                    {step.copy}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </section>

        {/* ---------------- Engineers ---------------- */}
        <section id="engineers" className="bg-page px-5 py-20 lg:px-8 lg:py-28">
          <div className="mx-auto max-w-[1320px]">
            <div className="mb-10 flex items-end justify-between gap-8 max-[700px]:flex-col max-[700px]:items-start">
              <div>
                <div className={`${eyebrow} text-muted`}>
                  <span className="h-[3px] w-6 bg-coral" />{" "}
                  {t("engineers.eyebrow")}
                </div>
                <h2 className="font-display text-[clamp(2.3rem,4vw,3.7rem)] font-bold leading-none tracking-[-.055em]">
                  {t("engineers.title")}
                </h2>
                <p className="mt-[.9rem] text-[1.05rem] text-muted-2">
                  {t("engineers.subtitle")}
                </p>
              </div>
              <a
                href="#"
                className="inline-flex shrink-0 items-center gap-2 border-b border-ink pb-[.3rem] text-[.9rem] font-extrabold"
              >
                {t("engineers.explore")} <Icon name="arrow" size={18} />
              </a>
            </div>

            <div className="grid grid-cols-1 gap-4 min-[701px]:grid-cols-2 lg:grid-cols-4">
              {engineers.map((engineer) => (
                <article
                  className="rounded-[18px] border border-line bg-card px-[.85rem] pb-5 pt-[.85rem] transition duration-[250ms] hover:-translate-y-[5px] hover:shadow-[0_18px_35px_rgba(31,36,28,.1)]"
                  key={engineer.name}
                >
                  <div
                    className="relative grid h-[230px] place-items-center overflow-hidden rounded-xl text-[#1d211c] min-[701px]:h-[210px]"
                    style={{ backgroundColor: engineer.color }}
                  >
                    <span className="relative z-[2] grid h-[86px] w-[86px] place-items-center rounded-full border-2 border-[#1d211c] bg-white/60 font-display text-2xl font-extrabold backdrop-blur-sm">
                      {engineer.monogram}
                    </span>
                    <div className="absolute h-40 w-40 rounded-full border-2 border-[#1d211c]/[.18] shadow-[0_0_0_25px_rgba(255,255,255,.13),0_0_0_50px_rgba(29,33,28,.06)]" />
                  </div>
                  <div className="mt-6 flex items-start justify-between gap-3">
                    <div>
                      <h3 className="font-display text-[1.12rem] font-bold tracking-[-.03em]">
                        {engineer.name}
                      </h3>
                      <p className="mt-1 text-sm font-semibold text-muted">
                        {engineer.role}
                      </p>
                    </div>
                    <button
                      className="grid h-8 w-8 flex-[0_0_32px] cursor-pointer place-items-center rounded-full border border-line-strong text-ink"
                      aria-label={engineer.name}
                    >
                      <Icon name="arrow" size={17} />
                    </button>
                  </div>
                  <div className="my-5 h-px bg-line" />
                  <p className="text-sm text-muted-2">{engineer.skills}</p>
                  <div className="mt-4 flex items-center justify-between">
                    <span
                      className={`flex items-center gap-[.4rem] text-[.7rem] font-bold ${
                        engineer.available
                          ? "text-[#3a7945] dark:text-[#8ede9a]"
                          : "text-[#9a6a22] dark:text-[#f0c070]"
                      }`}
                    >
                      <span
                        className={`h-[7px] w-[7px] rounded-full ${
                          engineer.available ? "bg-[#51b864]" : "bg-[#eaa83b]"
                        }`}
                      />
                      {engineer.available
                        ? t("engineers.available")
                        : t("engineers.openNextMonth")}
                    </span>
                    <span className="flex items-center gap-[.2rem] text-[.65rem] font-bold text-muted-3">
                      <span className="rounded-full bg-ink p-px text-page">
                        <Icon name="check" size={12} />
                      </span>{" "}
                      {t("engineers.verified")}
                    </span>
                  </div>
                </article>
              ))}
            </div>
          </div>
        </section>

        {/* ---------------- CTA (lime, always bright) ---------------- */}
        <section className="px-5 pb-20 lg:px-8 lg:pb-28">
          <div className="relative flex min-h-[330px] flex-col items-start justify-between gap-8 overflow-hidden rounded-3xl border border-[#1d211c] bg-lime p-6 text-[#1d211c] min-[701px]:p-10 min-[1024px]:flex-row min-[1024px]:items-center min-[1024px]:p-14">
            <div className="relative z-[2]">
              <div className={`${eyebrow} text-[#4a4f42]`}>
                <span className="h-[3px] w-6 bg-[#ff7451]" /> {t("cta.eyebrow")}
              </div>
              <h2 className="font-display text-[clamp(2.4rem,4vw,4.2rem)] font-bold leading-none tracking-[-.06em]">
                {t("cta.title")}
              </h2>
              <p className="mt-4 max-w-[500px] text-[#55594f]">{t("cta.lead")}</p>
            </div>
            <div className="relative z-[2] flex flex-wrap gap-3">
              <button
                className={`${btnBase} bg-[#1d211c] px-[1.4rem] py-4 text-white hover:-translate-y-0.5 hover:bg-[#30352e]`}
              >
                {t("cta.findRole")} <Icon name="arrow" size={18} />
              </button>
              <button
                className={`${btnBase} border border-[#1d211c] px-[1.4rem] py-4 hover:bg-[#1d211c] hover:text-white`}
              >
                {t("cta.hireTalent")}
              </button>
            </div>
            <div
              aria-hidden
              className="pointer-events-none absolute -top-[80%] start-[48%] grid h-[520px] w-[520px] place-items-center rounded-full border-2 border-[#1d211c]/[.12] shadow-[0_0_0_50px_rgba(29,33,28,.035),0_0_0_100px_rgba(29,33,28,.025)]"
            >
              <span className="font-display text-[12rem] font-extrabold text-[#1d211c]/[.07]">
                {locale === "fa" ? "م" : "m"}
              </span>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}
