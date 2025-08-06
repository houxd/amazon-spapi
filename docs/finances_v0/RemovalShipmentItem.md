# RemovalShipmentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**removal_shipment_item_id** | Option<**String**> | An identifier for an item in a removal shipment. | [optional]
**tax_collection_model** | Option<**String**> | The tax collection model applied to the item.  Possible values:  * MarketplaceFacilitator - Tax is withheld and remitted to the taxing authority by Amazon on behalf of the seller.  * Standard - Tax is paid to the seller and not remitted to the taxing authority by Amazon. | [optional]
**fulfillment_network_sku** | Option<**String**> | The Amazon fulfillment network SKU for the item. | [optional]
**quantity** | Option<**i32**> | The quantity of the item. | [optional]
**revenue** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**fee_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**tax_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**tax_withheld** | Option<[**models::Currency**](Currency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


