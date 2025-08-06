# ShipmentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | Option<**String**> | The Amazon Standard Identification Number (ASIN) of the item. | [optional]
**seller_sku** | Option<**String**> | The seller SKU of the item. | [optional]
**order_item_id** | Option<**String**> | The Amazon-defined identifier for the order item. | [optional]
**title** | Option<**String**> | The name of the item. | [optional]
**quantity_ordered** | Option<**f64**> | The number of items ordered. | [optional]
**item_price** | Option<[**models::Money**](Money.md)> |  | [optional]
**shipping_price** | Option<[**models::Money**](Money.md)> |  | [optional]
**gift_wrap_price** | Option<[**models::Money**](Money.md)> |  | [optional]
**shipping_discount** | Option<[**models::Money**](Money.md)> |  | [optional]
**promotion_discount** | Option<[**models::Money**](Money.md)> |  | [optional]
**serial_numbers** | Option<**Vec<String>**> | The list of serial numbers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


