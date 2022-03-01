# AclBinding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**resource_type** | [**crate::models::AclResourceType**](AclResourceType.md) |  | 
**resource_name** | **String** |  | 
**pattern_type** | [**crate::models::AclPatternType**](AclPatternType.md) |  | 
**principal** | **String** | Identifies the user or service account to which an ACL entry is bound. The literal prefix value of `User:` is required. May be used to specify all users with value `User:*`. | 
**operation** | [**crate::models::AclOperation**](AclOperation.md) |  | 
**permission** | [**crate::models::AclPermissionType**](AclPermissionType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


