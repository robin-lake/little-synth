.PHONY: llvm-tools hex flash

llvm-tools:
	rustup component add llvm-tools

# Build firmware and produce Intel HEX (requires: cargo install cargo-binutils)
hex:
	cargo build -p little-synth-firmware --release --target thumbv7em-none-eabihf
	cargo objcopy -p little-synth-firmware --release --target thumbv7em-none-eabihf -- -O ihex little-synth-firmware.hex

# Flash firmware to Teensy 4.1 (put board in bootloader mode first; builds hex if missing)
flash: hex
	teensy_loader_cli --mcu=TEENSY41 -w -v little-synth-firmware.hex
