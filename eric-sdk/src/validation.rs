use crate::EricError;
use anyhow::anyhow;
use quick_xml::de::from_str;
use serde::Deserialize;

/// One `<FehlerRegelpruefung>` entry from ERiC validation XML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub data_ticket: Option<String>,
    pub field_identifier: Option<String>,
    pub multi_line_index: Option<u32>,
    pub form_sequence_number: Option<u32>,
    pub error_code: Option<String>,
    pub text: Option<String>,
}

/// Parsed summary of ERiC validation XML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// Structured validation issues extracted from the XML.
    pub issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    pub fn parse(xml: &str) -> Result<Self, EricError> {
        let xml_report = from_str::<XmlValidationReport>(xml).map_err(|err| anyhow!("{err}"))?;

        let issues = xml_report
            .issues
            .into_iter()
            .map(|issue| ValidationIssue {
                data_ticket: issue.data_ticket,
                field_identifier: issue.field_identifier,
                multi_line_index: issue.multi_line_index,
                form_sequence_number: issue.form_sequence_number,
                error_code: issue.error_code,
                text: issue.text,
            })
            .collect();

        Ok(Self { issues })
    }
}

/// The raw xml response from ERiC may contain multiple `<FehlerRegelpruefung>`
/// entries, each describing a validation issue.
#[derive(Debug, Deserialize)]
struct XmlValidationReport {
    #[serde(rename = "FehlerRegelpruefung", default)]
    issues: Vec<XmlValidationIssue>,
}

/// An individual `<FehlerRegelpruefung>` entry from the ERiC validation XML.
#[derive(Debug, Deserialize)]
struct XmlValidationIssue {
    #[serde(rename = "Nutzdatenticket")]
    data_ticket: Option<String>,
    #[serde(rename = "Feldidentifikator")]
    field_identifier: Option<String>,
    #[serde(rename = "Mehrfachzeilenindex")]
    multi_line_index: Option<u32>,
    #[serde(rename = "LfdNrVordruck")]
    form_sequence_number: Option<u32>,
    #[serde(rename = "FachlicheFehlerId")]
    error_code: Option<String>,
    #[serde(rename = "Text")]
    text: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::{EricApiPayload, EricError};

    #[test]
    fn parse_validation_report_from_api_error() {
        let err = EricError::ApiError {
            code: 610001002,
            message: "Fehler waehrend der Plausibilitaetspruefung".to_string(),
            payload: EricApiPayload::new(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<EricBearbeiteVorgang xmlns="http://www.elster.de/EricXML/1.1/EricBearbeiteVorgang">
  <FehlerRegelpruefung>
    <Nutzdatenticket>-</Nutzdatenticket>
        <Feldidentifikator>gcd:genInfo.report.id.accountingStandard</Feldidentifikator>
    <Mehrfachzeilenindex>1</Mehrfachzeilenindex>
    <LfdNrVordruck>1</LfdNrVordruck>
    <FachlicheFehlerId>170105000</FachlicheFehlerId>
    <Text>missing required attribute 'unitRef'</Text>
  </FehlerRegelpruefung>
</EricBearbeiteVorgang>"#
                    .to_string(),
                String::new(),
            ),
        };

        let report = err
            .validation_report()
            .expect("report should parse")
            .unwrap();
        assert_eq!(report.issues.len(), 1);

        let issue = &report.issues[0];
        assert_eq!(
            issue.field_identifier.as_deref(),
            Some("gcd:genInfo.report.id.accountingStandard")
        );
        assert_eq!(issue.multi_line_index, Some(1));
        assert_eq!(issue.form_sequence_number, Some(1));
        assert_eq!(issue.error_code.as_deref(), Some("170105000"));
        assert!(issue
            .text
            .as_deref()
            .unwrap_or_default()
            .contains("unitRef"));
    }
}
