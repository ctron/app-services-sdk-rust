# \DefaultApi

All URIs are relative to *https://api.openshift.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_kafka**](DefaultApi.md#create_kafka) | **POST** /api/kafkas_mgmt/v1/kafkas | Creates a Kafka request
[**delete_kafka_by_id**](DefaultApi.md#delete_kafka_by_id) | **DELETE** /api/kafkas_mgmt/v1/kafkas/{id} | Deletes a Kafka request by ID
[**federate_metrics**](DefaultApi.md#federate_metrics) | **GET** /api/kafkas_mgmt/v1/kafkas/{id}/metrics/federate | Returns all metrics in scrapeable format for a given kafka id
[**get_cloud_provider_regions**](DefaultApi.md#get_cloud_provider_regions) | **GET** /api/kafkas_mgmt/v1/cloud_providers/{id}/regions | Returns the list of supported regions of the supported cloud provider
[**get_cloud_providers**](DefaultApi.md#get_cloud_providers) | **GET** /api/kafkas_mgmt/v1/cloud_providers | Returns the list of supported cloud providers
[**get_kafka_by_id**](DefaultApi.md#get_kafka_by_id) | **GET** /api/kafkas_mgmt/v1/kafkas/{id} | Returns a Kafka request by ID
[**get_kafkas**](DefaultApi.md#get_kafkas) | **GET** /api/kafkas_mgmt/v1/kafkas | Returns a list of Kafka requests
[**get_metrics_by_instant_query**](DefaultApi.md#get_metrics_by_instant_query) | **GET** /api/kafkas_mgmt/v1/kafkas/{id}/metrics/query | Returns metrics with instant query by Kafka ID
[**get_metrics_by_range_query**](DefaultApi.md#get_metrics_by_range_query) | **GET** /api/kafkas_mgmt/v1/kafkas/{id}/metrics/query_range | Returns metrics with timeseries range query by Kafka ID
[**get_service_status**](DefaultApi.md#get_service_status) | **GET** /api/kafkas_mgmt/v1/status | Returns the status of resources, such as whether maximum service capacity has been reached
[**get_version_metadata**](DefaultApi.md#get_version_metadata) | **GET** /api/kafkas_mgmt/v1 | Returns the version metadata
[**update_kafka_by_id**](DefaultApi.md#update_kafka_by_id) | **PATCH** /api/kafkas_mgmt/v1/kafkas/{id} | Update a Kafka instance by id



## create_kafka

> crate::models::KafkaRequest create_kafka(_async, kafka_request_payload)
Creates a Kafka request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_async** | **bool** | Perform the action in an asynchronous manner | [required] |
**kafka_request_payload** | [**KafkaRequestPayload**](KafkaRequestPayload.md) | Kafka data | [required] |

### Return type

[**crate::models::KafkaRequest**](KafkaRequest.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_kafka_by_id

> crate::models::Error delete_kafka_by_id(id, _async)
Deletes a Kafka request by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |
**_async** | **bool** | Perform the action in an asynchronous manner | [required] |

### Return type

[**crate::models::Error**](Error.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## federate_metrics

> String federate_metrics(id)
Returns all metrics in scrapeable format for a given kafka id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |

### Return type

**String**

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_provider_regions

> crate::models::CloudRegionList get_cloud_provider_regions(id, page, size, instance_type)
Returns the list of supported regions of the supported cloud provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |
**page** | Option<**String**> | Page index |  |
**size** | Option<**String**> | Number of items in each page |  |
**instance_type** | Option<**String**> | The Kafka instance type to filter the results by |  |

### Return type

[**crate::models::CloudRegionList**](CloudRegionList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_providers

> crate::models::CloudProviderList get_cloud_providers(page, size)
Returns the list of supported cloud providers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**String**> | Page index |  |
**size** | Option<**String**> | Number of items in each page |  |

### Return type

[**crate::models::CloudProviderList**](CloudProviderList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kafka_by_id

> crate::models::KafkaRequest get_kafka_by_id(id)
Returns a Kafka request by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |

### Return type

[**crate::models::KafkaRequest**](KafkaRequest.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kafkas

> crate::models::KafkaRequestList get_kafkas(page, size, order_by, search)
Returns a list of Kafka requests

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**String**> | Page index |  |
**size** | Option<**String**> | Number of items in each page |  |
**order_by** | Option<**String**> | Specifies the order by criteria. The syntax of this parameter is similar to the syntax of the `order by` clause of an SQL statement. Each query can be ordered by any of the following `kafkaRequests` fields:  * bootstrap_server_host * cloud_provider * cluster_id * created_at * href * id * instance_type * multi_az * name * organisation_id * owner * reauthentication_enabled * region * status * updated_at * version  For example, to return all Kafka instances ordered by their name, use the following syntax:  ```sql name asc ```  To return all Kafka instances ordered by their name _and_ created date, use the following syntax:  ```sql name asc, created_at asc ```  If the parameter isn't provided, or if the value is empty, then the results are ordered by name. |  |
**search** | Option<**String**> | Search criteria.  The syntax of this parameter is similar to the syntax of the `where` clause of an SQL statement. Allowed fields in the search are `cloud_provider`, `name`, `owner`, `region`, and `status`. Allowed comparators are `<>`, `=`, or `LIKE`. Allowed joins are `AND` and `OR`. However, you can use a maximum of 10 joins in a search query.  Examples:  To return a Kafka instance with the name `my-kafka` and the region `aws`, use the following syntax:  ``` name = my-kafka and cloud_provider = aws ```[p-]  To return a Kafka instance with a name that starts with `my`, use the following syntax:  ``` name like my%25 ```  If the parameter isn't provided, or if the value is empty, then all the Kafka instances that the user has permission to see are returned.  Note. If the query is invalid, an error is returned.  |  |

### Return type

[**crate::models::KafkaRequestList**](KafkaRequestList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metrics_by_instant_query

> crate::models::MetricsInstantQueryList get_metrics_by_instant_query(id, filters)
Returns metrics with instant query by Kafka ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |
**filters** | Option<[**Vec<String>**](String.md)> | List of metrics to fetch. Fetch all metrics when empty. List entries are Kafka internal metric names. |  |[default to []]

### Return type

[**crate::models::MetricsInstantQueryList**](MetricsInstantQueryList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metrics_by_range_query

> crate::models::MetricsRangeQueryList get_metrics_by_range_query(id, duration, interval, filters)
Returns metrics with timeseries range query by Kafka ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |
**duration** | **i64** | The length of time in minutes for which to return the metrics | [required] |[default to 5]
**interval** | **i64** | The interval in seconds between data points | [required] |[default to 30]
**filters** | Option<[**Vec<String>**](String.md)> | List of metrics to fetch. Fetch all metrics when empty. List entries are Kafka internal metric names. |  |[default to []]

### Return type

[**crate::models::MetricsRangeQueryList**](MetricsRangeQueryList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_status

> crate::models::ServiceStatus get_service_status()
Returns the status of resources, such as whether maximum service capacity has been reached

[DEPRECATED] The service capacity status is now reported per cloud provider region and Kafka instance type in the /api/kafkas_mgmt/v1/cloud_providers/{id}/regions endpoint. Please use this endpoint instead.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ServiceStatus**](ServiceStatus.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version_metadata

> crate::models::VersionMetadata get_version_metadata()
Returns the version metadata

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VersionMetadata**](VersionMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_kafka_by_id

> crate::models::KafkaRequest update_kafka_by_id(id, kafka_update_request)
Update a Kafka instance by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of record | [required] |
**kafka_update_request** | [**KafkaUpdateRequest**](KafkaUpdateRequest.md) | Update owner of kafka | [required] |

### Return type

[**crate::models::KafkaRequest**](KafkaRequest.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

