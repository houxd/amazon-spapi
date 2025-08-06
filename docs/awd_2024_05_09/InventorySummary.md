# InventorySummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expiration_details** | Option<[**Vec<models::ExpirationDetails>**](ExpirationDetails.md)> | The expiration details of the inventory. This object will only appear if the `details` parameter in the request is set to `SHOW`. | [optional]
**inventory_details** | Option<[**models::InventoryDetails**](InventoryDetails.md)> |  | [optional]
**sku** | **String** | The seller or merchant SKU. | 
**total_inbound_quantity** | Option<**i64**> | Total quantity that is in-transit from the seller and has not yet been received at an AWD Distribution Center | [optional]
**total_onhand_quantity** | Option<**i64**> | Total quantity that is present in AWD distribution centers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


