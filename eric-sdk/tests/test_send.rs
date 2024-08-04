use anyhow::Context;
use eric_sdk::{Eric, ErrorCode};
use roxmltree::Document;
use std::{env::current_dir, fs, path::Path};

#[test]
#[cfg_attr(not(feature = "external-test"), ignore)]
fn test_send() {
    let log_path = current_dir().unwrap();
    let xml_path = Path::new("./test_data/taxonomy/v6.5/SteuerbilanzAutoverkaeufer_PersG.xml");
    let xml = fs::read_to_string(xml_path)
        .context(format!("Can't read file: {}", xml_path.display()))
        .unwrap();
    let taxonomy_type = "Bilanz";
    let taxonomy_version = "6.5";
    let pdf_name = None;

    let eric = Eric::new(&log_path).unwrap();

    let res = eric.send(xml, taxonomy_type, taxonomy_version, pdf_name);
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
}

#[test]
#[cfg_attr(not(feature = "external-test"), ignore)]
fn test_send_and_print() {
    let log_path = current_dir().unwrap();
    let xml_path = Path::new("./test_data/taxonomy/v6.5/SteuerbilanzAutoverkaeufer_PersG.xml");
    let xml = fs::read_to_string(xml_path)
        .context(format!("Can't read file: {}", xml_path.display()))
        .unwrap();
    let taxonomy_type = "Bilanz";
    let taxonomy_version = "6.5";
    let pdf_name = "ebilanz.pdf";

    let eric = Eric::new(&log_path).unwrap();

    let res = eric.send(xml, taxonomy_type, taxonomy_version, Some(pdf_name));
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
}
