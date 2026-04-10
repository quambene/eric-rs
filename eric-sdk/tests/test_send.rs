use anyhow::Context;
use eric_sdk::{Eric, ErrorCode};
use roxmltree::Document;
use std::{
    env::{self, current_dir},
    fs,
    path::Path,
};

fn require_test_env() -> Result<(), anyhow::Error> {
    env::var("CERTIFICATE_PATH").context("CERTIFICATE_PATH is required for external test")?;
    env::var("CERTIFICATE_PASSWORD")
        .context("CERTIFICATE_PASSWORD is required for external test")?;
    Ok(())
}

fn get_xml_with_vendor_id() -> Result<String, anyhow::Error> {
    let xml_path = Path::new("test_data/taxonomy/v6.5/SteuerbilanzAutoverkaeufer_PersG.xml");
    let xml =
        fs::read_to_string(xml_path).context(format!("Can't read file: {}", xml_path.display()))?;

    let vendor_id = env::var("VENDOR_ID").context("VENDOR_ID is required for external test")?;
    Ok(xml.replace(
        "<HerstellerID>00000</HerstellerID>",
        &format!("<HerstellerID>{}</HerstellerID>", vendor_id),
    ))
}

#[test]
#[cfg_attr(not(feature = "external-test"), ignore)]
fn test_send() {
    require_test_env().unwrap();

    let log_path = current_dir().unwrap();
    let xml = get_xml_with_vendor_id().unwrap();
    let taxonomy_type = "Bilanz";
    let taxonomy_version = "6.7";
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
#[cfg_attr(not(feature = "external-test"), ignore)]
fn test_send_and_print() {
    require_test_env().unwrap();

    let log_path = current_dir().unwrap();
    let xml = get_xml_with_vendor_id().unwrap();
    let taxonomy_type = "Bilanz";
    let taxonomy_version = "6.7";
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
