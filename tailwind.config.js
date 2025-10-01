// /** @type {import('tailwindcss').Config} */
// module.exports = {
//   content: ["./src/**/*.{js,ts,jsx,tsx}"],
//   theme: {
//     extend: {
//       transitionProperty: {
//         width: "width",   // enables transition-width
//         spacing: "margin, padding", // optional, smoother sidebar feel
//       },
//       colors: {
//         sidebar: {
//           DEFAULT: "#1f2937", // gray-800 like feel
//           hover: "#374151",   // gray-700
//           active: "#111827",  // gray-900
//         },
//       },
//     },
//   },
//   plugins: [],
// }

// tailwind.config.js
module.exports = {
  content: ["./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        sidebar: "#1F1F1F", // Dark background
        sidebarHover: "#333333", // Hover state
        sidebarActive: "#4C6FFF", // Active item highlight
        textMuted: "#B0B0B0", // Muted text
        textHighlight: "#FFFFFF", // Highlighted text
      },
    },
  },
  plugins: [],
}
