# Carton

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**carton_identifiers** | Option<[**Vec<models::ContainerIdentification>**](ContainerIdentification.md)> | A list of carton identifiers. | [optional]
**carton_sequence_number** | **String** | Carton sequence number for the carton. The first carton will be 001, the second 002, and so on. This number is used as a reference to refer to this carton from the pallet level. | 
**dimensions** | Option<[**models::Dimensions**](Dimensions.md)> |  | [optional]
**weight** | Option<[**models::Weight**](Weight.md)> |  | [optional]
**tracking_number** | Option<**String**> | This is required to be provided for every carton in the small parcel shipments. | [optional]
**items** | [**Vec<models::ContainerItem>**](ContainerItem.md) | A list of container item details. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


