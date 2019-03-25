#!/usr/bin/env bash

echo "Cleaning output"
if [[ -d build ]]; then
	rm -rf build
fi
mkdir build

echo "Building app"
cd openbook-app
flutter build bundle
cd ..

echo "Copying app data"
cp -r openbook-app/build/flutter_assets build/
cp icudtl.dat build/

echo "Building binaries"
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

echo "Copying binaries"
cp target/x86_64-unknown-linux-gnu/release/openbook-desktop build/
cp target/x86_64-pc-windows-gnu/release/openbook-desktop.exe build/
cp target/x86_64-pc-windows-gnu/release/flutter_engine.dll build/
