# Partition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**partition** | **i32** | The partition id, unique among partitions of the same topic | 
**id** | Option<**i32**> | Unique id for the partition (deprecated, use `partition` instead) | [optional]
**replicas** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | List of replicas for the partition | [optional]
**isr** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | List in-sync replicas for this partition. | [optional]
**leader** | Option<[**serde_json::Value**](.md)> | Kafka server / broker. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


