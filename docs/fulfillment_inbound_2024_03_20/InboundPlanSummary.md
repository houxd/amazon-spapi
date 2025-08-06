# InboundPlanSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | The time at which the inbound plan was created. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) datetime format with pattern `yyyy-MM-ddTHH:mm:ssZ`. | 
**inbound_plan_id** | **String** | Identifier of an inbound plan. | 
**last_updated_at** | **String** | The time at which the inbound plan was last updated. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) datetime format with pattern `yyyy-MM-ddTHH:mm:ssZ`. | 
**marketplace_ids** | **Vec<String>** | A list of marketplace IDs. | 
**name** | **String** | Human-readable name of the inbound plan. | 
**source_address** | [**models::Address**](Address.md) |  | 
**status** | **String** | The current status of the inbound plan. Possible values: `ACTIVE`, `VOIDED`, `SHIPPED`, `ERRORED`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


