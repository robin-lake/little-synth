.PHONY: llvm-tools hex flash

# Serial console: set SERIAL_DEVICE to override auto-detect (e.g. /dev/cu.usbmodem* or /dev/ttyACM0)
SERIAL_BAUD ?= 115200

llvm-tools:
	rustup component add llvm-tools

# Matches workspace `.cargo/config.toml`: imxrt-log full-speed CDC + bulk MPS (required for hosts).
export IMXRT_LOG_USB_SPEED ?= FULL
export IMXRT_LOG_USB_BULK_MPS ?= 64

# Build firmware and produce Intel HEX (requires: cargo install cargo-binutils)
# Only flash-backed sections: default objcopy dumps OCRAM VMA (0x2020xxxx) too; teensy_loader_cli
# rejects those addresses (16 MiB image buffer). Omit BSS/stack/heap/defmt RAM-only sections.
hex:
	cargo build -p little-synth-firmware --release --target thumbv7em-none-eabihf
	cargo objcopy -p little-synth-firmware --release --target thumbv7em-none-eabihf -- \
		-j .boot -j .vector_table -j .text -j .rodata -j .data \
		-O ihex little-synth-firmware.hex

# Flash firmware to Teensy 4.1 (put board in bootloader mode first; builds hex if missing).
# After flashing, opens a serial console (screen). Detach with C-a d.
flash: hex
	teensy_loader_cli --mcu=TEENSY41 -w -v little-synth-firmware.hex
	@echo "Waiting for device to re-enumerate..."
	@sleep 3
	@if [ "$$SERIAL_SKIP" = 1 ]; then echo "SERIAL_SKIP=1: not opening serial console."; exit 0; fi
	@SERIAL="$$SERIAL_DEVICE"; \
	if [ -z "$$SERIAL" ]; then SERIAL=$$(ls /dev/cu.usbmodem* 2>/dev/null | head -1); fi; \
	if [ -z "$$SERIAL" ]; then SERIAL=$$(ls /dev/cu.usbserial* 2>/dev/null | head -1); fi; \
	if [ -z "$$SERIAL" ]; then SERIAL=$$(ls /dev/ttyACM* 2>/dev/null | head -1); fi; \
	if [ -z "$$SERIAL" ] || [ ! -e "$$SERIAL" ]; then \
		echo "Flash finished OK. No USB serial device found yet."; \
		echo "Try: ls /dev/cu.*   Look for cu.usbmodem* (product imxrt-log)."; \
		exit 0; \
	fi; \
	echo "Opening serial console at $$SERIAL $(SERIAL_BAUD) (detach: C-a d)"; \
	exec screen "$$SERIAL" $(SERIAL_BAUD)
