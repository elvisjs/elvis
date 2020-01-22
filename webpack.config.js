const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");
const webpack = require("webpack");

module.exports = {
  devServer: {
    hot: true,
    port: 5000,
  },
  devtool: "inline-source-map",
  entry: "./template/bootstrap",
  mode: "development",
  module: {
    rules: [{
      exclude: /node_modules/,
      test: /\.tsx?$/,
      use: "ts-loader",
    }, {
      enforce: "pre",
      test: /\.ts$/,
      use: [
        {
          loader: 'tslint-loader',
          options: {
            configFile: "tslint.json",
          },
        },
      ],
    }],
  },
  output: {
    filename: "[name].bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
  plugins: [
    new HtmlWebpackPlugin(),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."),
      outName: "elvis",
    }),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "pkg"),
    },
    extensions: ['.ts', '.js', '.wasm']
  },
};