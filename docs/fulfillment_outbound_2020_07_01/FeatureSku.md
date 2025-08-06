# FeatureSku

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | Option<**String**> | Used to identify an item in the given marketplace. `SellerSKU` is qualified by the seller's SellerId, which is included with every operation that you submit. | [optional]
**fn_sku** | Option<**String**> | The unique SKU used by Amazon's fulfillment network. | [optional]
**asin** | Option<**String**> | The Amazon Standard Identification Number (ASIN) of the item. | [optional]
**sku_count** | Option<**f64**> | The number of SKUs available for this service. | [optional]
**overlapping_skus** | Option<**Vec<String>**> | Other seller SKUs that are shared across the same inventory. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


