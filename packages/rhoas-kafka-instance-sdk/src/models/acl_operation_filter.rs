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
pub enum AclOperationFilter {
    #[serde(rename = "ALL")]
    ALL,
    #[serde(rename = "READ")]
    READ,
    #[serde(rename = "WRITE")]
    WRITE,
    #[serde(rename = "CREATE")]
    CREATE,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "ALTER")]
    ALTER,
    #[serde(rename = "DESCRIBE")]
    DESCRIBE,
    #[serde(rename = "DESCRIBE_CONFIGS")]
    DESCRIBECONFIGS,
    #[serde(rename = "ALTER_CONFIGS")]
    ALTERCONFIGS,
    #[serde(rename = "ANY")]
    ANY,

}

impl ToString for AclOperationFilter {
    fn to_string(&self) -> String {
        match self {
            Self::ALL => String::from("ALL"),
            Self::READ => String::from("READ"),
            Self::WRITE => String::from("WRITE"),
            Self::CREATE => String::from("CREATE"),
            Self::DELETE => String::from("DELETE"),
            Self::ALTER => String::from("ALTER"),
            Self::DESCRIBE => String::from("DESCRIBE"),
            Self::DESCRIBECONFIGS => String::from("DESCRIBE_CONFIGS"),
            Self::ALTERCONFIGS => String::from("ALTER_CONFIGS"),
            Self::ANY => String::from("ANY"),
        }
    }
}

impl Default for AclOperationFilter {
    fn default() -> AclOperationFilter {
        Self::ALL
    }
}




