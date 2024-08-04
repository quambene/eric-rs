# Eric

[![latest version](https://img.shields.io/crates/v/eric-bindings.svg)](https://crates.io/crates/eric-bindings)
[![documentation](https://docs.rs/eric-bindings/badge.svg)](https://docs.rs/eric-bindings)
[![build status](https://github.com/quambene/eric-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/quambene/eric-rs/actions/workflows/ci.yml)

Rust bindings and SDK for the ELSTER Rich Client (ERiC)

- [What is ELSTER?](#what-is-elster)
- [What is ERiC?](#what-is-eric)
- [Requirements](#requirements)
- [Rust bindings](#rust-bindings)
  - [Generate bindings](#generate-bindings)
  - [Test bindings](#test-bindings)

## What is ELSTER?

Elster (short for _Elektronische Steuererklärung_) is a project by the German tax administrations to process tax returns and declarations.

## What is ERiC?

ERiC is a shared C library that is integrated into a tax application. ERiC checks the data supplied by the tax application for plausibility, and transmits the data encrypted to the computing center of the respective tax administration.

## Requirements

You need to have the shared library `libericapi.so` and the header file `ericapi.h` available on your system which can be downloaded from [ELSTER for developers](https://www.elster.de/elsterweb/entwickler/login) after access has been requested [here](https://www.elster.de/elsterweb/registrierung-entwickler/form).

For generating the bindings on your platform and architecture, you need `libclang` as well. For example, on Debian/Ubuntu install:

``` bash
apt install llvm-dev libclang-dev clang
```

## Rust bindings

### Generate bindings

The environment variables `LIBRARY_NAME`, `LIBRARY_PATH`, `HEADER_FILE`, and
`PLUGIN_PATH` are expected. For example:

``` bash
LIBRARY_NAME=ericapi
LIBRARY_PATH=ERiC-38.1.6.0-Linux-x86_64/ERiC-38.1.6.0/Linux-x86_64/lib
HEADER_FILE=ERiC-38.1.6.0-Linux-x86_64/ERiC-38.1.6.0/Linux-x86_64/include/ericapi.h
PLUGIN_PATH=ERiC-38.1.6.0-Linux-x86_64/ERiC-38.1.6.0/Linux-x86_64/lib/plugins2
```

The bindings have to be generated on-the-fly for your specific platform and architecture:

``` bash
cargo build -p eric-bindings # Build bindings in debug mode
```

The bindings are generated in `target/debug/build/eric-bindings-<random-id>/out/bindings.rs`.

### Test bindings

The bindings are included in `src/lib.rs` via `include!` macro and tested by:

``` bash
cargo test -p eric-bindings --lib
```

Logs are written to `eric.log` in the current directory.
