# InboundShipment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**carrier_code** | Option<[**models::CarrierCode**](CarrierCode.md)> |  | [optional]
**created_at** | Option<**String**> | Timestamp when the shipment was created. The date is returned in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. | [optional]
**destination_address** | [**models::Address**](Address.md) |  | 
**external_reference_id** | Option<**String**> | Client-provided reference ID that can correlate this shipment to client resources. For example, to map this shipment to an internal bookkeeping order record. | [optional]
**order_id** | **String** | The AWD inbound order ID that this inbound shipment belongs to. | 
**origin_address** | [**models::Address**](Address.md) |  | 
**received_quantity** | Option<[**Vec<models::InventoryQuantity>**](InventoryQuantity.md)> | Quantity received (at the receiving end) as part of this shipment. | [optional]
**ship_by** | Option<**String**> | Timestamp when the shipment will be shipped. | [optional]
**shipment_container_quantities** | [**Vec<models::DistributionPackageQuantity>**](DistributionPackageQuantity.md) | Packages that are part of this shipment. | 
**shipment_id** | **String** | Unique shipment ID. | 
**shipment_sku_quantities** | Option<[**Vec<models::SkuQuantity>**](SkuQuantity.md)> | Quantity details at SKU level for the shipment. This attribute will only appear if the skuQuantities parameter in the request is set to SHOW. | [optional]
**destination_region** | Option<**String**> | Assigned region where the order will be shipped. This can differ from what was passed as preference. AWD currently supports following region IDs: [us-west, us-east, us-southcentral, us-southeast] | [optional]
**shipment_status** | [**models::InboundShipmentStatus**](InboundShipmentStatus.md) |  | 
**tracking_id** | Option<**String**> | Carrier-unique tracking ID for this shipment. | [optional]
**updated_at** | Option<**String**> | Timestamp when the shipment was updated. The date is returned in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. | [optional]
**warehouse_reference_id** | Option<**String**> | An AWD-provided reference ID that you can use to interact with the warehouse. For example, a carrier appointment booking. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


