# PlacementOption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**discounts** | [**Vec<models::Incentive>**](Incentive.md) | Discount for the offered option. | 
**expiration** | Option<**String**> | The expiration date of the placement option. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) datetime format with pattern `yyyy-MM-ddTHH:mm:ss.sssZ`. | [optional]
**fees** | [**Vec<models::Incentive>**](Incentive.md) | The fee for the offered option. | 
**placement_option_id** | **String** | The identifier of a placement option. A placement option represents the shipment splits and destinations of SKUs. | 
**shipment_ids** | **Vec<String>** | Shipment ids. | 
**status** | **String** | The status of a placement option. Possible values: `OFFERED`, `ACCEPTED`, `EXPIRED`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


