# Container

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_type** | Option<**String**> | The type of physical container being used. (always 'PACKAGE') | [optional]
**container_reference_id** | **String** | An identifier for the container. This must be unique within all the containers in the same shipment. | 
**value** | [**models::Currency**](Currency.md) |  | 
**dimensions** | [**models::Dimensions**](Dimensions.md) |  | 
**items** | [**Vec<models::ContainerItem>**](ContainerItem.md) | A list of the items in the container. | 
**weight** | [**models::Weight**](Weight.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


