use crate::EricError;
use anyhow::anyhow;
use quick_xml::de::from_str;
use serde::Deserialize;

/// One `<FehlerRegelpruefung>` entry from ERiC validation XML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub nutzdatenticket: Option<String>,
    pub mehrfachzeilenindex: Option<u32>,
    pub lfd_nr_vordruck: Option<u32>,
    pub fachliche_fehler_id: Option<String>,
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
                nutzdatenticket: issue.nutzdatenticket,
                mehrfachzeilenindex: issue.mehrfachzeilenindex,
                lfd_nr_vordruck: issue.lfd_nr_vordruck,
                fachliche_fehler_id: issue.fachliche_fehler_id,
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
    nutzdatenticket: Option<String>,
    #[serde(rename = "Mehrfachzeilenindex")]
    mehrfachzeilenindex: Option<u32>,
    #[serde(rename = "LfdNrVordruck")]
    lfd_nr_vordruck: Option<u32>,
    #[serde(rename = "FachlicheFehlerId")]
    fachliche_fehler_id: Option<String>,
    #[serde(rename = "Text")]
    text: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::EricError;

    #[test]
    fn parse_validation_report_from_api_error() {
        let err = EricError::ApiError {
            code: 610001002,
            message: "Fehler waehrend der Plausibilitaetspruefung".to_string(),
            validation_response: r#"<?xml version="1.0" encoding="UTF-8"?>
<EricBearbeiteVorgang xmlns="http://www.elster.de/EricXML/1.1/EricBearbeiteVorgang">
  <FehlerRegelpruefung>
    <Nutzdatenticket>-</Nutzdatenticket>
    <Mehrfachzeilenindex>1</Mehrfachzeilenindex>
    <LfdNrVordruck>1</LfdNrVordruck>
    <FachlicheFehlerId>170105000</FachlicheFehlerId>
    <Text>missing required attribute 'unitRef'</Text>
  </FehlerRegelpruefung>
</EricBearbeiteVorgang>"#
                .to_string(),
            server_response: String::new(),
        };

        let report = err
            .validation_report()
            .expect("report should parse")
            .unwrap();
        assert_eq!(report.issues.len(), 1);

        let issue = &report.issues[0];
        assert_eq!(issue.mehrfachzeilenindex, Some(1));
        assert_eq!(issue.lfd_nr_vordruck, Some(1));
        assert_eq!(issue.fachliche_fehler_id.as_deref(), Some("170105000"));
        assert!(issue
            .text
            .as_deref()
            .unwrap_or_default()
            .contains("unitRef"));
    }
}
