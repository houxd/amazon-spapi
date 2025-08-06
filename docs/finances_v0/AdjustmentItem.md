# AdjustmentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quantity** | Option<**String**> | Represents the number of units in the seller's inventory when the AdustmentType is FBAInventoryReimbursement. | [optional]
**per_unit_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**total_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**seller_sku** | Option<**String**> | The seller SKU of the item. The seller SKU is qualified by the seller's seller ID, which is included with every call to the Selling Partner API. | [optional]
**fn_sku** | Option<**String**> | A unique identifier assigned to products stored in and fulfilled from a fulfillment center. | [optional]
**product_description** | Option<**String**> | A short description of the item. | [optional]
**asin** | Option<**String**> | The Amazon Standard Identification Number (ASIN) of the item. | [optional]
**transaction_number** | Option<**String**> | The transaction number that is related to the adjustment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


