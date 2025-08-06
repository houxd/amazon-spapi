# InventorySummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | Option<**String**> | The Amazon Standard Identification Number (ASIN) of an item. | [optional]
**fn_sku** | Option<**String**> | Amazon's fulfillment network SKU identifier. | [optional]
**seller_sku** | Option<**String**> | The seller SKU of the item. | [optional]
**condition** | Option<**String**> | The condition of the item as described by the seller (for example, New Item). | [optional]
**inventory_details** | Option<[**models::InventoryDetails**](InventoryDetails.md)> |  | [optional]
**last_updated_time** | Option<**String**> | The date and time that any quantity was last updated. | [optional]
**product_name** | Option<**String**> | The localized language product title of the item within the specific marketplace. | [optional]
**total_quantity** | Option<**i32**> | The total number of units in an inbound shipment or in Amazon fulfillment centers. | [optional]
**stores** | Option<**Vec<String>**> | A list of seller-enrolled stores that apply to this seller SKU. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


