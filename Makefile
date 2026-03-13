.PHONY: llvm-tools hex flash

# Serial console: set SERIAL_DEVICE to override auto-detect (e.g. /dev/cu.usbmodem* or /dev/ttyACM0)
SERIAL_BAUD ?= 115200

llvm-tools:
	rustup component add llvm-tools

# Build firmware and produce Intel HEX (requires: cargo install cargo-binutils)
hex:
	cargo build -p little-synth-firmware --release --target thumbv7em-none-eabihf
	cargo objcopy -p little-synth-firmware --release --target thumbv7em-none-eabihf -- -O ihex little-synth-firmware.hex

# Flash firmware to Teensy 4.1 (put board in bootloader mode first; builds hex if missing).
# After flashing, opens a serial console (screen). Detach with C-a d.
flash: hex
	teensy_loader_cli --mcu=TEENSY41 -w -v little-synth-firmware.hex
	@echo "Waiting for device to re-enumerate..."
	@sleep 2
	@SERIAL=$$(if [ -n "$$SERIAL_DEVICE" ]; then echo "$$SERIAL_DEVICE"; elif ls /dev/cu.debug-console* 1>/dev/null 2>&1; then ls /dev/cu.usbmodem* 2>/dev/null | head -1; else ls /dev/ttyACM* 2>/dev/null | head -1; fi); \
	if [ -z "$$SERIAL" ] || [ ! -e "$$SERIAL" ]; then \
		echo "No serial device found. Set SERIAL_DEVICE= e.g. SERIAL_DEVICE=/dev/cu.usbmodem12345 make flash"; \
		exit 1; \
	fi; \
	echo "Opening serial console at $$SERIAL $(SERIAL_BAUD) (detach: C-a d)"; \
	exec screen "$$SERIAL" $(SERIAL_BAUD)
