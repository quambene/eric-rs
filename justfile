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

# Run cargo semver-checks with env vars
semver:
    cargo semver-checks

# Generate bindings via bindgen
generate-bindings:
    cargo build -p eric-bindings --features generate-bindings

# Generate bindings via bindgen and save into eric-bindings/bindings (maintainer-only)
save-bindings:
    #!/usr/bin/env bash
    set -euo pipefail
    out_dir=$(cargo build -p eric-bindings --features generate-bindings --message-format=json \
      | grep -o '"out_dir":"[^"]*eric-bindings-[^"]*"' \
      | tail -1 \
      | sed 's/"out_dir":"//;s/"$//')
    if [ -z "$out_dir" ]; then
      echo "Could not determine OUT_DIR for eric-bindings build script" >&2
      exit 1
    fi
    src="$out_dir/bindings.rs"
    host=$(rustc -vV | sed -n 's/^host: //p')
    arch=$(echo "$host" | cut -d- -f1)
    case "$host" in
      *linux*) os=linux ;;
      *darwin*|*apple*) os=darwin ;;
      *) os=$(echo "$host" | cut -d- -f2) ;;
    esac
    version=$(echo "$ERIC_VERSION" | tr '.' '_')
    dest="eric-bindings/bindings/bindings_eric_${version}_${os}_${arch}.rs"
    cp "$src" "$dest"
    echo "Wrote $dest"
