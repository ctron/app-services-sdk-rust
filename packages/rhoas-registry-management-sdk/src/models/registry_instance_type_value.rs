/*
 * Service Registry Fleet Manager
 *
 * Managed Service Registry cloud.redhat.com API Management API that lets you create new registry instances. Registry is a datastore for standard event schemas and API designs. Service Registry enables developers to manage and share the structure of their data using a REST interface. For example, client applications can dynamically push or pull the latest updates to or from the registry without needing to redeploy. Registry is an Managed version of upstream project called Apicurio Registry. Apicurio Registry also enables developers to create rules that govern how registry content can evolve over time. For example, this includes rules for content validation and version compatibility.
 *
 * The version of the OpenAPI document: 0.0.6
 * Contact: rhosak-eval-support@redhat.com
 * Generated by: https://openapi-generator.tech
 */

/// RegistryInstanceTypeValue : \"standard\": Standard, full-featured Registry instance  \"eval\": Evaluation (Trial) instance, provided for a limited time 

/// \"standard\": Standard, full-featured Registry instance  \"eval\": Evaluation (Trial) instance, provided for a limited time 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegistryInstanceTypeValue {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "eval")]
    Eval,

}

impl ToString for RegistryInstanceTypeValue {
    fn to_string(&self) -> String {
        match self {
            Self::Standard => String::from("standard"),
            Self::Eval => String::from("eval"),
        }
    }
}

impl Default for RegistryInstanceTypeValue {
    fn default() -> RegistryInstanceTypeValue {
        Self::Standard
    }
}



