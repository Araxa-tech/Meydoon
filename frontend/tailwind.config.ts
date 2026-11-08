import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: "class",
  content: [
    "./app/**/*.{ts,tsx}",
    "./components/**/*.{ts,tsx}",
    "./lib/**/*.{ts,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        page: { DEFAULT: "var(--page)", soft: "var(--page-soft)" },
        card: "var(--card)",
        chip: { DEFAULT: "var(--chip)", text: "var(--chip-text)" },
        line: { DEFAULT: "var(--line)", strong: "var(--line-strong)" },
        ink: { DEFAULT: "var(--ink)", 2: "var(--ink-2)" },
        muted: {
          DEFAULT: "var(--muted)",
          2: "var(--muted-2)",
          3: "var(--muted-3)",
        },
        wash: "var(--wash)",
        lime: "var(--lime)",
        coral: {
          DEFAULT: "var(--coral)",
          hover: "var(--coral-hover)",
          soft: "var(--coral-soft)",
          text: "var(--coral-text)",
        },
        night: { DEFAULT: "var(--night)", 2: "var(--night-2)" },
        "on-night": {
          DEFAULT: "var(--on-night)",
          muted: "var(--on-night-muted)",
          line: "var(--on-night-line)",
        },
      },
      fontFamily: {
        sans: ["var(--font-sans)", "ui-sans-serif", "system-ui", "sans-serif"],
        body: ["var(--font-body)"],
        display: ["var(--font-display)"],
        mono: ["var(--font-mono)", "ui-monospace", "monospace"],
      },
    },
  },
  plugins: [],
};
export default config;