# InboundShipmentSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Timestamp when the shipment was created. | [optional]
**external_reference_id** | Option<**String**> | Optional client-provided reference ID that can be used to correlate this shipment with client resources. For example, to map this shipment to an internal bookkeeping order record. | [optional]
**order_id** | **String** | The AWD inbound order ID that this inbound shipment belongs to. | 
**shipment_id** | **String** | A unique shipment ID. | 
**shipment_status** | [**models::InboundShipmentStatus**](InboundShipmentStatus.md) |  | 
**updated_at** | Option<**String**> | Timestamp when the shipment was updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


