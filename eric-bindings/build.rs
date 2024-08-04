use std::io;

fn main() -> io::Result<()> {
    #[cfg(not(feature = "no-build"))]
    {
        use std::{
            env,
            path::{Path, PathBuf},
        };

        let library_path =
            env::var("LIBRARY_PATH").expect("Missing environment variable 'LIBRARY_PATH'");
        let library_name =
            env::var("LIBRARY_NAME").expect("Missing environment variable 'LIBRARY_NAME'");
        let header_file =
            env::var("HEADER_FILE").expect("Missing environment variable 'HEADER_FILE'");

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
    }

    Ok(())
}
