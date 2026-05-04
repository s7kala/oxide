# Oxide

A minimal Rust-based experimental AArch64 operating system for Raspberry Pi 4, focused on exploring kernel fundamentals from bare metal to user space.

## Building

Install the Rust target and binary tooling once:

```sh
rustup target add aarch64-unknown-none
make install-tools
```

Build the kernel ELF:

```sh
make build
```

Produce the Raspberry Pi 4 boot image:

```sh
make img
```

This writes `kernel8.img` in the project root. Copy that file to the Pi boot partition when flashing or updating the SD card.

Clean generated build output:

```sh
make clean
```
