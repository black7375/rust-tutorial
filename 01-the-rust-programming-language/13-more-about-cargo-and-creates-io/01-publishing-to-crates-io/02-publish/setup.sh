#== Account ====================================================================
# 1. Regestor at https://crates.io/
# 2. Get API token at https://crates.io/me/
# 3. Cargo login
#    Will be saved at `~/.cargo/credentials`
cargo login YOUR_API_KEY

#== Meta Data ==================================================================
# Meta Data check at `Cargo.toml`

#== Publish ====================================================================
cargo publish

#== Version Remove =============================================================
# Just not dependency setting
cargo yank --vers 1.0.1

# undo
cargo yank --vers 1.0.1 --undo
