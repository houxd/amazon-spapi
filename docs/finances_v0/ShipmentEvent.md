# ShipmentEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | Option<**String**> | An Amazon-defined identifier for an order. | [optional]
**seller_order_id** | Option<**String**> | A seller-defined identifier for an order. | [optional]
**marketplace_name** | Option<**String**> | The name of the marketplace where the event occurred. | [optional]
**store_name** | Option<**String**> | The name of the store where the event occurred. | [optional]
**order_charge_list** | Option<[**Vec<models::ChargeComponent>**](ChargeComponent.md)> | A list of charge information on the seller's account. | [optional]
**order_charge_adjustment_list** | Option<[**Vec<models::ChargeComponent>**](ChargeComponent.md)> | A list of charge information on the seller's account. | [optional]
**shipment_fee_list** | Option<[**Vec<models::FeeComponent>**](FeeComponent.md)> | A list of fee component information. | [optional]
**shipment_fee_adjustment_list** | Option<[**Vec<models::FeeComponent>**](FeeComponent.md)> | A list of fee component information. | [optional]
**order_fee_list** | Option<[**Vec<models::FeeComponent>**](FeeComponent.md)> | A list of fee component information. | [optional]
**order_fee_adjustment_list** | Option<[**Vec<models::FeeComponent>**](FeeComponent.md)> | A list of fee component information. | [optional]
**direct_payment_list** | Option<[**Vec<models::DirectPayment>**](DirectPayment.md)> | A list of direct payment information. | [optional]
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**shipment_item_list** | Option<[**Vec<models::ShipmentItem>**](ShipmentItem.md)> | A list of shipment items. | [optional]
**shipment_item_adjustment_list** | Option<[**Vec<models::ShipmentItem>**](ShipmentItem.md)> | A list of shipment items. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


