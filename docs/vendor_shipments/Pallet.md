# Pallet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pallet_identifiers** | [**Vec<models::ContainerIdentification>**](ContainerIdentification.md) | A list of pallet identifiers. | 
**tier** | Option<**i32**> | Number of layers per pallet. Only applicable to container type Pallet. | [optional]
**block** | Option<**i32**> | Number of cartons per layer on the pallet. Only applicable to container type Pallet. | [optional]
**dimensions** | Option<[**models::Dimensions**](Dimensions.md)> |  | [optional]
**weight** | Option<[**models::Weight**](Weight.md)> |  | [optional]
**carton_reference_details** | Option<[**models::CartonReferenceDetails**](CartonReferenceDetails.md)> |  | [optional]
**items** | Option<[**Vec<models::ContainerItem>**](ContainerItem.md)> | A list of container item details. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


