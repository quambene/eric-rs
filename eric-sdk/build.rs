use std::env;

/// ERiC 44 removed the `transferHandle` parameter from
/// `EricBearbeiteVorgang`. The SDK compiles against whichever bindings
/// `ERIC_VERSION` selects, so the call site must match that signature at
/// compile time. Emit `eric_lt_44` for older versions; the version parsing
/// itself is validated by eric-bindings' build script, which fails the
/// build first on an unknown version.
pub fn main() {
    println!("cargo:rerun-if-env-changed=ERIC_VERSION");
    println!("cargo:rustc-check-cfg=cfg(eric_lt_44)");

    let eric_version = match env::var("ERIC_VERSION") {
        Ok(version) => version,
        // docs.rs has no ERIC_VERSION; eric-bindings falls back to its
        // LATEST (>= 44), so not emitting the cfg matches that choice.
        Err(_) => return,
    };

    let major: u32 = eric_version
        .split('.')
        .next()
        .and_then(|major| major.parse().ok())
        .unwrap_or_else(|| panic!("Can't parse major version from ERIC_VERSION={eric_version:?}"));

    if major < 44 {
        println!("cargo:rustc-cfg=eric_lt_44");
    }
}
