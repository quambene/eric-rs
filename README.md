# Eric

[![latest version](https://img.shields.io/crates/v/eric-bindings.svg?label=eric-bindings)](https://crates.io/crates/eric-bindings)
[![documentation](https://img.shields.io/docsrs/eric-bindings?label=eric-bindings)](https://docs.rs/eric-bindings)
[![latest version](https://img.shields.io/crates/v/eric-sdk.svg?label=eric-sdk)](https://crates.io/crates/eric-sdk)
[![documentation](https://img.shields.io/docsrs/eric-sdk?label=eric-sdk)](https://docs.rs/eric-sdk)
[![build status](https://github.com/quambene/eric-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/quambene/eric-rs/actions/workflows/ci.yml)

Rust bindings and SDK for the ELSTER Rich Client (ERiC)

- [What is ELSTER?](#what-is-elster)
- [What is ERiC?](#what-is-eric)
- [Requirements](#requirements)
- [Rust bindings](#rust-bindings)
  - [Generate bindings](#generate-bindings)
  - [Test bindings](#test-bindings)
- [Eric SDK](#eric-sdk)
  - [Usage](#usage)
  - [Supported Eric versions](#supported-eric-versions)
  - [Test SDK](#test-sdk)
- [Changelog](#changelog)

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

To generate the bindings, `eric-bindings` expects the environment variables `LIBRARY_NAME`, `LIBRARY_PATH`, `HEADER_FILE`, and
`PLUGIN_PATH`. For example:

``` bash
PATH_VENDOR="ERiC-40.2.10.0-Linux-x86_64/ERiC-40.2.10.0/Linux-x86_64"
LIBRARY_NAME=ericapi
LIBRARY_PATH="$PATH_VENDOR/lib"
HEADER_FILE="$PATH_VENDOR/include/ericapi.h"
PLUGIN_PATH="$PATH_VENDOR/lib/plugins2"
```

The bindings have to be generated on-the-fly for your specific platform and architecture:

``` bash
cargo build -p eric-bindings
```

The bindings are generated in `target/debug/build/eric-bindings-<random-id>/out/bindings.rs`.

### Test bindings

The bindings are included in `src/lib.rs` via `include!` macro and tested by:

``` bash
cargo test -p eric-bindings --lib
```

Logs are written to `eric.log` in the current directory.

## Eric SDK

`eric-sdk` supports single-threaded Eric instances.

### Usage

To use `eric-sdk`, add the path of the shared C library (e.g. to `LD_LIBRARY_PATH` on Linux).

To send the xml file, the path and password of the Elster certificate has to be provided via environment variables `CERTIFICATE_PATH` and `CERTIFICATE_PASSWORD`.

### Supported Eric versions

| Rust SDK | Eric     |
| -------- | -------- |
| 0.1.0    | 38.1.6.0 |
| 0.2.0    | 39.6.4.0 |
| 0.3.0    | 40.1.8.0 |

### Test SDK

``` bash
# Run unit tests
cargo test -p eric-sdk -- --test-threads=1

# Run integration tests
cargo test -p eric-sdk --test '*' -- --test-threads=1

# Run external tests
cargo test -p eric-sdk --test '*' --features external-test -- --test-threads=1
```

## Changelog

The `eric-rs` repository contains multiple crates with separate changelogs:

- `eric-bindings`: [view changelog](https://github.com/quambene/eric-rs/blob/main/eric-bindings/CHANGELOG.md)
- `eric-sdk`: [view changelog](https://github.com/quambene/eric-rs/blob/main/eric-sdk/CHANGELOG.md)
