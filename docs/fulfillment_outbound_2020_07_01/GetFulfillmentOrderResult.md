# GetFulfillmentOrderResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fulfillment_order** | [**models::FulfillmentOrder**](FulfillmentOrder.md) |  | 
**fulfillment_order_items** | [**Vec<models::FulfillmentOrderItem>**](FulfillmentOrderItem.md) | An array of fulfillment order item information. | 
**fulfillment_shipments** | Option<[**Vec<models::FulfillmentShipment>**](FulfillmentShipment.md)> | An array of fulfillment shipment information. | [optional]
**return_items** | [**Vec<models::ReturnItem>**](ReturnItem.md) | An array of items that Amazon accepted for return. Returns empty if no items were accepted for return. | 
**return_authorizations** | [**Vec<models::ReturnAuthorization>**](ReturnAuthorization.md) | An array of return authorization information. | 
**payment_information** | Option<[**Vec<models::PaymentInformation>**](PaymentInformation.md)> | An array of various payment attributes related to this fulfillment order. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


