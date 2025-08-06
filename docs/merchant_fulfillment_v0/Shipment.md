# Shipment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | **String** | An Amazon-defined shipment identifier. | 
**amazon_order_id** | **String** | An Amazon-defined order identifier, in 3-7-7 format. | 
**seller_order_id** | Option<**String**> | A seller-defined order identifier. | [optional]
**item_list** | [**Vec<models::Item>**](Item.md) | The list of items you want to include in a shipment. | 
**ship_from_address** | [**models::Address**](Address.md) |  | 
**ship_to_address** | [**models::Address**](Address.md) |  | 
**package_dimensions** | [**models::PackageDimensions**](PackageDimensions.md) |  | 
**weight** | [**models::Weight**](Weight.md) |  | 
**insurance** | [**models::CurrencyAmount**](CurrencyAmount.md) |  | 
**shipping_service** | [**models::ShippingService**](ShippingService.md) |  | 
**label** | [**models::Label**](Label.md) |  | 
**status** | [**models::ShipmentStatus**](ShipmentStatus.md) |  | 
**tracking_id** | Option<**String**> | The shipment tracking identifier provided by the carrier. | [optional]
**created_date** | **String** | Date-time formatted timestamp. | 
**last_updated_date** | Option<**String**> | Date-time formatted timestamp. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


