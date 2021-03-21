const postcssRustHelpers = require("./scripts/postcss_rust_helpers");

module.exports = ( ctx ) => {
  // we want to filter out unused css classes in production mode
  // NOTE: ctx.env is set in postcss-cli `--env` configs
  const usedCssClasses =
        ctx.env === "production"
        ? postcssRustHelpers.getUsedCssClasses()
        : null;

  console.log("postcss ctx.env: " + ctx.env);
  return {
    plugins: [
      require("tailwindcss")('tailwind.config.js'),
      require("autoprefixer"),
      require("postcss-typed-css-classes")({
        output_filepath: "src/generated/css_classes.rs",
        generator: "rust",
        filter: class_ => {
          if (ctx.env === "production") {
            return usedCssClasses.has(
              postcssRustHelpers.escapeClassName(class_)
            );
          } else {
            return true;
          }
        }
      }),
    ]
  };
};
