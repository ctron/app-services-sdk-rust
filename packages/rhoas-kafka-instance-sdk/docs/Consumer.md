# Consumer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | **String** | Unique identifier for the consumer group to which this consumer belongs. | 
**topic** | **String** | The unique topic name to which this consumer belongs | 
**partition** | **i32** | The partition number to which this consumer group is assigned to. | 
**offset** | **f32** | Offset denotes the position of the consumer in a partition. | 
**log_end_offset** | Option<**f32**> | The log end offset is the offset of the last message written to a log. | [optional]
**lag** | **i32** | Offset Lag is the delta between the last produced message and the last consumer's committed offset. | 
**member_id** | Option<**String**> | The member ID is a unique identifier given to a consumer by the coordinator upon initially joining the group. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


