TARGET := aarch64-unknown-none
PROFILE := debug
KERNEL := target/$(TARGET)/$(PROFILE)/oxide
IMG := kernel8.img

.PHONY: build img clean install-tools deploy test

build:
	cargo build --target $(TARGET)

img: build
	rust-objcopy -O binary $(KERNEL) $(IMG)

# Copy kernel8.img and boot/config.txt to SD card boot partition
# Usage: make deploy SDCARD=/path/to/sdcard/mount/point
deploy: img
	@if [ -z "$(SDCARD)" ]; then \
		echo "Usage: make deploy SDCARD=/path/to/sdcard/boot"; \
		exit 1; \
	fi
	cp $(IMG) $(SDCARD)/
	cp boot/config.txt $(SDCARD)/

test:
	cargo test --lib --target x86_64-unknown-linux-gnu

clean:
	cargo clean
	rm -f $(IMG)

install-tools:
	rustup component add llvm-tools-preview
	cargo install cargo-binutils
