use anyhow::Context;
use eric_sdk::{Eric, ErrorCode};
use roxmltree::Document;
use std::{env::{self, current_dir}, fs, path::Path};

fn setup_test_env() -> bool {
    let mut cert_path = current_dir().unwrap();
    // When running from eric-sdk
    if cert_path.ends_with("eric-sdk") {
        cert_path.push("../vendor/Test_Zertifikate/test-softidnr-pse.pfx");
    } else {
        cert_path.push("vendor/Test_Zertifikate/test-softidnr-pse.pfx");
    }
    
    if !cert_path.exists() {
        println!("WARNING: Test certificate not found at {:?}", cert_path);
        println!("To run transmission tests:");
        println!("1. Download 'ERiC-Testzertifikate' from the ELSTER Developer Portal.");
        println!("2. Place them in 'vendor/Test_Zertifikate/'.");
        println!("3. Configure your HERSTELLER_ID in a .env file.");
        return false;
    }

    let cert_path = cert_path.canonicalize().unwrap();
    env::set_var("CERTIFICATE_PATH", cert_path.to_str().unwrap());
    env::set_var("CERTIFICATE_PASSWORD", "123456");
    
    if env::var("HERSTELLER_ID").is_err() {
        println!("WARNING: HERSTELLER_ID not set. Using fallback 00000 (might fail server validation).");
    }

    true
}

fn get_xml_with_hersteller_id() -> String {
    let xml_path = Path::new("test_data/taxonomy/v6.5/SteuerbilanzAutoverkaeufer_PersG.xml");
    let xml = fs::read_to_string(xml_path)
        .context(format!("Can't read file: {}", xml_path.display()))
        .unwrap();
    
    let hersteller_id = env::var("HERSTELLER_ID").unwrap_or_else(|_| "00000".to_string());
    xml.replace("<HerstellerID>00000</HerstellerID>", &format!("<HerstellerID>{}</HerstellerID>", hersteller_id))
}

#[test]
fn test_send() {
    if !setup_test_env() {
        return;
    }
    
    let log_path = current_dir().unwrap();
    let xml = get_xml_with_hersteller_id();
    let taxonomy_type = "Bilanz";
    let taxonomy_version = "6.5";
    let pdf_path = None;

    let eric = Eric::new(&log_path).unwrap();

    let res = eric.send(xml, taxonomy_type, taxonomy_version, pdf_path);
    assert!(res.is_ok(), "{}", res.unwrap_err());

    let response = res.unwrap();
    assert_eq!(response.error_code, ErrorCode::ERIC_OK as i32);

    let doc = Document::parse(&response.validation_response).unwrap();
    let node = doc.descendants().find(|node| node.has_tag_name("Erfolg"));
    assert!(node.is_some());
}

#[test]
fn test_send_and_print() {
    if !setup_test_env() {
        return;
    }
    
    let log_path = current_dir().unwrap();
    let xml = get_xml_with_hersteller_id();
    let taxonomy_type = "Bilanz";
    let taxonomy_version = "6.5";
    let pdf_path = "ebilanz_send.pdf";

    let eric = Eric::new(&log_path).unwrap();

    let res = eric.send(xml, taxonomy_type, taxonomy_version, Some(pdf_path));
    assert!(res.is_ok(), "{}", res.unwrap_err());

    let response = res.unwrap();
    assert_eq!(response.error_code, ErrorCode::ERIC_OK as i32);

    let doc = Document::parse(&response.validation_response).unwrap();
    let node = doc.descendants().find(|node| node.has_tag_name("Erfolg"));
    assert!(node.is_some());
}
