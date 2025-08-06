# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_value** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**description** | Option<**String**> | The product description of the item. | [optional]
**item_identifier** | Option<**String**> | A unique identifier for an item provided by the client. | [optional]
**quantity** | **i32** | The number of units. This value is required. | 
**weight** | Option<[**models::Weight**](Weight.md)> |  | [optional]
**liquid_volume** | Option<[**models::LiquidVolume**](LiquidVolume.md)> |  | [optional]
**is_hazmat** | Option<**bool**> | When true, the item qualifies as hazardous materials (hazmat). Defaults to false. | [optional]
**dangerous_goods_details** | Option<[**models::DangerousGoodsDetails**](DangerousGoodsDetails.md)> |  | [optional]
**product_type** | Option<**String**> | The product type of the item. | [optional]
**invoice_details** | Option<[**models::InvoiceDetails**](InvoiceDetails.md)> |  | [optional]
**serial_numbers** | Option<**Vec<String>**> | A list of unique serial numbers in an Amazon package that can be used to guarantee non-fraudulent items. The number of serial numbers in the list must be less than or equal to the quantity of items being shipped. Only applicable when channel source is Amazon. | [optional]
**direct_fulfillment_item_identifiers** | Option<[**models::DirectFulfillmentItemIdentifiers**](DirectFulfillmentItemIdentifiers.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


