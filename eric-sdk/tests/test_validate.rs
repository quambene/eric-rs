use anyhow::Context;
use eric_sdk::{Eric, ErrorCode};
use roxmltree::Document;
use std::{env::current_dir, fs, path::Path};

#[test]
#[cfg_attr(feature = "external-test", ignore)]
fn test_validate() {
    let log_path = current_dir().unwrap();
    let xml_path = Path::new("./test_data/taxonomy/v6.5/SteuerbilanzAutoverkaeufer_PersG.xml");
    let xml = fs::read_to_string(xml_path)
        .context(format!("Can't read file: {}", xml_path.display()))
        .unwrap();
    let taxonomy_type = "Bilanz";
    let taxonomy_version = "6.7";
    let pdf_path = None;

    let eric = Eric::new(&log_path).unwrap();

    let res = eric.validate(xml, taxonomy_type, taxonomy_version, pdf_path);
    println!("{:#?}", res);
    assert!(res.is_ok(), "{}", res.unwrap_err());

    let response = res.unwrap();
    assert_eq!(response.error_code, ErrorCode::ERIC_OK as i32);

    let doc = Document::parse(&response.validation_response).unwrap();
    println!("Doc: {:#?}", doc);
    let node = doc.descendants().find(|node| node.has_tag_name("Erfolg"));
    assert!(node.is_some());
    let node = node.unwrap();
    assert_eq!(node.tag_name().name(), "Erfolg");

    assert!(response.server_response.is_empty());
}

#[test]
#[cfg_attr(feature = "external-test", ignore)]
fn test_validate_and_print() {
    let log_path = current_dir().unwrap();
    let xml_path = Path::new("./test_data/taxonomy/v6.5/SteuerbilanzAutoverkaeufer_PersG.xml");
    let xml = fs::read_to_string(xml_path)
        .context(format!("Can't read file: {}", xml_path.display()))
        .unwrap();
    let taxonomy_type = "Bilanz";
    let taxonomy_version = "6.7";
    let pdf_path = "ebilanz.pdf";

    let eric = Eric::new(&log_path).unwrap();

    let res = eric.validate(xml, taxonomy_type, taxonomy_version, Some(pdf_path));
    println!("{:#?}", res);
    assert!(res.is_ok(), "{}", res.unwrap_err());

    let response = res.unwrap();
    assert_eq!(response.error_code, ErrorCode::ERIC_OK as i32);

    let doc = Document::parse(&response.validation_response).unwrap();
    println!("Doc: {:#?}", doc);
    let node = doc.descendants().find(|node| node.has_tag_name("Erfolg"));
    assert!(node.is_some());
    let node = node.unwrap();
    assert_eq!(node.tag_name().name(), "Erfolg");

    assert!(response.server_response.is_empty());
}

#[test]
#[cfg_attr(feature = "external-test", ignore)]
fn test_validate_invalid_xml() {
    let log_path = current_dir().unwrap();
    let xml = "<Invalid>XML</Invalid>".to_string();
    let taxonomy_type = "Bilanz";
    let taxonomy_version = "6.7";
    let pdf_path = None;

    let eric = Eric::new(&log_path).unwrap();

    let res = eric.validate(xml, taxonomy_type, taxonomy_version, pdf_path);
    // 610001002 (PRUEF_FEHLER) is a plausibility result, not a hard error — returned as Ok.
    // Other structural errors (e.g. 610301200) still surface as Err.
    match res {
        Ok(response) => {
            println!("Plausibility failure (Ok): error_code={}", response.error_code);
            println!("Validation response: {}", response.validation_response);
            assert_eq!(response.error_code, ErrorCode::ERIC_GLOBAL_PRUEF_FEHLER as i32);
        }
        Err(err) => {
            println!("Hard error: {}", err);
            // A non-plausibility structural error is also acceptable
            assert!(err.to_string().contains("610301200") || err.to_string().contains("610001"));
        }
    }
}
