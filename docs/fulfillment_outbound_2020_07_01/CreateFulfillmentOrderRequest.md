# CreateFulfillmentOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | Option<**String**> | The marketplace the fulfillment order is placed against. | [optional]
**seller_fulfillment_order_id** | **String** | A fulfillment order identifier that the seller creates to track their fulfillment order. The `SellerFulfillmentOrderId` must be unique for each fulfillment order that a seller creates. If the seller's system already creates unique order identifiers, then these might be good values for them to use. | 
**displayable_order_id** | **String** | A fulfillment order identifier that the seller creates. This value displays as the order identifier in recipient-facing materials such as the outbound shipment packing slip. The value of `DisplayableOrderId` should match the order identifier that the seller provides to the recipient. The seller can use the `SellerFulfillmentOrderId` for this value or they can specify an alternate value if they want the recipient to reference an alternate order identifier.  The value must be an alpha-numeric or ISO 8859-1 compliant string from one to 40 characters in length. Cannot contain two spaces in a row. Leading and trailing white space is removed. | 
**displayable_order_date** | **String** | Date timestamp | 
**displayable_order_comment** | **String** | Order-specific text that appears in recipient-facing materials such as the outbound shipment packing slip. | 
**shipping_speed_category** | [**models::ShippingSpeedCategory**](ShippingSpeedCategory.md) |  | 
**delivery_window** | Option<[**models::DeliveryWindow**](DeliveryWindow.md)> |  | [optional]
**destination_address** | [**models::Address**](Address.md) |  | 
**delivery_preferences** | Option<[**models::DeliveryPreferences**](DeliveryPreferences.md)> |  | [optional]
**fulfillment_action** | Option<[**models::FulfillmentAction**](FulfillmentAction.md)> |  | [optional]
**fulfillment_policy** | Option<[**models::FulfillmentPolicy**](FulfillmentPolicy.md)> |  | [optional]
**cod_settings** | Option<[**models::CodSettings**](CODSettings.md)> |  | [optional]
**ship_from_country_code** | Option<**String**> | The two-character country code for the country from which the fulfillment order ships. Must be in ISO 3166-1 alpha-2 format. | [optional]
**notification_emails** | Option<**Vec<String>**> | A list of email addresses that the seller provides that are used by Amazon to send ship-complete notifications to recipients on behalf of the seller. | [optional]
**feature_constraints** | Option<[**Vec<models::FeatureSettings>**](FeatureSettings.md)> | A list of features and their fulfillment policies to apply to the order. | [optional]
**items** | [**Vec<models::CreateFulfillmentOrderItem>**](CreateFulfillmentOrderItem.md) | An array of item information for creating a fulfillment order. | 
**payment_information** | Option<[**Vec<models::PaymentInformation>**](PaymentInformation.md)> | An array of various payment attributes related to this fulfillment order. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


