"use client";

import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
} from "react";

export type Locale = "en" | "fa";

const en = {
  "brand.name": "meydoon",
  "brand.home": "Meydoon home",

  "nav.primary": "Primary navigation",
  "nav.findWork": "Find work",
  "nav.findTalent": "Find talent",
  "nav.howItWorks": "How it works",
  "nav.resources": "Resources",
  "nav.login": "Log in",
  "nav.postProject": "Post a project",
  "nav.toggleNav": "Toggle navigation",
  "nav.themeToggle": "Toggle theme",
  "nav.langToggle": "Switch language",

  "hero.eyebrow": "Where good work meets great people",
  "hero.title1": "Find your way",
  "hero.title2": "to",
  "hero.titleEm": "great work.",
  "hero.lead":
    "The meeting point for ambitious engineers and companies building what's next. No noise, just the right opportunities.",
  "hero.labelLooking": "I'm looking for",
  "hero.labelWhere": "Where",
  "hero.placeholderRole": "Role, skill, or project",
  "hero.placeholderWhere": "Remote or city",
  "hero.search": "Search",
  "hero.searchAria": "Search jobs",
  "hero.popular": "Popular:",
  "hero.tagFrontend": "Frontend",
  "hero.tagAi": "AI / ML",
  "hero.tagProduct": "Product",
  "hero.tagRemote": "Remote",
  "hero.roleDefault": "engineering",
  "hero.placeDefault": "all locations",
  "hero.searchMsg": "Showing the best {role} opportunities in {place}.",
  "hero.trusted": "Trusted by teams at",
  "hero.newRoles": "432 new roles this week",

  "jobs.eyebrow": "Fresh opportunities",
  "jobs.title": "Top jobs & projects",
  "jobs.subtitle": "Hand-picked roles from teams doing meaningful work.",
  "jobs.browseAll": "Browse all opportunities",
  "jobs.featured": "Featured",
  "jobs.saveAria": "Save {title}",
  "jobs.unsaveAria": "Unsave {title}",
  "jobs.viewAria": "View {title}",

  "job1.title": "Senior Product Engineer",
  "job1.location": "San Francisco, CA",
  "job1.mode": "Hybrid",
  "job1.salary": "$180k – $220k",
  "job1.posted": "2h ago",
  "job2.title": "Founding ML Engineer",
  "job2.location": "New York, NY",
  "job2.mode": "On-site",
  "job2.salary": "$190k – $240k",
  "job2.posted": "5h ago",
  "job3.title": "Full-stack Developer",
  "job3.location": "Worldwide",
  "job3.mode": "Remote",
  "job3.salary": "$140k – $175k",
  "job3.posted": "1d ago",

  "steps.eyebrow": "Built differently",
  "steps.title": "A clearer road to your next move.",
  "steps.lead":
    "We removed the busywork from hiring so everyone can focus on the work that matters.",
  "steps.link": "See how Meydoon works",
  "steps.1.num": "01",
  "steps.1.title": "Set your direction",
  "steps.1.copy":
    "Tell us what you're great at and where you want to go.",
  "steps.2.num": "02",
  "steps.2.title": "Meet your match",
  "steps.2.copy":
    "Get relevant roles or talent—not an endless, noisy feed.",
  "steps.3.num": "03",
  "steps.3.title": "Make it happen",
  "steps.3.copy":
    "Connect directly, move quickly, and build something great.",

  "engineers.eyebrow": "People to know",
  "engineers.title": "Top engineers",
  "engineers.subtitle":
    "Exceptional builders, ready for their next challenge.",
  "engineers.explore": "Explore all talent",
  "engineers.available": "Available now",
  "engineers.openNextMonth": "Open next month",
  "engineers.verified": "Verified",
  "eng1.role": "Staff Frontend Engineer",
  "eng2.role": "AI / ML Engineer",
  "eng3.role": "Product Engineer",
  "eng4.role": "Backend Engineer",

  "cta.eyebrow": "Your turn",
  "cta.title": "Where will you go next?",
  "cta.lead":
    "Join thousands of engineers and teams already moving work forward.",
  "cta.findRole": "Find your next role",
  "cta.hireTalent": "Hire great talent",

  "footer.tagline":
    "The meeting point for ambitious engineers and companies building what's next.",
  "footer.forTalent": "For talent",
  "footer.browseJobs": "Browse jobs",
  "footer.browseProjects": "Browse projects",
  "footer.careerResources": "Career resources",
  "footer.forCompanies": "For companies",
  "footer.findEngineers": "Find engineers",
  "footer.postProject": "Post a project",
  "footer.hiringGuide": "Hiring guide",
  "footer.brand": "Meydoon",
  "footer.about": "About us",
  "footer.contact": "Contact",
  "footer.privacy": "Privacy",
  "footer.copyright": "© 2025 Meydoon. Good work starts here.",
  "footer.madeBy": "Made for builders, by builders.",
} as const;

export type TranslationKey = keyof typeof en;

const fa: Record<TranslationKey, string> = {
  "brand.name": "میدون",
  "brand.home": "خانه میدون",

  "nav.primary": "ناوبری اصلی",
  "nav.findWork": "یافتن کار",
  "nav.findTalent": "جذب استعداد",
  "nav.howItWorks": "چطور کار می‌کند",
  "nav.resources": "منابع",
  "nav.login": "ورود",
  "nav.postProject": "ثبت پروژه",
  "nav.toggleNav": "باز و بسته کردن منو",
  "nav.themeToggle": "تغییر پوسته",
  "nav.langToggle": "تغییر زبان",

  "hero.eyebrow": "جایی که کار خوب به آدم‌های خوب می‌رسد",
  "hero.title1": "راهت را",
  "hero.title2": "به",
  "hero.titleEm": "کارِ عالی پیدا کن.",
  "hero.lead":
    "نقطه‌ی تلاقی مهندسان بلندپرواز و شرکت‌هایی که آینده را می‌سازند. بدون شلوغی، فقط فرصت‌های درست.",
  "hero.labelLooking": "دنبال چه هستی؟",
  "hero.labelWhere": "کجا",
  "hero.placeholderRole": "نقش، مهارت یا پروژه",
  "hero.placeholderWhere": "دورکاری یا شهر",
  "hero.search": "جست‌وجو",
  "hero.searchAria": "جست‌وجوی موقعیت‌ها",
  "hero.popular": "محبوب:",
  "hero.tagFrontend": "فرانت‌اند",
  "hero.tagAi": "هوش مصنوعی",
  "hero.tagProduct": "محصول",
  "hero.tagRemote": "دورکاری",
  "hero.roleDefault": "مهندسی",
  "hero.placeDefault": "همه‌ی مکان‌ها",
  "hero.searchMsg":
    "بهترین فرصت‌های {role} در {place} را نمایش می‌دهیم.",
  "hero.trusted": "مورد اعتماد تیم‌هایی در",
  "hero.newRoles": "۴۳۲ نقش جدید این هفته",

  "jobs.eyebrow": "فرصت‌های تازه",
  "jobs.title": "بهترین موقعیت‌ها و پروژه‌ها",
  "jobs.subtitle":
    "نقش‌های دست‌چین‌شده از تیم‌هایی که کار معنادار می‌کنند.",
  "jobs.browseAll": "مرور همه‌ی فرصت‌ها",
  "jobs.featured": "ویژه",
  "jobs.saveAria": "ذخیره‌ی {title}",
  "jobs.unsaveAria": "حذف {title} از ذخیره‌ها",
  "jobs.viewAria": "مشاهده‌ی {title}",

  "job1.title": "مهندس ارشد محصول",
  "job1.location": "سان‌فرانسیسکو، کالیفرنیا",
  "job1.mode": "ترکیبی",
  "job1.salary": "۱۸۰ تا ۲۲۰ هزار دلار",
  "job1.posted": "۲ ساعت پیش",
  "job2.title": "مهندس بنیان‌گذار یادگیری ماشین",
  "job2.location": "نیویورک",
  "job2.mode": "حضوری",
  "job2.salary": "۱۹۰ تا ۲۴۰ هزار دلار",
  "job2.posted": "۵ ساعت پیش",
  "job3.title": "توسعه‌دهنده‌ی فول‌استک",
  "job3.location": "سراسر جهان",
  "job3.mode": "دورکاری",
  "job3.salary": "۱۴۰ تا ۱۷۵ هزار دلار",
  "job3.posted": "۱ روز پیش",

  "steps.eyebrow": "متفاوت ساخته شده",
  "steps.title": "مسیری روشن‌تر به قدم بعدی.",
  "steps.lead":
    "کارهای وقت‌گیر استخدام را حذف کردیم تا همه روی کارِ مهم تمرکز کنند.",
  "steps.link": "ببین میدون چطور کار می‌کند",
  "steps.1.num": "۰۱",
  "steps.1.title": "مسیرت را مشخص کن",
  "steps.1.copy": "بگو در چه چیزی خوبی و می‌خواهی کجا برسی.",
  "steps.2.num": "۰۲",
  "steps.2.title": "با گزینه‌ی مناسب آشنا شو",
  "steps.2.copy":
    "نقش‌ها یا استعدادهای مرتبط را ببین؛ نه یک فید بی‌پایان و شلوغ.",
  "steps.3.num": "۰۳",
  "steps.3.title": "کار را پیش ببر",
  "steps.3.copy":
    "مستقیم وصل شو، سریع پیش برو و چیز بزرگی بساز.",

  "engineers.eyebrow": "آدم‌هایی که باید بشناسی",
  "engineers.title": "مهندسان برتر",
  "engineers.subtitle": "سازنده‌های برجسته، آماده‌ی چالش بعدی.",
  "engineers.explore": "مشاهده‌ی همه‌ی استعدادها",
  "engineers.available": "اکنون در دسترس",
  "engineers.openNextMonth": "از ماه آینده",
  "engineers.verified": "تأییدشده",
  "eng1.role": "مهندس ارشد فرانت‌اند",
  "eng2.role": "مهندس هوش مصنوعی / یادگیری ماشین",
  "eng3.role": "مهندس محصول",
  "eng4.role": "مهندس بک‌اند",

  "cta.eyebrow": "نوبت توست",
  "cta.title": "قدم بعدی‌ات کجاست؟",
  "cta.lead":
    "به هزاران مهندس و تیمی بپیوند که کار را به جلو می‌برند.",
  "cta.findRole": "نقش بعدی‌ات را پیدا کن",
  "cta.hireTalent": "استعدادهای عالی را جذب کن",

  "footer.tagline":
    "نقطه‌ی تلاقی مهندسان بلندپرواز و شرکت‌هایی که آینده را می‌سازند.",
  "footer.forTalent": "برای استعدادها",
  "footer.browseJobs": "مرور موقعیت‌ها",
  "footer.browseProjects": "مرور پروژه‌ها",
  "footer.careerResources": "منابع شغلی",
  "footer.forCompanies": "برای شرکت‌ها",
  "footer.findEngineers": "یافتن مهندس",
  "footer.postProject": "ثبت پروژه",
  "footer.hiringGuide": "راهنمای استخدام",
  "footer.brand": "میدون",
  "footer.about": "درباره‌ی ما",
  "footer.contact": "تماس",
  "footer.privacy": "حریم خصوصی",
  "footer.copyright": "© ۲۰۲۵ میدون. کارِ خوب از اینجا شروع می‌شود.",
  "footer.madeBy": "ساخته‌شده برای سازنده‌ها، به دست سازنده‌ها.",
};

const dictionaries: Record<Locale, Record<TranslationKey, string>> = {
  en,
  fa,
};

type Vars = Record<string, string | number>;

type I18nContextValue = {
  locale: Locale;
  dir: "ltr" | "rtl";
  setLocale: (locale: Locale) => void;
  toggleLocale: () => void;
  t: (key: TranslationKey, vars?: Vars) => string;
};

const I18nContext = createContext<I18nContextValue | null>(null);

export function I18nProvider({
  initialLocale = "en",
  children,
}: {
  initialLocale?: Locale;
  children: React.ReactNode;
}) {
  const [locale, setLocaleState] = useState<Locale>(initialLocale);
  const dir: "ltr" | "rtl" = locale === "fa" ? "rtl" : "ltr";

  const setLocale = useCallback((next: Locale) => {
    setLocaleState(next);
    document.documentElement.lang = next;
    document.documentElement.dir = next === "fa" ? "rtl" : "ltr";
    // Persist so the server renders the right language on the next request.
    document.cookie = `locale=${next}; path=/; max-age=31536000; samesite=lax`;
  }, []);

  const t = useCallback(
    (key: TranslationKey, vars?: Vars) => {
      let value = dictionaries[locale][key] ?? en[key] ?? key;
      if (vars) {
        for (const [name, replacement] of Object.entries(vars)) {
          value = value.split(`{${name}}`).join(String(replacement));
        }
      }
      return value;
    },
    [locale],
  );

  const value = useMemo<I18nContextValue>(
    () => ({
      locale,
      dir,
      setLocale,
      toggleLocale: () => setLocale(locale === "en" ? "fa" : "en"),
      t,
    }),
    [locale, dir, setLocale, t],
  );

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n() {
  const ctx = useContext(I18nContext);
  if (!ctx) throw new Error("useI18n must be used inside <I18nProvider>");
  return ctx;
}