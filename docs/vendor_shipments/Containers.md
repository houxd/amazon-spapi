# Containers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_type** | **String** | The type of container. | 
**container_sequence_number** | Option<**String**> | An integer that must be submitted for multi-box shipments only, where one item may come in separate packages. | [optional]
**container_identifiers** | [**Vec<models::ContainerIdentification>**](ContainerIdentification.md) | A list of carton identifiers. | 
**tracking_number** | Option<**String**> | The tracking number used for identifying the shipment. | [optional]
**dimensions** | Option<[**models::Dimensions**](Dimensions.md)> |  | [optional]
**weight** | Option<[**models::Weight**](Weight.md)> |  | [optional]
**tier** | Option<**i32**> | Number of layers per pallet. | [optional]
**block** | Option<**i32**> | Number of cartons per layer on the pallet. | [optional]
**inner_containers_details** | Option<[**models::InnerContainersDetails**](InnerContainersDetails.md)> |  | [optional]
**packed_items** | Option<[**Vec<models::PackedItems>**](PackedItems.md)> | A list of packed items. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


