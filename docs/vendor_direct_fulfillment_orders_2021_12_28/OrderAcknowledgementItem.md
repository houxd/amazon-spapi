# OrderAcknowledgementItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The purchase order number for this order. Formatting Notes: alpha-numeric code. | 
**vendor_order_number** | **String** | The vendor's order number for this order. | 
**acknowledgement_date** | **String** | The date and time when the order is acknowledged, in ISO-8601 date/time format. For example: 2018-07-16T23:00:00Z / 2018-07-16T23:00:00-05:00 / 2018-07-16T23:00:00-08:00. | 
**acknowledgement_status** | [**models::AcknowledgementStatus**](AcknowledgementStatus.md) |  | 
**selling_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**ship_from_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**item_acknowledgements** | [**Vec<models::OrderItemAcknowledgement>**](OrderItemAcknowledgement.md) | Item details including acknowledged quantity. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


