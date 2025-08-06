# UpdateFulfillmentOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | Option<**String**> | The marketplace the fulfillment order is placed against. | [optional]
**displayable_order_id** | Option<**String**> | A fulfillment order identifier that the seller creates. This value displays as the order identifier in recipient-facing materials such as the outbound shipment packing slip. The value of `DisplayableOrderId` should match the order identifier that the seller provides to the recipient. The seller can use the `SellerFulfillmentOrderId` for this value or they can specify an alternate value if they want the recipient to reference an alternate order identifier. | [optional]
**displayable_order_date** | Option<**String**> | Date timestamp | [optional]
**displayable_order_comment** | Option<**String**> | Order-specific text that appears in recipient-facing materials such as the outbound shipment packing slip. | [optional]
**shipping_speed_category** | Option<[**models::ShippingSpeedCategory**](ShippingSpeedCategory.md)> |  | [optional]
**destination_address** | Option<[**models::Address**](Address.md)> |  | [optional]
**fulfillment_action** | Option<[**models::FulfillmentAction**](FulfillmentAction.md)> |  | [optional]
**fulfillment_policy** | Option<[**models::FulfillmentPolicy**](FulfillmentPolicy.md)> |  | [optional]
**ship_from_country_code** | Option<**String**> | The two-character country code for the country from which the fulfillment order ships. Must be in ISO 3166-1 alpha-2 format. | [optional]
**notification_emails** | Option<**Vec<String>**> | A list of email addresses that the seller provides that are used by Amazon to send ship-complete notifications to recipients on behalf of the seller. | [optional]
**feature_constraints** | Option<[**Vec<models::FeatureSettings>**](FeatureSettings.md)> | A list of features and their fulfillment policies to apply to the order. | [optional]
**items** | Option<[**Vec<models::UpdateFulfillmentOrderItem>**](UpdateFulfillmentOrderItem.md)> | An array of fulfillment order item information for updating a fulfillment order. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


