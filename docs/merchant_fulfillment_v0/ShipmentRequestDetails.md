# ShipmentRequestDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon-defined order identifier, in 3-7-7 format. | 
**seller_order_id** | Option<**String**> | A seller-defined order identifier. | [optional]
**item_list** | [**Vec<models::Item>**](Item.md) | The list of items you want to include in a shipment. | 
**ship_from_address** | [**models::Address**](Address.md) |  | 
**package_dimensions** | [**models::PackageDimensions**](PackageDimensions.md) |  | 
**weight** | [**models::Weight**](Weight.md) |  | 
**must_arrive_by_date** | Option<**String**> | Date-time formatted timestamp. | [optional]
**ship_date** | Option<**String**> | Date-time formatted timestamp. | [optional]
**shipping_service_options** | [**models::ShippingServiceOptions**](ShippingServiceOptions.md) |  | 
**label_customization** | Option<[**models::LabelCustomization**](LabelCustomization.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


