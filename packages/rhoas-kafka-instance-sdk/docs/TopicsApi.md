# \TopicsApi

All URIs are relative to */rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_topic**](TopicsApi.md#create_topic) | **POST** /topics | Creates a new topic
[**delete_topic**](TopicsApi.md#delete_topic) | **DELETE** /topics/{topicName} | Deletes a  topic
[**get_topic**](TopicsApi.md#get_topic) | **GET** /topics/{topicName} | Retrieves the topic with the specified name.
[**get_topics**](TopicsApi.md#get_topics) | **GET** /topics | List of topics
[**update_topic**](TopicsApi.md#update_topic) | **PATCH** /topics/{topicName} | Updates the topic with the specified name.



## create_topic

> crate::models::Topic create_topic(new_topic_input)
Creates a new topic

Creates a new topic for Kafka.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_topic_input** | [**NewTopicInput**](NewTopicInput.md) | Topic to create. | [required] |

### Return type

[**crate::models::Topic**](Topic.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_topic

> delete_topic(topic_name)
Deletes a  topic

Deletes the topic with the specified name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic_name** | **String** | The topic name to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_topic

> crate::models::Topic get_topic(topic_name)
Retrieves the topic with the specified name.

Topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic_name** | **String** | The topic name to retrieve. | [required] |

### Return type

[**crate::models::Topic**](Topic.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_topics

> crate::models::TopicsList get_topics(offset, limit, size, filter, page, order, order_key)
List of topics

Returns a list of all of the available topics, or the list of topics that meet the request query parameters. The topics returned are limited to those records the requestor is authorized to view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The page offset |  |
**limit** | Option<**i32**> | Maximum number of topics to return |  |
**size** | Option<**i32**> | Maximum number of topics to return on single page |  |
**filter** | Option<**String**> | Filter to apply when returning the list of topics |  |
**page** | Option<**i32**> | The page when returning the limit of requested topics. |  |
**order** | Option<**String**> | Order of the items sorting. Ascending order is used as default. |  |
**order_key** | Option<**String**> | Order key to sort the topics by. |  |

### Return type

[**crate::models::TopicsList**](TopicsList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_topic

> crate::models::Topic update_topic(topic_name, update_topic_input)
Updates the topic with the specified name.

updates the topic with the new data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic_name** | **String** | The topic name which is its unique id. | [required] |
**update_topic_input** | [**UpdateTopicInput**](UpdateTopicInput.md) |  | [required] |

### Return type

[**crate::models::Topic**](Topic.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

