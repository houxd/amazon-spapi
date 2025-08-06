# BoxUpdateInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_information_source** | [**models::BoxContentInformationSource**](BoxContentInformationSource.md) |  | 
**dimensions** | [**models::Dimensions**](Dimensions.md) |  | 
**items** | Option<[**Vec<models::ItemInput>**](ItemInput.md)> | The items and their quantity in the box. This must be empty if the box `contentInformationSource` is `BARCODE_2D` or `MANUAL_PROCESS`. | [optional]
**package_id** | Option<**String**> | Primary key to uniquely identify a Box Package. PackageId must be provided if the intent is to update an existing box. Adding a new box will not require providing this value. Any existing PackageIds not provided will be treated as to-be-removed | [optional]
**quantity** | **i32** | The number of containers where all other properties like weight or dimensions are identical. | 
**weight** | [**models::Weight**](Weight.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


