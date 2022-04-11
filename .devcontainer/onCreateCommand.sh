rustup target add wasm32-wasi

wget https://github.com/WebAssembly/binaryen/releases/download/version_105/binaryen-version_105-x86_64-linux.tar.gz
tar -xvzf binaryen-version_105-x86_64-linux.tar.gz
mv ./binaryen-version_105 ./binaryen
rm binaryen-version_105-x86_64-linux.tar.gz
