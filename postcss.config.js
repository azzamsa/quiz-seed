module.exports = {
  plugins: [
    require("tailwindcss")('tailwind.config.js'),
    require("autoprefixer"),
    require("postcss-typed-css-classes")({
      generator: "rust",
      output_filepath: "src/generated/css_classes.rs",
      content: [
        { path: ['src/**/*.rs'] }],
    }),
  ]
};
