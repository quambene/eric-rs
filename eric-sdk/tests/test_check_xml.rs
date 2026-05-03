use anyhow::Context;
use eric_sdk::{Eric, EricError, ErrorCode};
use roxmltree::Document;
use std::{env::current_dir, fs, path::Path};

#[test]
#[cfg_attr(feature = "external-test", ignore)]
fn test_check_xml_valid() {
    let log_path = current_dir().unwrap();
    let xml_path = Path::new("./test_data/taxonomy/v6.5/SteuerbilanzAutoverkaeufer_PersG.xml");
    let xml = fs::read_to_string(xml_path)
        .context(format!("Can't read file: {}", xml_path.display()))
        .unwrap();

    let eric = Eric::new(Some(&log_path), None).unwrap();

    let res = eric.check_xml(xml, "Bilanz", "6.5");
    println!("{:#?}", res);

    match res {
        Ok(response) => {
            assert_eq!(response.error_code, ErrorCode::ERIC_OK as i32);
            assert!(response.server_response.is_empty());

            let doc = Document::parse(&response.validation_response).unwrap();
            let node = doc.descendants().find(|node| node.has_tag_name("Erfolg"));
            assert!(node.is_some());
        }
        Err(err) => {
            let err = err.to_string();
            // Depending on ERiC data-version support, schema check might not be available.
            assert!(
                err.contains(
                    &(ErrorCode::ERIC_GLOBAL_FUNKTION_NICHT_UNTERSTUETZT as i32).to_string()
                ) || err.contains(
                    &(ErrorCode::ERIC_GLOBAL_DATENARTVERSION_UNBEKANNT as i32).to_string()
                )
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

    let err = res.unwrap_err().to_string();
    println!("Caught expected error: {}", err);

    assert!(
        err.contains(&(ErrorCode::ERIC_IO_PARSE_FEHLER as i32).to_string())
            || err.contains(
                &(ErrorCode::ERIC_IO_READER_SCHEMA_VALIDIERUNGSFEHLER as i32).to_string()
            )
            || err
                .contains(&(ErrorCode::ERIC_GLOBAL_FUNKTION_NICHT_UNTERSTUETZT as i32).to_string())
    );
}
