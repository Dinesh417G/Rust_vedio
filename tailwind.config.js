/** @type {import('tailwindcss').Config} */
export default {
  darkMode: "class",
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        surface: {
          0: "rgb(var(--s0) / <alpha-value>)",
          1: "rgb(var(--s1) / <alpha-value>)",
          2: "rgb(var(--s2) / <alpha-value>)",
          3: "rgb(var(--s3) / <alpha-value>)",
        },
        ink: {
          DEFAULT: "rgb(var(--ink) / <alpha-value>)",
          muted: "rgb(var(--ink-muted) / <alpha-value>)",
          faint: "rgb(var(--ink-faint) / <alpha-value>)",
        },
        line: "rgb(var(--line) / <alpha-value>)",
        brand: {
          DEFAULT: "rgb(var(--brand) / <alpha-value>)",
          soft: "rgb(var(--brand-soft) / <alpha-value>)",
        },
        danger: "rgb(var(--danger) / <alpha-value>)",
        warn: "rgb(var(--warn) / <alpha-value>)",
        ok: "rgb(var(--ok) / <alpha-value>)",
      },
      fontFamily: {
        sans: ["Inter", "Segoe UI", "system-ui", "sans-serif"],
        mono: ["Cascadia Code", "Consolas", "SFMono-Regular", "monospace"],
      },
      keyframes: {
        flash: {
          "0%,100%": { boxShadow: "0 0 0 0 rgb(var(--brand) / 0)" },
          "30%": { boxShadow: "0 0 0 4px rgb(var(--brand) / 0.45)" },
        },
      },
      animation: { flash: "flash 1.2s ease-out 1" },
    },
  },
  plugins: [],
};
