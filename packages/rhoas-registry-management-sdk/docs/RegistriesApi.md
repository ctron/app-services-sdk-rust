# \RegistriesApi

All URIs are relative to *https://api.openshift.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_registry**](RegistriesApi.md#create_registry) | **POST** /api/serviceregistry_mgmt/v1/registries | Create a new Registry instance
[**delete_registry**](RegistriesApi.md#delete_registry) | **DELETE** /api/serviceregistry_mgmt/v1/registries/{id} | Delete a Registry
[**get_registries**](RegistriesApi.md#get_registries) | **GET** /api/serviceregistry_mgmt/v1/registries | Get the list of all registries.
[**get_registry**](RegistriesApi.md#get_registry) | **GET** /api/serviceregistry_mgmt/v1/registries/{id} | Get a Registry



## create_registry

> crate::models::Registry create_registry(registry_create)
Create a new Registry instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_create** | [**RegistryCreate**](RegistryCreate.md) | A new `Registry` to be created. | [required] |

### Return type

[**crate::models::Registry**](Registry.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_registry

> delete_registry(id)
Delete a Registry

Deletes an existing `Registry`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique identifier for a `Registry`. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_registries

> crate::models::RegistryList get_registries(page, size, order_by, search)
Get the list of all registries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page index |  |
**size** | Option<**i32**> | Number of items in each page |  |
**order_by** | Option<**String**> | Specifies the order by criteria. The syntax of this parameter is similar to the syntax of the _order by_ clause of an SQL statement. Each query can be ordered by any of the kafkaRequests fields. For example, in order to retrieve all kafkas ordered by their name:  ```sql name asc ```  Or in order to retrieve all kafkas ordered by their name _and_ created date:  ```sql name asc, created_at asc ```  If the parameter isn't provided, or if the value is empty, then the results will be ordered by name. |  |
**search** | Option<**String**> | Search criteria.  The syntax of this parameter is similar to the syntax of the _where_ clause of an SQL statement. Allowed fields in the search are: name, status. Allowed comparators are `=` or `LIKE`. Allowed joins are `AND` and `OR`, however there is a limit of max 10 joins in the search query.  Examples:  To retrieve request with name equal `my-registry`  the value should be:  ``` name = my-registry  ```  To retrieve kafka request with its name starting with `my`, the value should be:  ``` name like my%25 ```  If the parameter isn't provided, or if the value is empty, then all the kafkas that the user has permission to see will be returned.  Note. If the query is invalid, an error will be returned  |  |

### Return type

[**crate::models::RegistryList**](RegistryList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_registry

> crate::models::Registry get_registry(id)
Get a Registry

Gets the details of a single instance of a `Registry`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A unique identifier for a `Registry`. | [required] |

### Return type

[**crate::models::Registry**](Registry.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

