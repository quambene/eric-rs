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
    use std::{env, ffi::CString};
    use anyhow::Context;

    #[test]
    fn test_ericapi() {
        let plugin_path = env::var("PLUGIN_PATH")
            .expect("Missing environment variable 'PLUGIN_PATH'");
        let plugin_path = CString::new(plugin_path).context("Can't convert to CString")
            .unwrap();

        let log_path = env::current_dir()
            .unwrap();
        let log_path = CString::new(log_path.to_str().unwrap()).context("Can't convert to CString")
        .unwrap();
    
        unsafe {
            let error_code = EricInitialisiere(plugin_path.as_ptr(), log_path.as_ptr());
            assert_eq!(error_code, 0);

            let buffer = EricRueckgabepufferErzeugen();
            let error_code = EricVersion(buffer);
            assert_eq!(error_code, 0);

            let error_code = EricBeende();
            assert_eq!(error_code, 0);
        }
    }
}