#!/bin/sh

# release build
rustup install nightly
rustup +nightly target add thumbv7em-none-eabi
rustup run nightly cargo build --release

# Copy to library directory
mkdir -p rust_nes_emulator_for_arduino_lib/src/cortex-m4
cp rust_nes_emulator_embedded.h rust_nes_emulator_for_arduino_lib/src/rust_nes_emulator_embedded.h
cp target/thumbv7em-none-eabi/release/librust_nes_emulator_embedded.a rust_nes_emulator_for_arduino_lib/src/cortex-m4/librust_nes_emulator_embedded.a
