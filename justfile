set dotenv-load

# Run unit tests
test-unit:
    cargo test -p eric-sdk --lib --features no-linking

# Run integration tests (requires ERiC library)
test-integration:
    cargo test -p eric-sdk --test '*' -- --test-threads=1

# Run cargo check with env vars
check:
    cargo check

# Run cargo clippy with env vars
clippy:
    cargo clippy --all-targets --all-features

# Run cargo build with env vars
build:
    cargo build

# Run cargo build with env vars in release mode
build-release:
    cargo build --release
