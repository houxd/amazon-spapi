# FulfillmentOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_fulfillment_order_id** | **String** | The fulfillment order identifier submitted with the `createFulfillmentOrder` operation. | 
**marketplace_id** | **String** | The identifier for the marketplace the fulfillment order is placed against. | 
**displayable_order_id** | **String** | A fulfillment order identifier submitted with the `createFulfillmentOrder` operation. Displays as the order identifier in recipient-facing materials such as the packing slip. | 
**displayable_order_date** | **String** | Date timestamp | 
**displayable_order_comment** | **String** | A text block submitted with the `createFulfillmentOrder` operation. Displays in recipient-facing materials such as the packing slip. | 
**shipping_speed_category** | [**models::ShippingSpeedCategory**](ShippingSpeedCategory.md) |  | 
**delivery_window** | Option<[**models::DeliveryWindow**](DeliveryWindow.md)> |  | [optional]
**destination_address** | [**models::Address**](Address.md) |  | 
**fulfillment_action** | Option<[**models::FulfillmentAction**](FulfillmentAction.md)> |  | [optional]
**fulfillment_policy** | Option<[**models::FulfillmentPolicy**](FulfillmentPolicy.md)> |  | [optional]
**cod_settings** | Option<[**models::CodSettings**](CODSettings.md)> |  | [optional]
**received_date** | **String** | Date timestamp | 
**fulfillment_order_status** | [**models::FulfillmentOrderStatus**](FulfillmentOrderStatus.md) |  | 
**status_updated_date** | **String** | Date timestamp | 
**notification_emails** | Option<**Vec<String>**> | A list of email addresses that the seller provides that are used by Amazon to send ship-complete notifications to recipients on behalf of the seller. | [optional]
**feature_constraints** | Option<[**Vec<models::FeatureSettings>**](FeatureSettings.md)> | A list of features and their fulfillment policies to apply to the order. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


