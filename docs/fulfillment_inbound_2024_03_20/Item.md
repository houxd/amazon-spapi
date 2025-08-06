# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | 
**expiration** | Option<**String**> | The expiration date of the MSKU. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) datetime format with pattern`YYYY-MM-DD`. The same MSKU with different expiration dates cannot go into the same box. | [optional]
**fnsku** | **String** | A unique identifier assigned by Amazon to products stored in and fulfilled from an Amazon fulfillment center. | 
**label_owner** | **String** | Specifies who will label the items. Options include `AMAZON`, `SELLER`, and `NONE`. | 
**manufacturing_lot_code** | Option<**String**> | The manufacturing lot code. | [optional]
**msku** | **String** | The merchant SKU, a merchant-supplied identifier of a specific SKU. | 
**prep_instructions** | [**Vec<models::PrepInstruction>**](PrepInstruction.md) | Special preparations that are required for an item. | 
**quantity** | **i32** | The number of the specified MSKU. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


