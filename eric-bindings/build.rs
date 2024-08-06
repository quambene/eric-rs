use std::{
    env, fmt, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum EricVersion {
    Eric38_1_6_0,
    Eric39_6_4_0,
    Eric40_1_8_0,
    Eric40_2_10_0,
}

impl fmt::Display for EricVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let request_method = match self {
            Self::Eric38_1_6_0 => "38.1.6.0",
            Self::Eric39_6_4_0 => "39.6.4.0",
            Self::Eric40_1_8_0 => "40.1.8.0",
            Self::Eric40_2_10_0 => "40.2.10.0",
        };

        write!(f, "{}", request_method)
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
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("Set by cargo");
    let is_windows = std::env::var("CARGO_CFG_WINDOWS").is_ok();
    let library_name =
        env::var("LIBRARY_NAME").expect("Missing environment variable 'LIBRARY_NAME'");
    let library_path =
        env::var("LIBRARY_PATH").expect("Missing environment variable 'LIBRARY_PATH'");
    let header_file = env::var("HEADER_FILE").expect("Missing environment variable 'HEADER_FILE'");
    let out_dir = env::var("OUT_DIR").expect("Can't read environment variable 'OUT_DIR'");
    let bindings_target = PathBuf::from(out_dir).join("bindings.rs");

    println!("cargo:rustc-link-search={}", library_path);
    println!("cargo:rustc-link-lib={}", library_name);
    println!("cargo:rerun-if-changed={}", header_file);
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", library_path);

    let eric_version = if library_path.contains("38.1.6.0") {
        EricVersion::Eric38_1_6_0
    } else if library_path.contains("39.6.4.0") {
        EricVersion::Eric39_6_4_0
    } else if library_path.contains("40.1.8.0") {
        EricVersion::Eric40_1_8_0
    } else if library_path.contains("40.2.10.0") {
        EricVersion::Eric40_2_10_0
    } else {
        panic!("Missing bindings: Unknown Eric version");
    };
    let bindings_file = match (&eric_version, target_arch.as_ref(), is_windows) {
        (EricVersion::Eric38_1_6_0, "x86_64", false) => "bindings_eric_38_1_6_0_linux_x86_64.rs",
        (EricVersion::Eric39_6_4_0, "x86_64", false) => "bindings_eric_39_6_4_0_linux_x86_64.rs",
        (EricVersion::Eric40_1_8_0, "x86_64", false) => "bindings_eric_40_1_8_0_linux_x86_64.rs",
        (EricVersion::Eric40_2_10_0, "x86_64", false) => "bindings_eric_40_2_10_0_linux_x86_64.rs",
        _ => {
            panic!("Missing bindings for Eric version {eric_version} and target {target_arch}");
        }
    };

    println!("Select bindings for Eric version {eric_version} and target {target_arch}");

    let root_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Set by cargo");
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
    let library_name =
        env::var("LIBRARY_NAME").expect("Missing environment variable 'LIBRARY_NAME'");
    let library_path =
        env::var("LIBRARY_PATH").expect("Missing environment variable 'LIBRARY_PATH'");
    let header_file = env::var("HEADER_FILE").expect("Missing environment variable 'HEADER_FILE'");

    let library_path = Path::new(&library_path);
    let header_file = Path::new(&header_file);

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

    let out_dir = env::var("OUT_DIR").expect("Can't read environment variable 'OUT_DIR'");
    let output_path = PathBuf::from(out_dir);

    bindings
        .write_to_file(output_path.join("bindings.rs"))
        .expect("Can't write bindings");

    Ok(())
}
