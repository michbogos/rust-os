rustup override set nightly
cargo build -Z build-std=std
rustup component add llvm-tools-preview