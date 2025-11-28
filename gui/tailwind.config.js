/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        // Quantum Lab Theme
        quantum: {
          primary: '#6366F1',    // Indigo - quantum energy
          secondary: '#8B5CF6',  // Violet - superposition
          accent: '#06B6D4',     // Cyan - measurement
          success: '#10B981',    // Emerald - valid state
          warning: '#F59E0B',    // Amber - caution
          error: '#EF4444',      // Red - invalid
        },
        // Dark theme backgrounds
        surface: {
          900: '#0F172A',  // Slate-900 - main background
          800: '#1E293B',  // Slate-800 - cards/panels
          700: '#334155',  // Slate-700 - elevated surfaces
          600: '#475569',  // Slate-600 - hover states
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Consolas', 'monospace'],
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'spin-slow': 'spin 3s linear infinite',
      },
    },
  },
  plugins: [],
}
