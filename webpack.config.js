const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');

const src = path.join(__dirname, 'src')

module.exports = {
  // mode: 'production',
  mode: 'development',
  entry: {
    main: path.join(src, 'main.js')
  },
  output: {
    filename: '[name].js',
    path: path.join(__dirname, 'dist')
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        {
          from: path.join(__dirname, 'node_modules', 'openmct', 'dist'),
          to: 'openmct',
          globOptions: {
            ignore: [
              '**/index.html'
            ]
          }
        },
        { from: 'src/index.html' }
      ]
    })
  ],
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    compress: true,
    port: 8080,
  },
};