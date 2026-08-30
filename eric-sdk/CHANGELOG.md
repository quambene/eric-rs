<!-- markdownlint-disable MD041 -->

## Unreleased

- added
- changed
- removed

## 0.7.2 (2026-08-31)

- added
  - Add `Eric::certificate_properties`, a safe wrapper over `EricHoleZertifikatEigenschaften`

## 0.7.1 (2026-08-26)

- removed
  - Remove `transfer_code` from `Eric::process`

## 0.7.0 (2026-07-21)

- added
  - Support `ERIC_VERSION` environment variable as the version detection signal
  - Remove `docs-rs` feature
- changed
  - Update eric-bindings to 0.6.1

## v0.6.0 (2026-05-15)

- added
  - Provide `Eric::check_xml`
  - Parse `ValidationReport` from `EricError`
  - Add `ValidationIssue` and `ValidationReport`
  - Add `EricApiPayload`
- changed
  - Provide path and password as parameter in `Eric::send`
  - Make `log_path` in `Eric::new` optional, and add parameter `plugin_path`
  - Refactor logging to use `tracing` crate
- removed
  - Remove `error_code` from `EricResponse`
- fixed
  - Fix dangling pointer

## v0.5.0 (2026-02-24)

- added
  - Add structured error `EricError`
  - Add feature `generate-bindings`

## v0.4.0 (2025-12-30)

This release requires Eric library >= 43.x. Older versions are no longer supported.
Rust API is backward-compatible, but linking to older Eric versions will fail.

- added
  - Support ERiC v43.3.2.0

## v0.3.1 (2024-08-07)

- added
  - Add feature flag `docs-rs`
- changed
  - Fix publishing on docs.rs

## v0.3.0 (2024-08-07)

- added
  - Support ERiC v40.1.8.0

## v0.2.0 (2024-08-05)

- added
  - Support Eric v39.6.4.0

## v0.1.0 (2024-08-05)

- added
  - Validate and send xml
  - Support Eric v38.1.6.0
