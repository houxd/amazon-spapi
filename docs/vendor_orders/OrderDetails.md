# OrderDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_date** | **String** | The date the purchase order was placed. Must be in ISO-8601 date/time format. | 
**purchase_order_changed_date** | Option<**String**> | The date when purchase order was last changed by Amazon after the order was placed. This date will be greater than 'purchaseOrderDate'. This means the PO data was changed on that date and vendors are required to fulfill the  updated PO. The PO changes can be related to Item Quantity, Ship to Location, Ship Window etc. This field will not be present in orders that have not changed after creation. Must be in ISO-8601 date/time format. | [optional]
**purchase_order_state_changed_date** | **String** | The date when current purchase order state was changed. Current purchase order state is available in the field 'purchaseOrderState'. Must be in ISO-8601 date/time format. | 
**purchase_order_type** | Option<**String**> | Type of purchase order. | [optional]
**import_details** | Option<[**models::ImportDetails**](ImportDetails.md)> |  | [optional]
**deal_code** | Option<**String**> | If requested by the recipient, this field will contain a promotional/deal number. The discount code line is optional. It is used to obtain a price discount on items on the order. | [optional]
**payment_method** | Option<**String**> | Payment method used. | [optional]
**buying_party** | Option<[**models::PartyIdentification**](PartyIdentification.md)> |  | [optional]
**selling_party** | Option<[**models::PartyIdentification**](PartyIdentification.md)> |  | [optional]
**ship_to_party** | Option<[**models::PartyIdentification**](PartyIdentification.md)> |  | [optional]
**bill_to_party** | Option<[**models::PartyIdentification**](PartyIdentification.md)> |  | [optional]
**ship_window** | Option<**String**> | Defines a date time interval according to ISO8601. Interval is separated by double hyphen (--). | [optional]
**delivery_window** | Option<**String**> | Defines a date time interval according to ISO8601. Interval is separated by double hyphen (--). | [optional]
**items** | [**Vec<models::OrderItem>**](OrderItem.md) | A list of items in this purchase order. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


