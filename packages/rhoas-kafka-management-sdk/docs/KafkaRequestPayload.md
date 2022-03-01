# KafkaRequestPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud_provider** | Option<**String**> | The cloud provider where the Kafka cluster will be created in | [optional]
**multi_az** | Option<**bool**> | Set this to true to configure the Kafka cluster to be multiAZ | [optional]
**name** | **String** | The name of the Kafka cluster. It must consist of lower-case alphanumeric characters or '-', start with an alphabetic character, and end with an alphanumeric character, and can not be longer than 32 characters. | 
**region** | Option<**String**> | The region where the Kafka cluster will be created in | [optional]
**reauthentication_enabled** | Option<**bool**> | Whether connection reauthentication is enabled or not. If set to true, connection reauthentication on the Kafka instance will be required every 5 minutes. The default value is true | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


