set dotenv-load

# Run unit tests
test-unit:
    cargo test -p eric-sdk --lib --features no-linking

# Run integration tests (requires ERiC library)
test-integration:
    cargo test -p eric-sdk --test '*' -- --test-threads=1
