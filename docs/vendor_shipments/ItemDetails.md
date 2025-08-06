# ItemDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | Option<**String**> | The purchase order number for the shipment being confirmed. If the items in this shipment belong to multiple purchase order numbers that are in particular carton or pallet within the shipment, then provide the purchaseOrderNumber at the appropriate carton or pallet level. Formatting Notes: 8-character alpha-numeric code. | [optional]
**lot_number** | Option<**String**> | The batch or lot number associates an item with information the manufacturer considers relevant for traceability of the trade item to which the Element String is applied. The data may refer to the trade item itself or to items contained. This field is mandatory for all perishable items. | [optional]
**expiry** | Option<[**models::Expiry**](Expiry.md)> |  | [optional]
**maximum_retail_price** | Option<[**models::Money**](Money.md)> |  | [optional]
**handling_code** | Option<**String**> | Identification of the instructions on how specified item/carton/pallet should be handled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


