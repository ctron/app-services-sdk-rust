# \SecurityApi

All URIs are relative to *https://api.openshift.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_service_account**](SecurityApi.md#create_service_account) | **POST** /api/kafkas_mgmt/v1/service_accounts | Creates a service account
[**delete_service_account_by_id**](SecurityApi.md#delete_service_account_by_id) | **DELETE** /api/kafkas_mgmt/v1/service_accounts/{id} | Deletes a service account by ID
[**get_service_account_by_id**](SecurityApi.md#get_service_account_by_id) | **GET** /api/kafkas_mgmt/v1/service_accounts/{id} | Returned service account by ID
[**get_service_accounts**](SecurityApi.md#get_service_accounts) | **GET** /api/kafkas_mgmt/v1/service_accounts | Returns a list of service accounts
[**reset_service_account_creds**](SecurityApi.md#reset_service_account_creds) | **POST** /api/kafkas_mgmt/v1/service_accounts/{id}/reset_credentials | Resets the credentials for a service account by ID



## create_service_account

> crate::models::ServiceAccount create_service_account(service_account_request)
Creates a service account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account_request** | [**ServiceAccountRequest**](ServiceAccountRequest.md) | Service account request | [required] |

### Return type

[**crate::models::ServiceAccount**](ServiceAccount.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_account_by_id

> crate::models::Error delete_service_account_by_id(id)
Deletes a service account by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |

### Return type

[**crate::models::Error**](Error.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_account_by_id

> crate::models::ServiceAccount get_service_account_by_id(id)
Returned service account by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |

### Return type

[**crate::models::ServiceAccount**](ServiceAccount.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_accounts

> crate::models::ServiceAccountList get_service_accounts()
Returns a list of service accounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ServiceAccountList**](ServiceAccountList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_service_account_creds

> crate::models::ServiceAccount reset_service_account_creds(id)
Resets the credentials for a service account by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |

### Return type

[**crate::models::ServiceAccount**](ServiceAccount.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

