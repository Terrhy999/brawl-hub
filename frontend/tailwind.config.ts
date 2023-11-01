import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  plugins: [],
  theme: {
    extend: {
      colors: {
        // 'accent-color': '#6D8B4D',
        // 'accent-color': '#3A5925',
        'accent-color': '#9499ff',
        'bg-color': '#1e1e20',
        // 'bg-color': 'black',
        // 'bg-color': #0f0f0f,
        // 'bg-color': #141d26,
        // 'bg-color': #1E1E1E,
        'header-color': '#161618',
        // 'header-color': '#a2ac94',
        // 'header-color': #000000 opacity-20
      },
    },
  },
}
export default config
