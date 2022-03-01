/*
 * Service Registry Fleet Manager
 *
 * Managed Service Registry cloud.redhat.com API Management API that lets you create new registry instances. Registry is a datastore for standard event schemas and API designs. Service Registry enables developers to manage and share the structure of their data using a REST interface. For example, client applications can dynamically push or pull the latest updates to or from the registry without needing to redeploy. Registry is an Managed version of upstream project called Apicurio Registry. Apicurio Registry also enables developers to create rules that govern how registry content can evolve over time. For example, this includes rules for content validation and version compatibility.
 *
 * The version of the OpenAPI document: 0.0.6
 * Contact: rhosak-eval-support@redhat.com
 * Generated by: https://openapi-generator.tech
 */

/// RegistryCreate : Information used to create a new Service Registry instance within a multi-tenant deployment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegistryCreate {
    /// User-defined Registry name. Required. Does not have to be unique.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-provided description of the new Registry instance. Not required.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl RegistryCreate {
    /// Information used to create a new Service Registry instance within a multi-tenant deployment.
    pub fn new() -> RegistryCreate {
        RegistryCreate {
            name: None,
            description: None,
        }
    }
}


