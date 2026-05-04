TARGET := aarch64-unknown-none
PROFILE := debug
KERNEL := target/$(TARGET)/$(PROFILE)/oxide
IMG := kernel8.img

.PHONY: build img clean install-tools

build:
	cargo build --target $(TARGET)

img: build
	rust-objcopy -O binary $(KERNEL) $(IMG)

clean:
	cargo clean
	rm -f $(IMG)

install-tools:
	rustup component add llvm-tools-preview
	cargo install cargo-binutils
