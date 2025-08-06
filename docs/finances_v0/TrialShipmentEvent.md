# TrialShipmentEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | Option<**String**> | An Amazon-defined identifier for an order. | [optional]
**financial_event_group_id** | Option<**String**> | The identifier of the financial event group. | [optional]
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**sku** | Option<**String**> | The seller SKU of the item. The seller SKU is qualified by the seller's seller ID, which is included with every call to the Selling Partner API. | [optional]
**fee_list** | Option<[**Vec<models::FeeComponent>**](FeeComponent.md)> | A list of fee component information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


