# ProductQuantity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attributes** | Option<[**Vec<models::ProductAttribute>**](ProductAttribute.md)> | Contains attributes for this instance of the product. For example, item color, or other attributes that distinguish the product beyond the SKU. This is metadata for the product and Amazon does not process this data. | [optional]
**quantity** | **i32** | Product quantity. | 
**sku** | **String** | The seller or merchant SKU. | 
**expiration** | Option<**String**> | The expiration date for the SKU. Values are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. | [optional]
**prep_details** | Option<[**models::PrepDetails**](PrepDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


