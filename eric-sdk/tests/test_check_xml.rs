use anyhow::Context;
use eric_sdk::{Eric, ErrorCode};
use roxmltree::Document;
use std::{env::current_dir, fs, path::Path};

#[test]
#[cfg_attr(feature = "external-test", ignore)]
fn test_check_xml_valid() {
    let log_path = current_dir().unwrap();
    let xml_path = Path::new("./test_data/taxonomy/v6.5/tax_balance_sheet.xml");
    let xml = fs::read_to_string(xml_path)
        .context(format!("Can't read file: {}", xml_path.display()))
        .unwrap();

    let eric = Eric::new(Some(&log_path), None).unwrap();

    let res = eric.check_xml(xml, "Bilanz", "6.5");
    println!("{:#?}", res);

    match res {
        Ok(response) => {
            assert!(response.server_response().is_empty());

            let doc = Document::parse(response.validation_response()).unwrap();
            let node = doc.descendants().find(|node| node.has_tag_name("Erfolg"));
            assert!(node.is_some());
        }
        Err(err) => {
            let error_code = err.code().unwrap();
            // Depending on ERiC data-version support, schema check might not be
            // available.
            assert!(
                error_code == ErrorCode::ERIC_GLOBAL_FUNKTION_NICHT_UNTERSTUETZT as i32
                    || error_code == ErrorCode::ERIC_GLOBAL_DATENARTVERSION_UNBEKANNT as i32
            );
        }
    }
}

#[test]
#[cfg_attr(feature = "external-test", ignore)]
fn test_check_xml_invalid_xml() {
    let log_path = current_dir().unwrap();
    let xml = "<Invalid>XML</Invalid>".to_string();

    let eric = Eric::new(Some(&log_path), None).unwrap();

    let res = eric.check_xml(xml, "Bilanz", "6.5");
    assert!(res.is_err());

    let error_code = res.unwrap_err().code().unwrap();

    // Depending on ERiC data-version support, schema check might not be
    // available.
    assert!(
        error_code == ErrorCode::ERIC_IO_PARSE_FEHLER as i32
            || error_code == ErrorCode::ERIC_IO_READER_SCHEMA_VALIDIERUNGSFEHLER as i32
            || error_code == ErrorCode::ERIC_GLOBAL_FUNKTION_NICHT_UNTERSTUETZT as i32
    );
}
