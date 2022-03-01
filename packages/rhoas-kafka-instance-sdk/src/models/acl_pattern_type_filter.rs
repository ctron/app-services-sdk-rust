/*
 * Kafka Admin REST API
 *
 * An API to provide REST endpoints for query Kafka for admin operations
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AclPatternTypeFilter : Use value 'MATCH' to perform pattern matching.

/// Use value 'MATCH' to perform pattern matching.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AclPatternTypeFilter {
    #[serde(rename = "LITERAL")]
    LITERAL,
    #[serde(rename = "PREFIXED")]
    PREFIXED,
    #[serde(rename = "ANY")]
    ANY,
    #[serde(rename = "MATCH")]
    _MATCH,

}

impl ToString for AclPatternTypeFilter {
    fn to_string(&self) -> String {
        match self {
            Self::LITERAL => String::from("LITERAL"),
            Self::PREFIXED => String::from("PREFIXED"),
            Self::ANY => String::from("ANY"),
            Self::_MATCH => String::from("MATCH"),
        }
    }
}

impl Default for AclPatternTypeFilter {
    fn default() -> AclPatternTypeFilter {
        Self::LITERAL
    }
}



