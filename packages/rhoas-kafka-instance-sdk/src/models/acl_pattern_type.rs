/*
 * Kafka Admin REST API
 *
 * An API to provide REST endpoints for query Kafka for admin operations
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AclPatternType {
    #[serde(rename = "LITERAL")]
    LITERAL,
    #[serde(rename = "PREFIXED")]
    PREFIXED,

}

impl ToString for AclPatternType {
    fn to_string(&self) -> String {
        match self {
            Self::LITERAL => String::from("LITERAL"),
            Self::PREFIXED => String::from("PREFIXED"),
        }
    }
}

impl Default for AclPatternType {
    fn default() -> AclPatternType {
        Self::LITERAL
    }
}




