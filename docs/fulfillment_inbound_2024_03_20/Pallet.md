# Pallet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dimensions** | Option<[**models::Dimensions**](Dimensions.md)> |  | [optional]
**package_id** | **String** | Primary key to uniquely identify a Package (Box or Pallet). | 
**quantity** | Option<**i32**> | The number of containers where all other properties like weight or dimensions are identical. | [optional]
**stackability** | Option<[**models::Stackability**](Stackability.md)> |  | [optional]
**weight** | Option<[**models::Weight**](Weight.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


