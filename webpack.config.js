const path = require('path')
const CopyPlugin = require('copy-webpack-plugin')

const src = path.join(__dirname, 'src')

module.exports = {
  entry: path.join(src, 'main.js'),
  output: {
    filename: 'bundle.js',
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
  externals: {
    openmct: 'openmct'
  },
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    compress: true,
    port: 8000
  }
}
