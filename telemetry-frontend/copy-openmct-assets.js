// NodeJS script to copy OpenMCT assets to the dist folder

const fs = require('fs-extra')
const path = require('path');

const openmctPath = path.join(__dirname, 'node_modules', 'openmct', 'dist');
const distPath = path.join(__dirname, 'dist', 'openmctAssets');

// Create the dist folder if it doesn't exist + parent folders
fs.mkdirSync(distPath, { recursive: true });

const exclude = [
    'index.html',
    'openmct.js',
]

// copy all files from `openmctPath` to `distPath` except for those in `exclude`, including subfolders
fs.readdirSync(openmctPath).forEach(file => {
    if (!exclude.includes(file)) {
        // copy file or folder
        fs.copySync(path.join(openmctPath, file), path.join(distPath, file));
    }
});
