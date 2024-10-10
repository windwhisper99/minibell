import { Configuration } from "@rspack/cli";

const config: Configuration = {
  entry: {
    main: "./js/main.js",
    style: "./styles/app.css",
  },
  output: { path: "assets" },
  experiments: { css: true },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: ["postcss-loader"],
        type: "css",
      },
    ],
  },
};

export default config;
