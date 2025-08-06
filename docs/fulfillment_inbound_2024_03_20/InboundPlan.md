# InboundPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | The time at which the inbound plan was created. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) datetime with pattern `yyyy-MM-ddTHH:mm:ssZ`. | 
**inbound_plan_id** | **String** | Identifier of an inbound plan. | 
**last_updated_at** | **String** | The time at which the inbound plan was last updated. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) datetime format with pattern `yyyy-MM-ddTHH:mm:ssZ`. | 
**marketplace_ids** | **Vec<String>** | A list of marketplace IDs. | 
**name** | **String** | Human-readable name of the inbound plan. | 
**packing_options** | Option<[**Vec<models::PackingOptionSummary>**](PackingOptionSummary.md)> | Packing options for the inbound plan. This property will be populated when it has been generated via the corresponding operation. If there is a chosen placement option, only packing options for that placement option will be returned. If there are confirmed shipments, only packing options for those shipments will be returned. Query the packing option for more details. | [optional]
**placement_options** | Option<[**Vec<models::PlacementOptionSummary>**](PlacementOptionSummary.md)> | Placement options for the inbound plan. This property will be populated when it has been generated via the corresponding operation. If there is a chosen placement option, that will be the only returned option. Query the placement option for more details. | [optional]
**shipments** | Option<[**Vec<models::ShipmentSummary>**](ShipmentSummary.md)> | A list of shipment IDs for the inbound plan. This property is populated when it has been generated with the `confirmPlacementOptions` operation. Only shipments from the chosen placement option are returned. Query the shipment for more details. | [optional]
**source_address** | [**models::Address**](Address.md) |  | 
**status** | **String** | Current status of the inbound plan. Possible values: `ACTIVE`, `VOIDED`, `SHIPPED`, `ERRORED`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


