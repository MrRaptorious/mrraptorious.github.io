// tailwind.config.js
const plugin = require("tailwindcss/plugin");

module.exports = {
  mode: 'jit',
  content: [
    "./src/**/*.{html,js,ts,svelte}", // Scan Svelte, JS, TS, and HTML files
  ],
  plugins: [
  ],
  theme: {
    extend: {
      colors: {
        rosewater: { DEFAULT: "rgb(var(--color-rosewater-500))" },
        flamingo: { DEFAULT: "rgb(var(--color-flamingo-500))" },
        pink: { DEFAULT: "rgb(var(--color-pink-500))" },
        mauve: { DEFAULT: "rgb(var(--color-mauve-500))" },
        red: { DEFAULT: "rgb(var(--color-red-500))" },
        maroon: { DEFAULT: "rgb(var(--color-maroon-500))" },
        peach: { DEFAULT: "rgb(var(--color-peach-500))" },
        yellow: { DEFAULT: "rgb(var(--color-yellow-500))" },
        green: { DEFAULT: "rgb(var(--color-green-500))" },
        teal: { DEFAULT: "rgb(var(--color-teal-500))" },
        sky: { DEFAULT: "rgb(var(--color-sky-500))" },
        sapphire: { DEFAULT: "rgb(var(--color-sapphire-500))" },
        blue: { DEFAULT: "rgb(var(--color-blue-500))" },
        lavender: { DEFAULT: "rgb(var(--color-lavender-500))" },
        text: { DEFAULT: "rgb(var(--color-text-500))" },
        subtext1: { DEFAULT: "rgb(var(--color-subtext1-500))" },
        subtext0: { DEFAULT: "rgb(var(--color-subtext0-500))" },
        overlay2: { DEFAULT: "rgb(var(--color-overlay2-500))" },
        overlay1: { DEFAULT: "rgb(var(--color-overlay1-500))" },
        overlay0: { DEFAULT: "rgb(var(--color-overlay0-500))" },
        surface2: { DEFAULT: "rgb(var(--color-surface2-500))" },
        surface1: { DEFAULT: "rgb(var(--color-surface1-500))" },
        surface0: { DEFAULT: "rgb(var(--color-surface0-500))" },
        base: { DEFAULT: "rgb(var(--color-base-500))" },
        mantle: { DEFAULT: "rgb(var(--color-mantle-500))" },
        crust: { DEFAULT: "rgb(var(--color-crust-500))" },
      }
    }
  }
};
