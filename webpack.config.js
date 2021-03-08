const path = require('path')
const CopyPlugin = require('copy-webpack-plugin')

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
          to: 'openmct-assets',
          globOptions: {
            ignore: [
              '**/index.html',
              '**/openmct.js',
              '**/openmct.js.map'
            ]
          }
        },
        { from: 'src/index.html' }
      ]
    })
  ],
  resolve: {
    alias: {
      node_modules: path.resolve(__dirname, 'node_modules')
    }
  },
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    compress: true,
    port: 8000
  }
}
