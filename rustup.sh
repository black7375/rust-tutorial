curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y

# https://rust-lang.github.io/rustup/concepts/toolchains.html
rustup toolchain install nightly --component clippy
