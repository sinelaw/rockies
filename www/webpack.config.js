const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        'index.html',
        'favicon.ico'
      ]
    })
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  target: 'web',
};
