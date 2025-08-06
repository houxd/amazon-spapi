# CreateClaimRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tracking_id** | **String** | The carrier generated identifier for a package in a purchased shipment. | 
**declared_value** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**claim_reason** | [**models::ClaimReason**](ClaimReason.md) |  | 
**is_replacement_package_sent** | Option<**bool**> | Applicable for only On Amazon shipments to identify if replacement was sent | [optional]
**proofs** | Option<**Vec<String>**> | A list of proof URLs for a claim. Basic URL validation will happen for each URLs present in the list | [optional]
**settlement_type** | [**models::SettlementType**](SettlementType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


