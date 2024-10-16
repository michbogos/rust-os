#! /usr/bin/env bash
cargo build
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-rust-os.bin