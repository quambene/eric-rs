use std::{
    env, fmt, io,
    path::{Path, PathBuf},
    str::FromStr,
};

/// Supported versions of the Eric library.
#[derive(Debug)]
pub enum EricVersion {
    Eric38_1_6_0,
    Eric39_6_4_0,
    Eric40_1_8_0,
    Eric40_2_10_0,
    Eric43_3_2_0,
    Eric43_4_6_0,
}

#[cfg(not(feature = "generate-bindings"))]
impl EricVersion {
    const LATEST: Self = Self::Eric43_4_6_0;
}

impl fmt::Display for EricVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = match self {
            Self::Eric38_1_6_0 => "38.1.6.0",
            Self::Eric39_6_4_0 => "39.6.4.0",
            Self::Eric40_1_8_0 => "40.1.8.0",
            Self::Eric40_2_10_0 => "40.2.10.0",
            Self::Eric43_3_2_0 => "43.3.2.0",
            Self::Eric43_4_6_0 => "43.4.6.0",
        };

        write!(f, "{version}")
    }
}

impl FromStr for EricVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "38.1.6.0" => Ok(Self::Eric38_1_6_0),
            "39.6.4.0" => Ok(Self::Eric39_6_4_0),
            "40.1.8.0" => Ok(Self::Eric40_1_8_0),
            "40.2.10.0" => Ok(Self::Eric40_2_10_0),
            "43.3.2.0" => Ok(Self::Eric43_3_2_0),
            "43.4.6.0" => Ok(Self::Eric43_4_6_0),
            other => Err(format!(
                "Unsupported ERIC_VERSION={other:?}; \
                 add the corresponding bindings file and EricVersion variant"
            )),
        }
    }
}

pub fn main() -> io::Result<()> {
    #[cfg(feature = "generate-bindings")]
    generate_bindings()?;

    #[cfg(not(feature = "generate-bindings"))]
    select_bindings()?;

    Ok(())
}

/// Select existing bindings
#[cfg(not(feature = "generate-bindings"))]
fn select_bindings() -> io::Result<()> {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("environment variable `CARGO_CFG_TARGET_ARCH` not set");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS")
        .expect("environment variable `CARGO_CFG_TARGET_OS` not set");

    let out_dir = env::var("OUT_DIR").expect("environment variable `OUT_DIR` not set");
    let bindings_target = PathBuf::from(out_dir).join("bindings.rs");

    println!("cargo:rerun-if-env-changed=ERIC_VERSION");

    let eric_version = env::var("ERIC_VERSION")
        .unwrap_or_else(|_| {
            if env::var("DOCS_RS").is_ok() {
                EricVersion::LATEST.to_string()
            } else {
                panic!("environment variable `ERIC_VERSION` not set")
            }
        })
        .parse::<EricVersion>()
        .unwrap_or_else(|err| panic!("{err}"));

    println!(
        "Select bindings for Eric version {eric_version} and target {target_os}/{target_arch}"
    );

    let bindings_file = match (&eric_version, target_os.as_ref(), target_arch.as_ref()) {
        (EricVersion::Eric38_1_6_0, "linux", "x86_64") => "bindings_eric_38_1_6_0_linux_x86_64.rs",
        (EricVersion::Eric39_6_4_0, "linux", "x86_64") => "bindings_eric_39_6_4_0_linux_x86_64.rs",
        (EricVersion::Eric40_1_8_0, "linux", "x86_64") => "bindings_eric_40_1_8_0_linux_x86_64.rs",
        (EricVersion::Eric40_2_10_0, "linux", "x86_64") => {
            "bindings_eric_40_2_10_0_linux_x86_64.rs"
        }
        (EricVersion::Eric43_3_2_0, "linux", "x86_64") => "bindings_eric_43_3_2_0_linux_x86_64.rs",
        (EricVersion::Eric43_3_2_0, "macos", "aarch64") => {
            "bindings_eric_43_3_2_0_darwin_aarch64.rs"
        }
        (EricVersion::Eric43_4_6_0, "linux", "x86_64") => "bindings_eric_43_4_6_0_linux_x86_64.rs",
        (EricVersion::Eric43_4_6_0, "macos", "aarch64") => {
            "bindings_eric_43_4_6_0_darwin_aarch64.rs"
        }
        _ => {
            panic!("Missing bindings for Eric version {eric_version} and target {target_os}/{target_arch}");
        }
    };

    #[cfg(not(feature = "no-linking"))]
    {
        let eric_path = env::var("ERIC_PATH").expect("environment variable `ERIC_PATH` not set");
        emit_link_instructions(&eric_path);
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
    let eric_path_str = env::var("ERIC_PATH").expect("environment variable `ERIC_PATH` not set");
    let eric_path = Path::new(&eric_path_str);
    let header_file = get_header_file(eric_path);

    #[cfg(not(feature = "no-linking"))]
    emit_link_instructions(&eric_path_str);

    let header = header_file.to_str().expect("Can't convert path to string");

    let include_comments = env::var("CARGO_FEATURE_BINDGEN_COMMENTS").is_ok();

    let bindings = bindgen::Builder::default()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_comments(include_comments)
        .generate()
        .expect("Can't generate bindings");

    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_BINDGEN_COMMENTS");

    let out_dir = env::var("OUT_DIR").expect("environment variable `OUT_DIR` not set");
    let output_path = PathBuf::from(out_dir);

    bindings
        .write_to_file(output_path.join("bindings.rs"))
        .expect("Can't write bindings");

    Ok(())
}

/// Emit cargo link instructions for the ERiC shared library.
///
/// Note: `cargo:rustc-link-arg` from a library crate's build script does not
/// propagate to the final binary. Runtime library paths (rpath on macOS,
/// LD_LIBRARY_PATH on Linux) must be set by the binary crate's build script
/// or by the user's environment.
#[cfg(not(feature = "no-linking"))]
fn emit_link_instructions(eric_path_str: &str) {
    let eric_path = Path::new(eric_path_str);
    let library_name = get_library_name();
    let library_path = get_library_path(eric_path);
    let header_file = get_header_file(eric_path);

    println!("cargo:rustc-link-search={}", library_path.display());
    println!("cargo:rustc-link-lib={}", library_name);

    // Rerun build script if relevant environment variables change.
    println!("cargo:rerun-if-env-changed=ERIC_PATH");
    println!("cargo:rerun-if-env-changed=LIBRARY_NAME");
    println!("cargo:rerun-if-env-changed=LIBRARY_PATH");
    println!("cargo:rerun-if-env-changed=HEADER_FILE");

    // Rerun build script if file itself changes.
    println!("cargo:rerun-if-changed={}", header_file.display());
}

#[cfg(not(feature = "no-linking"))]
fn get_library_name() -> String {
    env::var("LIBRARY_NAME").unwrap_or_else(|_| "ericapi".to_owned())
}

#[cfg(not(feature = "no-linking"))]
fn get_library_path(eric_path: &Path) -> PathBuf {
    env::var("LIBRARY_PATH")
        .ok()
        .map(|path| PathBuf::from_str(&path).expect("invalid path for `LIBRARY_PATH`"))
        .unwrap_or_else(|| eric_path.join("lib"))
}

#[cfg(any(feature = "generate-bindings", not(feature = "no-linking")))]
fn get_header_file(eric_path: &Path) -> PathBuf {
    env::var("HEADER_FILE")
        .ok()
        .map(|path| PathBuf::from_str(&path).expect("invalid path for `HEADER_FILE`"))
        .unwrap_or_else(|| eric_path.join("include").join("ericapi.h"))
}
