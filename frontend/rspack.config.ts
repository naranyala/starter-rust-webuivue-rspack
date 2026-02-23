import { defineConfig } from '@rspack/cli';
import { VueLoaderPlugin } from 'rspack-vue-loader';
import HtmlRspackPlugin from '@rspack/plugin-html';
import { DefinePlugin } from '@rspack/core';
import path from 'node:path';

export default defineConfig({
  entry: {
    index: './src/main.ts',
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
    extensions: ['.ts', '.js', '.vue', '.json'],
  },
  plugins: [
    new VueLoaderPlugin(),
    new DefinePlugin({
      'import.meta.env': JSON.stringify({
        MODE: process.env.NODE_ENV || 'production',
        VITE_APP_VERSION: '1.0.0',
      }),
    }),
    new HtmlRspackPlugin({
      template: './index.html',
    }),
  ],
  module: {
    rules: [
      {
        test: /\.vue$/,
        loader: 'rspack-vue-loader',
        options: {
          experimentalInlineMatchResource: true,
        },
      },
      {
        test: /\.css$/,
        type: 'css',
      },
      {
        test: /\.module\.css$/,
        type: 'css',
        generator: {
          localIdentName: '[local]',
        },
      },
      {
        test: /\.ts$/,
        loader: 'builtin:swc-loader',
        options: {
          jsc: {
            parser: {
              syntax: 'typescript',
            },
          },
        },
        type: 'javascript/auto',
      },
    ],
  },
  experiments: {
    css: true,
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    publicPath: './',
    filename: 'static/js/[name].[contenthash:8].js',
    cssFilename: 'static/css/[name].[contenthash:8].css',
    clean: true,
  },
  server: {
    port: 3000,
    strictPort: true,
    open: true,
  },
  optimization: {
    splitChunks: {
      chunks: 'all',
      hidePathInfo: true,
      maxInitialRequests: 20,
      maxAsyncRequests: 20,
      cacheGroups: {
        defaultVendors: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          priority: -10,
          chunks: 'all',
        },
        default: {
          minChunks: 2,
          priority: -20,
          reuseExistingChunk: true,
        },
      },
    },
  },
  stats: {
    preset: 'normal',
  },
  cache: {
    type: 'filesystem',
    buildDependencies: {
      config: [__filename],
    },
  },
});