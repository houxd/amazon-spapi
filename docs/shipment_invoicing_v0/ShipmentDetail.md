# ShipmentDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**warehouse_id** | Option<**String**> | The Amazon-defined identifier for the warehouse. | [optional]
**amazon_order_id** | Option<**String**> | The Amazon-defined identifier for the order. | [optional]
**amazon_shipment_id** | Option<**String**> | The Amazon-defined identifier for the shipment. | [optional]
**purchase_date** | Option<**String**> | The date and time when the order was created. | [optional]
**shipping_address** | Option<[**models::Address**](Address.md)> |  | [optional]
**payment_method_details** | Option<**Vec<String>**> | The list of payment method details. | [optional]
**marketplace_id** | Option<**String**> | The identifier for the marketplace where the order was placed. | [optional]
**seller_id** | Option<**String**> | The seller identifier. | [optional]
**buyer_name** | Option<**String**> | The name of the buyer. | [optional]
**buyer_county** | Option<**String**> | The county of the buyer. | [optional]
**buyer_tax_info** | Option<[**models::BuyerTaxInfo**](BuyerTaxInfo.md)> |  | [optional]
**marketplace_tax_info** | Option<[**models::MarketplaceTaxInfo**](MarketplaceTaxInfo.md)> |  | [optional]
**seller_display_name** | Option<**String**> | The seller’s friendly name registered in the marketplace. | [optional]
**shipment_items** | Option<[**Vec<models::ShipmentItem>**](ShipmentItem.md)> | A list of shipment items. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


