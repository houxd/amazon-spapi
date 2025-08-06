# ShippingService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipping_service_name** | **String** | A plain text representation of a carrier's shipping service. For example, \"UPS Ground\" or \"FedEx Standard Overnight\".  | 
**carrier_name** | **String** | The name of the carrier. | 
**shipping_service_id** | **String** | An Amazon-defined shipping service identifier. | 
**shipping_service_offer_id** | **String** | An Amazon-defined shipping service offer identifier. | 
**ship_date** | **String** | Date-time formatted timestamp. | 
**earliest_estimated_delivery_date** | Option<**String**> | Date-time formatted timestamp. | [optional]
**latest_estimated_delivery_date** | Option<**String**> | Date-time formatted timestamp. | [optional]
**rate** | [**models::CurrencyAmount**](CurrencyAmount.md) |  | 
**shipping_service_options** | [**models::ShippingServiceOptions**](ShippingServiceOptions.md) |  | 
**available_shipping_service_options** | Option<[**models::AvailableShippingServiceOptions**](AvailableShippingServiceOptions.md)> |  | [optional]
**available_label_formats** | Option<[**Vec<models::LabelFormat>**](LabelFormat.md)> | List of label formats. | [optional]
**available_format_options_for_label** | Option<[**Vec<models::LabelFormatOption>**](LabelFormatOption.md)> | The available label formats. | [optional]
**requires_additional_seller_inputs** | **bool** | When true, additional seller inputs are required. | 
**benefits** | Option<[**models::Benefits**](Benefits.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


