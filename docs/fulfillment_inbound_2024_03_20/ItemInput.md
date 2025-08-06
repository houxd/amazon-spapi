# ItemInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expiration** | Option<**String**> | The expiration date of the MSKU. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) datetime format with pattern `YYYY-MM-DD`. Items with the same MSKU but different expiration dates cannot go into the same box. | [optional]
**label_owner** | [**models::LabelOwner**](LabelOwner.md) |  | 
**manufacturing_lot_code** | Option<**String**> | The manufacturing lot code. | [optional]
**msku** | **String** | The merchant SKU, a merchant-supplied identifier of a specific SKU. | 
**prep_owner** | [**models::PrepOwner**](PrepOwner.md) |  | 
**quantity** | **i32** | The number of units of the specified MSKU that will be shipped. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


