/*
 * Service Registry Fleet Manager
 *
 * Managed Service Registry cloud.redhat.com API Management API that lets you create new registry instances. Registry is a datastore for standard event schemas and API designs. Service Registry enables developers to manage and share the structure of their data using a REST interface. For example, client applications can dynamically push or pull the latest updates to or from the registry without needing to redeploy. Registry is an Managed version of upstream project called Apicurio Registry. Apicurio Registry also enables developers to create rules that govern how registry content can evolve over time. For example, this includes rules for content validation and version compatibility.
 *
 * The version of the OpenAPI document: 0.0.6
 * Contact: rhosak-eval-support@redhat.com
 * Generated by: https://openapi-generator.tech
 */

/// RegistryStatusValue : \"accepted\": Registry status when accepted for processing.  \"provisioning\": Registry status when provisioning a new instance.  \"ready\": Registry status when ready for use.  \"failed\": Registry status when the provisioning failed. When removing a Registry in this state, the status transitions directly to \"deleting\".   \"deprovision\": Registry status when accepted for deprovisioning.  \"deleting\": Registry status when deprovisioning. 

/// \"accepted\": Registry status when accepted for processing.  \"provisioning\": Registry status when provisioning a new instance.  \"ready\": Registry status when ready for use.  \"failed\": Registry status when the provisioning failed. When removing a Registry in this state, the status transitions directly to \"deleting\".   \"deprovision\": Registry status when accepted for deprovisioning.  \"deleting\": Registry status when deprovisioning. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegistryStatusValue {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "provisioning")]
    Provisioning,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "deprovision")]
    Deprovision,
    #[serde(rename = "deleting")]
    Deleting,

}

impl ToString for RegistryStatusValue {
    fn to_string(&self) -> String {
        match self {
            Self::Accepted => String::from("accepted"),
            Self::Provisioning => String::from("provisioning"),
            Self::Ready => String::from("ready"),
            Self::Failed => String::from("failed"),
            Self::Deprovision => String::from("deprovision"),
            Self::Deleting => String::from("deleting"),
        }
    }
}

impl Default for RegistryStatusValue {
    fn default() -> RegistryStatusValue {
        Self::Accepted
    }
}



