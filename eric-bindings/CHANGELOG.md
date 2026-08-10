<!-- markdownlint-disable MD041 -->

## Unreleased

- added
- changed
- removed

## 0.7.0 (unreleased)

- added
  - Add bindings for Eric v44.1.6.0 (Linux x86_64)
  - Add bindings for Eric v44.1.6.0 (Darwin aarch64)
- removed
  - `EricBearbeiteVorgang` lost its `transferHandle` parameter in ERiC 44; the v44.1.6.0 bindings reflect that upstream API change.

## 0.6.1 (2026-07-21)

- fixed
  - Fix build failure on docs.rs

## 0.6.0 (2026-07-21)

- added
  - Add `bindgen-comments` feature flag to opt in to C header doc comments in generated bindings (off by default)
  - Add bindings for Eric v43.4.6.0 (Linux x86_64)
  - Add bindings for Eric v43.4.6.0 (Darwin aarch64)
  - Support `ERIC_VERSION` environment variable as the version detection signal
  - Remove `docs-rs` feature

## 0.5.1 (2026-02-24)

- added
  - Add bindings for macOS aarch64
- changed
  - Match on `CARGO_CFG_TARGET_OS` instead of `CARGO_CFG_WINDOWS` for more explicit platform selection
- removed
  - `cargo:rustc-env=LD_LIBRARY_PATH=...` build instruction

## v0.5.0 (2025-12-30)

- added
  - Test `cargo package` in CI pipeline
  - Add bindings for Eric v43.3.2.0

## v0.4.1 (2024-08-07)

- added
  - Add feature flag `docs-rs`
- changed
  - Fix publishing on docs.rs

## v0.4.0 (2024-08-07)

- added
  - Add bindings for Eric v40.1.8.0
  - Add feature flag `generate-bindings`
- changed
  - Select existing bindings
- removed
  - Remove feature flag `no-build`

## v0.3.0 (2024-08-04)

- changed
  - Support documentation for Eric v39.6.4.0

## v0.2.0 (2024-08-04)

- changed
  - Support documentation for Eric v38.1.6.0
  - Rename feature from `docs-rs` to `no-build`

## v0.1.2 (2024-08-03)

- changed
  - Disable `build.rs` on docs.rs

## v0.1.1 (2024-08-03)

- added
  - Add feature flag `docs-rs`
- changed
  - Fix documentation on docs.rs

## v0.1.0 (2024-08-03)

- added
  - Generate bindings on-the-fly
