# \AclsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_acl**](AclsApi.md#create_acl) | **POST** /acls | Create ACL binding
[**delete_acls**](AclsApi.md#delete_acls) | **DELETE** /acls | Delete ACL bindings
[**get_acl_resource_operations**](AclsApi.md#get_acl_resource_operations) | **GET** /acls/resource-operations | Retrieve allowed ACL resources and operations
[**get_acls**](AclsApi.md#get_acls) | **GET** /acls | List ACL bindings



## create_acl

> create_acl(acl_binding)
Create ACL binding

Creates a new ACL binding for a Kafka instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_binding** | [**AclBinding**](AclBinding.md) | ACL to create. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_acls

> crate::models::AclBindingListPage delete_acls(resource_type, resource_name, pattern_type, principal, operation, permission)
Delete ACL bindings

Deletes ACL bindings that match the query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_type** | Option<[**crate::models::AclResourceTypeFilter**](.md)> | ACL Resource Type Filter |  |
**resource_name** | Option<**String**> | ACL Resource Name Filter |  |
**pattern_type** | Option<[**crate::models::AclPatternTypeFilter**](.md)> | ACL Pattern Type Filter |  |
**principal** | Option<**String**> | ACL Principal Filter. Either a specific user or the wildcard user `User:*` may be provided. - When fetching by a specific user, the results will also include ACL bindings that apply to all users. - When deleting, ACL bindings to be delete must match the provided `principal` exactly. |  |
**operation** | Option<[**crate::models::AclOperationFilter**](.md)> | ACL Operation Filter. The ACL binding operation provided should be valid for the resource type in the request, if not `ANY`. |  |
**permission** | Option<[**crate::models::AclPermissionTypeFilter**](.md)> | ACL Permission Type Filter |  |

### Return type

[**crate::models::AclBindingListPage**](AclBindingListPage.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_acl_resource_operations

> ::std::collections::HashMap<String, Vec<String>> get_acl_resource_operations()
Retrieve allowed ACL resources and operations

Retrieve the resources and associated operations that may have ACLs configured.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, Vec<String>>**](array.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_acls

> crate::models::AclBindingListPage get_acls(resource_type, resource_name, pattern_type, principal, operation, permission, page, size, order, order_key)
List ACL bindings

Returns a list of all of the available ACL bindings, or the list of bindings that meet the users URL Query Parameters. If no parameters are specified, all ACL bindings known to the system will be returned (with paging).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_type** | Option<[**crate::models::AclResourceTypeFilter**](.md)> | ACL Resource Type Filter |  |
**resource_name** | Option<**String**> | ACL Resource Name Filter |  |
**pattern_type** | Option<[**crate::models::AclPatternTypeFilter**](.md)> | ACL Pattern Type Filter |  |
**principal** | Option<**String**> | ACL Principal Filter. Either a specific user or the wildcard user `User:*` may be provided. - When fetching by a specific user, the results will also include ACL bindings that apply to all users. - When deleting, ACL bindings to be delete must match the provided `principal` exactly. |  |
**operation** | Option<[**crate::models::AclOperationFilter**](.md)> | ACL Operation Filter. The ACL binding operation provided should be valid for the resource type in the request, if not `ANY`. |  |
**permission** | Option<[**crate::models::AclPermissionTypeFilter**](.md)> | ACL Permission Type Filter |  |
**page** | Option<**f32**> | Page number for result lists |  |[default to 1]
**size** | Option<**f32**> | Page size for result lists |  |[default to 10]
**order** | Option<**String**> | Order of the ACL binding sorting. |  |[default to desc]
**order_key** | Option<**String**> | Order key to sort the items by. |  |[default to permission]

### Return type

[**crate::models::AclBindingListPage**](AclBindingListPage.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

