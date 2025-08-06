# RemovalShipmentItemAdjustment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**removal_shipment_item_id** | Option<**String**> | An identifier for an item in a removal shipment. | [optional]
**tax_collection_model** | Option<**String**> | The tax collection model applied to the item.  Possible values:  * MarketplaceFacilitator - Tax is withheld and remitted to the taxing authority by Amazon on behalf of the seller.  * Standard - Tax is paid to the seller and not remitted to the taxing authority by Amazon. | [optional]
**fulfillment_network_sku** | Option<**String**> | The Amazon fulfillment network SKU for the item. | [optional]
**adjusted_quantity** | Option<**i32**> | Adjusted quantity of removal shipmentItemAdjustment items. | [optional]
**revenue_adjustment** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**tax_amount_adjustment** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**tax_withheld_adjustment** | Option<[**models::Currency**](Currency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


