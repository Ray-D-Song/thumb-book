import { defineConfig, presetTypography, presetUno } from 'unocss'

export default defineConfig({
  presets: [presetTypography(), presetUno()],
  cli: {
    entry: {
      patterns: ["*.html"],
      outFile: "assets/index.css",
    },
  },
})
