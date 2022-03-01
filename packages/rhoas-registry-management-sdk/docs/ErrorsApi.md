# \ErrorsApi

All URIs are relative to *https://api.openshift.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_error**](ErrorsApi.md#get_error) | **GET** /api/serviceregistry_mgmt/v1/errors/{id} | Get information about a specific error type.
[**get_errors**](ErrorsApi.md#get_errors) | **GET** /api/serviceregistry_mgmt/v1/errors | Get the list of all errors.



## get_error

> crate::models::Error get_error(id)
Get information about a specific error type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique identifier for an error type. | [required] |

### Return type

[**crate::models::Error**](Error.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_errors

> crate::models::ErrorList get_errors(page, size)
Get the list of all errors.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page index |  |
**size** | Option<**i32**> | Number of items in each page |  |

### Return type

[**crate::models::ErrorList**](ErrorList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

