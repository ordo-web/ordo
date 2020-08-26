const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
module.exports = (args, options) => {
  /**
   * Configure React-App
   */
  const appConfig = {
    // webpack will take the files from ./src/index
    entry: "./src/index",

    // and output it into /dist as bundle.js
    output: {
      path: path.join(__dirname, "/dist"),
      filename: "bundle.js",
    },

    // adding .ts and .tsx to resolve.extensions will help babel look for .ts and .tsx files to transpile
    resolve: {
      modules: ["src", "node_modules"],
      extensions: [".ts", ".tsx", ".js", ".wasm"],
    },

    module: {
      rules: [
        // we use babel-loader to load our jsx and tsx files
        {
          test: /\.(ts|js)x?$/,
          exclude: /node_modules/,
          use: {
            loader: "babel-loader",
          },
        },
        // loader for source maps
        {
          enforce: "pre",
          test: /\.js$/,
          use: "source-map-loader",
        },
        {
          enforce: "pre",
          test: /\.ts?$/,
          use: "source-map-loader",
        },
        // css-loader to bundle all the css files into one file and style-loader to add all the styles  inside the style tag of the document
        {
          test: /\.css$/,
          use: ["style-loader", "css-loader"],
        },
      ],
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: "./src/index.html",
      }),
      /**new WorkboxPlugin.GenerateSW({
                swDest: 'appSW.js'
            }),*/
    ],

    devtool: "source-map",
    devServer: {
      hot: true,
    },
  };

  /*
   * Configure WebWorker with WebAssembly
   */
  const workerConfig = {
    entry: "./src/worker/worker",
    target: "webworker",

    resolve: {
      extensions: [".ts", ".js", ".wasm"],
    },
    output: {
      path: path.resolve(__dirname, "dist"),
      publicPath: "/",
      filename: "worker.js",
    },
    module: {
      rules: [
        // we use babel-loader to load our jsx and tsx files
        {
          test: /\.(ts|js)x?$/,
          exclude: /node_modules/,
          use: {
            loader: "babel-loader",
          },
        },
        // loader for source maps
        {
          enforce: "pre",
          test: /\.js$/,
          use: "source-map-loader",
        },
        {
          enforce: "pre",
          test: /\.ts?$/,
          use: "source-map-loader",
        },
      ],
    },
    /**plugins: [
       new WorkboxPlugin.GenerateSW({
                swDest: 'workerSW.js'
            }),
       ],*/
    devtool: "source-map",
  };

  return [appConfig, workerConfig];
};
