#!/bin/sh

# release build
rustup install nightly
rustup +nightly target add thumbv7em-none-eabihf
rustup run nightly cargo build --release

# Copy to library directory
mkdir -p "rust_nes_emulator_embedded/src/cortex-m4"
mkdir -p "rust_nes_emulator_embedded/src/cortex-m4 -mfpu=fpv4-sp-d16 -mfloat-abi=hard"
cp "rust_nes_emulator_embedded.h" "rust_nes_emulator_embedded/src/rust_nes_emulator_embedded.h"
cp "target/thumbv7em-none-eabihf/release/librust_nes_emulator_embedded.a" "rust_nes_emulator_embedded/src/cortex-m4/rust_nes_emulator_embedded.a"
cp "target/thumbv7em-none-eabihf/release/librust_nes_emulator_embedded.a" "rust_nes_emulator_embedded/src/cortex-m4 -mfpu=fpv4-sp-d16 -mfloat-abi=hard/rust_nes_emulator_embedded.a"
zip -r rust_nes_emulator_embedded.zip rust_nes_emulator_embedded