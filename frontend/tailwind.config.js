/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{svelte,js,ts}'],
  theme: {
    colors: {
      // Color Palettes
      'pine-teal': {
        50: '#eef6f5',
        100: '#ddeeeb',
        200: '#bbddd6',
        300: '#99ccc2',
        400: '#77bbad',
        500: '#55aa99',
        600: '#44887a',
        700: '#33665c',
        800: '#22443d',
        900: '#11221f',
        950: '#0c1815',
      },
      'hunter-green': {
        50: '#edf8ef',
        100: '#daf1df',
        200: '#b6e2bf',
        300: '#91d49f',
        400: '#6cc680',
        500: '#47b860',
        600: '#39934d',
        700: '#2b6e39',
        800: '#1d4926',
        900: '#0e2513',
        950: '#0a1a0d',
      },
      'wine-plum': {
        50: '#f8edef',
        100: '#f1dadf',
        200: '#e3b5c0',
        300: '#d590a0',
        400: '#c76b81',
        500: '#b94661',
        600: '#94384e',
        700: '#6f2a3a',
        800: '#4a1c27',
        900: '#250e13',
        950: '#1a0a0e',
      },
      'sunflower-gold': {
        50: '#fef6e6',
        100: '#feeccd',
        200: '#fdd99b',
        300: '#fcc669',
        400: '#fbb337',
        500: '#faa005',
        600: '#c88004',
        700: '#966003',
        800: '#644002',
        900: '#322001',
        950: '#231601',
      },
      'bright-snow': {
        50: '#f0f0f4',
        100: '#e2e2e9',
        200: '#c5c5d3',
        300: '#a8a8bd',
        400: '#8b8ba7',
        500: '#6e6e91',
        600: '#585874',
        700: '#424257',
        800: '#2c2c3a',
        900: '#16161d',
        950: '#0f0f14',
      },
      // Semantic colors mapped to new palette (WCAG AAA 7:1 compliant)
      'dark-bg': '#0c1815',        // pine-teal-950 - Base background (darkest)
      'content-bg': '#0a1a0d',     // hunter-green-950 - Containers and cards
      'text-primary': '#99ccc2',   // pine-teal-300 - Main text (10.17:1 contrast)
      'text-secondary': '#77bbad', // pine-teal-400 - Supporting text (8.21:1 contrast)
      'accent': '#faa005',         // sunflower-gold-500 - Headers / highlights (8.72:1 contrast)
      'success': '#47b860',        // hunter-green-500 - Success state (7.17:1 contrast)
      'error': '#d590a0',          // wine-plum-300 - Error state (7.20:1 contrast)
      'surface-hover': '#11221f',  // pine-teal-900 - Hover for secondary surfaces
      'transparent': 'transparent',
      'white': '#ffffff',
    },
    extend: {
      fontFamily: {
        sans: ['-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
