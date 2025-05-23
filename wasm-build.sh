#!/bin/bash
set -eux
export RUST_LOG=info 

# Build the wasm package
wasm-pack build --profiling -- --features=wasm_js --no-default-features

cd www

# Remove the existing node_modules/rockies if it exists
rm -rf node_modules/rockies

# Create a symlink instead of copying
mkdir -p node_modules
ln -sf ../../pkg node_modules/rockies

# Start webpack dev server
npm run start
