# BoxInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_information_source** | [**models::BoxContentInformationSource**](BoxContentInformationSource.md) |  | 
**dimensions** | [**models::Dimensions**](Dimensions.md) |  | 
**items** | Option<[**Vec<models::ItemInput>**](ItemInput.md)> | The items and their quantity in the box. This must be empty if the box `contentInformationSource` is `BARCODE_2D` or `MANUAL_PROCESS`. | [optional]
**quantity** | **i32** | The number of containers where all other properties like weight or dimensions are identical. | 
**weight** | [**models::Weight**](Weight.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


