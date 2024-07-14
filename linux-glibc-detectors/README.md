# GLibc detector

This directory contains the source code for a very simple program to detect the glibc version using the `gnu_get_libc_version` function.

## Building

To build the program, we need to make sure that we link to an ancient version of glibc to ensure maximum compatiblity.
We can use [cargo zigbuild](https://github.com/rust-cross/cargo-zigbuild) to use `zig` as the linker to allow linking against a specific glibc version without having to muck about with docker images.

To build to program, run the following commands:

```sh
ARCH=x86_64-unknown-linux-gnu

# Make sure you have the appropriate target to build for. 
rustup target add $ARCH

# Build the program using zigbuild
cargo +nightly zigbuild -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target $ARCH --release
```