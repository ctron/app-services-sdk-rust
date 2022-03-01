# RootTypeForRegistry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**status** | [**crate::models::RegistryStatusValue**](RegistryStatusValue.md) |  | 
**registry_url** | Option<**String**> |  | [optional]
**browser_url** | Option<**String**> |  | [optional]
**name** | Option<**String**> | User-defined Registry name. Does not have to be unique. | [optional]
**registry_deployment_id** | Option<**i32**> | Identifier of a multi-tenant deployment, where this Service Registry instance resides. | [optional]
**owner** | Option<**String**> | Registry instance owner | [optional]
**description** | Option<**String**> | Description of the Registry instance. | [optional]
**created_at** | **String** | ISO 8601 UTC timestamp. | 
**updated_at** | **String** | ISO 8601 UTC timestamp. | 
**instance_type** | [**crate::models::RegistryInstanceTypeValue**](RegistryInstanceTypeValue.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


