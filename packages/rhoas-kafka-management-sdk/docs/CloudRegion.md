# CloudRegion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | Option<**String**> | Indicates the type of this object. Will be 'CloudRegion'. | [optional]
**id** | Option<**String**> | Unique identifier of the object. | [optional]
**display_name** | Option<**String**> | Name of the region for display purposes, for example `N. Virginia`. | [optional]
**enabled** | **bool** | Whether the region is enabled for deploying an OSD cluster. | [default to false]
**supported_instance_types** | **Vec<String>** | The Kafka instance types supported by this region. | 
**capacity** | [**Vec<crate::models::RegionCapacityListItem>**](RegionCapacityListItem.md) | Indicates whether there is capacity left per instance type | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


