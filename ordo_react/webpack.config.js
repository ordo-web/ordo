const path = require("path");

// TODO remove devtool: "inline-source-map" in prod build
module.exports = {
  entry: "./src/index.ts",
  mode: "production",
  devtool: "inline-source-map",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "ordo-react.js",
    libraryTarget: "umd",
  },
};
