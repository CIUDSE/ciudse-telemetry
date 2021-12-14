# Checkout Code
if cd openmct; then git pull; else git clone https://github.com/nasa/openmct.git; fi

# Build
cd openmct
npm install
npm run build:prod
cd ..

# Install
mkdir -p ./dist/openmct
rsync -a --delete ./openmct/dist/ ./dist/openmct

cd ..
