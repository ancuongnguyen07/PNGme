const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
// const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
    mode: "development",
    entry: {
        index: "./bootstrap.js"
    },
    output: {
        path: dist,
        filename: "bootstrap.js",
        // publicPath: '/',
    },
    devServer: {
        static: {
            directory: dist,
        },
        port: 7800,
        compress: true,
        hot: true,
    },
    experiments: {
        syncWebAssembly: true // Enable WebAssembly experiments
    },
    plugins: [
        new CopyPlugin({
            patterns: [
                { from: "index.html", to: "index.html" }
            ]
        }),

        // new HtmlWebpackPlugin({
        //     template: 'index.html'
        // }),

        // new WasmPackPlugin({
        //     crateDirectory: __dirname,
        // }),
    ]
};