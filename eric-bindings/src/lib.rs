#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
// TODO: extern block uses type u128, which is not FFI-safe
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use std::{
        env,
        ffi::CString,
        path::{Path, PathBuf},
        str::FromStr,
    };

    fn get_plugin_path(eric_path: &Path) -> PathBuf {
        env::var("PLUGIN_PATH")
            .ok()
            .map(|path| PathBuf::from_str(&path).expect("invalid path for `PLUGIN_PATH`"))
            .unwrap_or_else(|| eric_path.join("lib").join("plugins"))
    }

    #[test]
    fn test_ericapi() {
        let eric_path = env::var("ERIC_PATH").expect("Missing environment variable `ERIC_PATH`");
        let eric_path = Path::new(&eric_path);
        let plugin_path = get_plugin_path(eric_path);
        let plugin_path = CString::new(plugin_path.to_str().expect("empty path for `ERIC_PATH`"))
            .context("Can't convert to CString")
            .unwrap();

        let log_path =
            env::current_dir().expect("Missing environment variable for current directory");
        let log_path = CString::new(log_path.to_str().unwrap())
            .context("Can't convert to CString")
            .unwrap();

        let error_code = unsafe { EricInitialisiere(plugin_path.as_ptr(), log_path.as_ptr()) };
        assert_eq!(error_code, 0);

        let buffer = unsafe { EricRueckgabepufferErzeugen() };
        let error_code = unsafe { EricVersion(buffer) };
        assert_eq!(error_code, 0);

        let error_code = unsafe { EricBeende() };
        assert_eq!(error_code, 0);
    }
}
