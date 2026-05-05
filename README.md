<!-- markdownlint-disable MD059 -->

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
  - [Select bindings](#select-bindings)
  - [Generate bindings](#generate-bindings)
  - [Test bindings](#test-bindings)
- [Eric SDK](#eric-sdk)
  - [Usage](#usage)
  - [Supported Eric versions](#supported-eric-versions)
  - [Test SDK](#test-sdk)
- [Changelog](#changelog)

## What is ELSTER?

Elster (short for _Elektronische Steuererklärung_) is a project by the German tax administration to process tax returns and declarations.

## What is ERiC?

ERiC (short for _Elster Rich Client_) is a shared C library that is integrated into a tax application. ERiC checks the data provided by the tax application for plausibility, and transmits the validated data in encrypted form to the computing center of the respective tax administration.

## Requirements

You need to have the shared library `libericapi.so` and the header file `ericapi.h` available on your system which can be downloaded from [ELSTER for developers](https://www.elster.de/elsterweb/entwickler/login) after access has been requested [here](https://www.elster.de/elsterweb/registrierung-entwickler/form).

## Rust bindings

Specify the path to the Eric library via environment variable `ERIC_PATH`. For
example:

``` bash
ERIC_PATH="ERiC-43.3.2.0-Linux-x86_64/ERiC-43.3.2.0/Linux-x86_64"
```

Relevant environment variables then default to

``` bash
LIBRARY_NAME=ericapi
LIBRARY_PATH="$ERIC_PATH/lib"
HEADER_FILE="$ERIC_PATH/include/ericapi.h"
PLUGIN_PATH="$ERIC_PATH/lib/plugins"
```

### Select bindings

Select bindings from the pre-generated bindings:

``` bash
cargo build -p eric-bindings
```

### Generate bindings

You can also generate bindings on-the-fly for your specific platform and
architecture by using feature flag `generate-bindings`:

``` bash
cargo build -p eric-bindings --features generate-bindings
```

The bindings are generated in
`target/debug/build/eric-bindings-<random-id>/out/bindings.rs`.

To generate the bindings on your platform and architecture, you need `libclang` as well. For example, on Debian/Ubuntu install:

``` bash
apt install llvm-dev libclang-dev clang
```

### Test bindings

The bindings are included in `src/lib.rs` via `include!` macro and tested by:

``` bash
cargo test -p eric-bindings --lib
```

Logs are written to `eric.log` in the current directory.

## Eric SDK

`eric-sdk` supports single-threaded Eric instances.

### Usage

To use `eric-sdk`, add the shared C library to your path (e.g. to `LD_LIBRARY_PATH` on Linux):

``` bash
export LD_LIBRARY_PATH="$ERIC_PATH/lib:$LD_LIBRARY_PATH"
```

To send the xml file, the path and password of the Elster certificate have to be provided.

### Supported Eric versions

Currently, only the latest version of the Eric library is supported.

| Rust SDK | Rust bindings | Eric     |
| -------- | ------------- | -------- |
| 0.1.0    | 0.2.0         | 38.1.6.0 |
| 0.2.0    | 0.3.0         | 39.6.4.0 |
| 0.3.0    | 0.4.0         | 40.1.8.0 |
| 0.5.0    | 0.5.1         | 43.3.2.0 |

### Test SDK

``` bash
# Run unit tests
cargo test -p eric-sdk --lib

# Run integration tests (requires ERiC library)
cargo test -p eric-sdk --test '*' -- --test-threads=1
```

Tests that interact with the ELSTER servers are enabled via feature flag
`external-test`. To run these tests you need to provide the following environment variables:

- `CERTIFICATE_PATH`: Path to your ELSTER certificate (`.pfx`)
- `CERTIFICATE_PASSWORD`: Password for your certificate
- `VENDOR_ID`: Your official ELSTER Vendor ID (Hersteller-ID)

``` bash
# Run external tests (requires ERiC library and credentials)
cargo test -p eric-sdk --test '*' --features external-test -- --test-threads=1
```

## Changelog

The `eric-rs` repository contains multiple crates with separate changelogs:

- `eric-bindings`: [view changelog](https://github.com/quambene/eric-rs/blob/main/eric-bindings/CHANGELOG.md)
- `eric-sdk`: [view changelog](https://github.com/quambene/eric-rs/blob/main/eric-sdk/CHANGELOG.md)
