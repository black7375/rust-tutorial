# Don't want to parallel test
cargo test -- --test-threads=1

# No Capture print(Standard Output)
cargo test -- --nocapture

# Function test
cargo test test_function_name_pattern

# Run ignore
cargo test -- --ignored
