# \GroupsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_consumer_group_by_id**](GroupsApi.md#delete_consumer_group_by_id) | **DELETE** /consumer-groups/{consumerGroupId} | Delete a consumer group.
[**get_consumer_group_by_id**](GroupsApi.md#get_consumer_group_by_id) | **GET** /consumer-groups/{consumerGroupId} | Get a single consumer group by its unique ID.
[**get_consumer_groups**](GroupsApi.md#get_consumer_groups) | **GET** /consumer-groups | List of consumer groups in the Kafka instance.
[**reset_consumer_group_offset**](GroupsApi.md#reset_consumer_group_offset) | **POST** /consumer-groups/{consumerGroupId}/reset-offset | Reset the offset for a consumer group.



## delete_consumer_group_by_id

> delete_consumer_group_by_id(consumer_group_id)
Delete a consumer group.

Delete a consumer group, along with its consumers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consumer_group_id** | **String** | The unique ID of the cobsumer group. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consumer_group_by_id

> crate::models::ConsumerGroup get_consumer_group_by_id(consumer_group_id, order, order_key, partition_filter, topic)
Get a single consumer group by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consumer_group_id** | **String** | The unique ID of the consumer group | [required] |
**order** | Option<**String**> | Order of the items sorting. Ascending order is used as default. |  |
**order_key** | Option<**String**> | Order key to sort the topics by. |  |
**partition_filter** | Option<**i32**> | Value of partition to include. Value -1 means filter is not active. |  |
**topic** | Option<**String**> | Filter consumer groups for a specific topic |  |

### Return type

[**crate::models::ConsumerGroup**](ConsumerGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consumer_groups

> crate::models::ConsumerGroupList get_consumer_groups(offset, limit, size, page, topic, group_id_filter, order, order_key)
List of consumer groups in the Kafka instance.

Returns a list of all consumer groups for a particular Kafka instance. The consumer groups returned are limited to those records the requestor is authorized to view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The page offset |  |
**limit** | Option<**i32**> | Maximum number of consumer groups to return |  |
**size** | Option<**i32**> | Maximum number of consumer groups to return on single page |  |
**page** | Option<**i32**> | The page when returning the list of consumer groups |  |
**topic** | Option<**String**> | Return consumer groups where the topic name contains with this value |  |
**group_id_filter** | Option<**String**> | Return the consumer groups where the ID contains with this value |  |
**order** | Option<**String**> | Order of the consumer groups sorting. Ascending order is used as default. |  |
**order_key** | Option<**String**> | Order key to sort the items by. Only the value 'name' is currently applicable. |  |

### Return type

[**crate::models::ConsumerGroupList**](ConsumerGroupList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_consumer_group_offset

> crate::models::ConsumerGroupResetOffsetResult reset_consumer_group_offset(consumer_group_id, consumer_group_reset_offset_parameters)
Reset the offset for a consumer group.

Reset the offset for a particular consumer group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consumer_group_id** | **String** | The ID of the consumer group. | [required] |
**consumer_group_reset_offset_parameters** | [**ConsumerGroupResetOffsetParameters**](ConsumerGroupResetOffsetParameters.md) |  | [required] |

### Return type

[**crate::models::ConsumerGroupResetOffsetResult**](ConsumerGroupResetOffsetResult.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

