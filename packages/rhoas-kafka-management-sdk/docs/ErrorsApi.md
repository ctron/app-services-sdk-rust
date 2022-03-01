# \ErrorsApi

All URIs are relative to *https://api.openshift.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_error_by_id**](ErrorsApi.md#get_error_by_id) | **GET** /api/kafkas_mgmt/v1/errors/{id} | Returns the error by id
[**get_errors**](ErrorsApi.md#get_errors) | **GET** /api/kafkas_mgmt/v1/errors | Returns the list of possible API errors



## get_error_by_id

> crate::models::Error get_error_by_id(id)
Returns the error by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |

### Return type

[**crate::models::Error**](Error.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_errors

> crate::models::ErrorList get_errors()
Returns the list of possible API errors

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ErrorList**](ErrorList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

