/*
 * Service Registry Fleet Manager
 *
 * Managed Service Registry cloud.redhat.com API Management API that lets you create new registry instances. Registry is a datastore for standard event schemas and API designs. Service Registry enables developers to manage and share the structure of their data using a REST interface. For example, client applications can dynamically push or pull the latest updates to or from the registry without needing to redeploy. Registry is an Managed version of upstream project called Apicurio Registry. Apicurio Registry also enables developers to create rules that govern how registry content can evolve over time. For example, this includes rules for content validation and version compatibility.
 *
 * The version of the OpenAPI document: 0.0.6
 * Contact: rhosak-eval-support@redhat.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Registry {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "status")]
    pub status: crate::models::RegistryStatusValue,
    #[serde(rename = "registryUrl", skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
    #[serde(rename = "browserUrl", skip_serializing_if = "Option::is_none")]
    pub browser_url: Option<String>,
    /// User-defined Registry name. Does not have to be unique.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier of a multi-tenant deployment, where this Service Registry instance resides.
    #[serde(rename = "registryDeploymentId", skip_serializing_if = "Option::is_none")]
    pub registry_deployment_id: Option<i32>,
    /// Registry instance owner
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Description of the Registry instance.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ISO 8601 UTC timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// ISO 8601 UTC timestamp.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "instance_type")]
    pub instance_type: crate::models::RegistryInstanceTypeValue,
}

impl Registry {
    pub fn new(id: String, status: crate::models::RegistryStatusValue, created_at: String, updated_at: String, instance_type: crate::models::RegistryInstanceTypeValue) -> Registry {
        Registry {
            id,
            kind: None,
            href: None,
            status,
            registry_url: None,
            browser_url: None,
            name: None,
            registry_deployment_id: None,
            owner: None,
            description: None,
            created_at,
            updated_at,
            instance_type,
        }
    }
}


