use std::{
    env, fmt, io,
    path::{Path, PathBuf},
};

/// Supported versions of the Eric library.
#[derive(Debug)]
pub enum EricVersion {
    Eric38_1_6_0,
    Eric39_6_4_0,
    Eric40_1_8_0,
    Eric40_2_10_0,
    Eric43_3_2_0,
}

impl fmt::Display for EricVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let request_method = match self {
            Self::Eric38_1_6_0 => "38.1.6.0",
            Self::Eric39_6_4_0 => "39.6.4.0",
            Self::Eric40_1_8_0 => "40.1.8.0",
            Self::Eric40_2_10_0 => "40.2.10.0",
            Self::Eric43_3_2_0 => "43.3.2.0",
        };

        write!(f, "{request_method}")
    }
}

pub fn main() -> io::Result<()> {
    #[cfg(feature = "generate-bindings")]
    generate_bindings()?;

    #[cfg(not(feature = "generate-bindings"))]
    {
        #[cfg(not(feature = "docs-rs"))]
        select_bindings()?;

        #[cfg(feature = "docs-rs")]
        select_bindings_for_docs_rs()?;
    }

    Ok(())
}

/// Select existing bindings
#[cfg(not(feature = "generate-bindings"))]
#[cfg(not(feature = "docs-rs"))]
fn select_bindings() -> io::Result<()> {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("environment variable `environment variable` not set");
    let is_windows = std::env::var("CARGO_CFG_WINDOWS").is_ok();
    let eric_path = env::var("ERIC_PATH").expect("environment variable `ERIC_PATH` not set");

    let out_dir = env::var("OUT_DIR").expect("environment variable `OUT_DIR` not set");
    let bindings_target = PathBuf::from(out_dir).join("bindings.rs");

    let eric_version = if eric_path.contains("38.1.6.0") {
        EricVersion::Eric38_1_6_0
    } else if eric_path.contains("39.6.4.0") {
        EricVersion::Eric39_6_4_0
    } else if eric_path.contains("40.1.8.0") {
        EricVersion::Eric40_1_8_0
    } else if eric_path.contains("40.2.10.0") {
        EricVersion::Eric40_2_10_0
    } else if eric_path.contains("43.3.2.0") {
        EricVersion::Eric43_3_2_0
    } else {
        panic!("Missing bindings: Unknown Eric version");
    };

    println!("Select bindings for Eric version {eric_version} and target {target_arch}");

    let bindings_file = match (&eric_version, target_arch.as_ref(), is_windows) {
        (EricVersion::Eric38_1_6_0, "x86_64", false) => "bindings_eric_38_1_6_0_linux_x86_64.rs",
        (EricVersion::Eric39_6_4_0, "x86_64", false) => "bindings_eric_39_6_4_0_linux_x86_64.rs",
        (EricVersion::Eric40_1_8_0, "x86_64", false) => "bindings_eric_40_1_8_0_linux_x86_64.rs",
        (EricVersion::Eric40_2_10_0, "x86_64", false) => "bindings_eric_40_2_10_0_linux_x86_64.rs",
        (EricVersion::Eric43_3_2_0, "x86_64", false) => "bindings_eric_43_3_2_0_linux_x86_64.rs",
        _ => {
            panic!("Missing bindings for Eric version {eric_version} and target {target_arch}");
        }
    };

    #[cfg(not(feature = "no-linking"))]
    {
        let eric_path = Path::new(&eric_path);
        let library_name = get_library_name();
        let library_path = get_library_path(eric_path);
        let header_file = get_header_file(eric_path);

        println!("cargo:rustc-link-search={}", library_path.display());
        println!("cargo:rustc-link-lib={}", library_name);
        println!("cargo:rerun-if-changed={}", header_file.display());
        println!("cargo:rustc-env=LD_LIBRARY_PATH={}", library_path.display());
    }

    let root_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("environment variable `CARGO_MANIFEST_DIR` not set");
    let bindings_path = Path::new(&root_dir).join("bindings").join(bindings_file);

    std::fs::copy(bindings_path.clone(), bindings_target.clone()).unwrap_or_else(|_| {
        panic!(
            "Can't copy file from {} to {}",
            bindings_path.display(),
            bindings_target.display(),
        )
    });

    Ok(())
}

/// Generate bindings on-the-fly
#[cfg(feature = "generate-bindings")]
fn generate_bindings() -> io::Result<()> {
    let eric_path = env::var("ERIC_PATH").expect("environment variable `ERIC_PATH` not set");
    let eric_path = Path::new(&eric_path);
    let library_name = get_library_name();
    let library_path = get_library_path(eric_path);
    let header_file = get_header_file(eric_path);

    println!("cargo:rustc-link-search={}", library_path.display());
    println!("cargo:rustc-link-lib={}", library_name);
    println!("cargo:rerun-if-changed={}", header_file.display());
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", library_path.display());

    let header = header_file.to_str().expect("Can't convert path to string");

    let bindings = bindgen::Builder::default()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Can't generate bindings");

    let out_dir = env::var("OUT_DIR").expect("environment variable `OUT_DIR` not set");
    let output_path = PathBuf::from(out_dir);

    bindings
        .write_to_file(output_path.join("bindings.rs"))
        .expect("Can't write bindings");

    Ok(())
}

/// Select latest bindings for documentation on docs.rs
#[cfg(feature = "docs-rs")]
fn select_bindings_for_docs_rs() -> io::Result<()> {
    let bindings_file = "bindings_eric_43_3_2_0_linux_x86_64.rs";

    let root_dir = env::var("CARGO_MANIFEST_DIR").expect("Set by cargo");
    let bindings_path = Path::new(&root_dir).join("bindings").join(bindings_file);

    let out_dir = env::var("OUT_DIR").expect("environment variable `OUT_DIR` not set");
    let bindings_target = PathBuf::from(out_dir).join("bindings.rs");

    std::fs::copy(bindings_path.clone(), bindings_target.clone()).unwrap_or_else(|_| {
        panic!(
            "Can't copy file from {} to {}",
            bindings_path.display(),
            bindings_target.display(),
        )
    });
    Ok(())
}

#[cfg(not(feature = "docs-rs"))]
fn get_library_name() -> String {
    env::var("LIBRARY_NAME").unwrap_or_else(|_| "ericapi".to_owned())
}

#[cfg(not(feature = "docs-rs"))]
fn get_library_path(eric_path: &Path) -> PathBuf {
    use std::str::FromStr;

    env::var("LIBRARY_PATH")
        .ok()
        .map(|path| PathBuf::from_str(&path).expect("invalid path for `LIBRARY_PATH`"))
        .unwrap_or_else(|| eric_path.join("lib"))
}

#[cfg(not(feature = "docs-rs"))]
fn get_header_file(eric_path: &Path) -> PathBuf {
    use std::str::FromStr;

    env::var("HEADER_FILE")
        .ok()
        .map(|path| PathBuf::from_str(&path).expect("invalid path for `HEADER_FILE`"))
        .unwrap_or_else(|| eric_path.join("include").join("ericapi.h"))
}
