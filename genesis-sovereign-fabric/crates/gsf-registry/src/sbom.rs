//! SBOM generation — supply chain verification

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SbomComponent {
    pub name: String,
    pub version: String,
    pub sha256: String,
}

#[derive(Debug, Serialize)]
pub struct Sbom {
    pub format: String,
    pub components: Vec<SbomComponent>,
}

pub struct SbomGenerator;

impl SbomGenerator {
    pub fn generate(components: Vec<(String, String, String)>) -> Sbom {
        Sbom {
            format: "CycloneDX/1.4".to_string(),
            components: components
                .into_iter()
                .map(|(name, version, sha256)| SbomComponent {
                    name,
                    version,
                    sha256,
                })
                .collect(),
        }
    }

    pub fn to_json(sbom: &Sbom) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(sbom)
    }
}
