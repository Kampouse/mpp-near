/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './app/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        near: {
          dark: '#1F1F1F',
          gray: '#2B2B2B',
          light: '#4A4A4A',
        }
      }
    },
  },
  plugins: [],
}
