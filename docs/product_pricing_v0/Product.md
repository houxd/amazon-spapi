# Product

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifiers** | [**models::IdentifierType**](IdentifierType.md) |  | 
**attribute_sets** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | A list of product attributes if they are applicable to the product that is returned. | [optional]
**relationships** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | A list that contains product variation information, if applicable. | [optional]
**competitive_pricing** | Option<[**models::CompetitivePricingType**](CompetitivePricingType.md)> |  | [optional]
**sales_rankings** | Option<[**Vec<models::SalesRankType>**](SalesRankType.md)> | A list of sales rank information for the item, by category. | [optional]
**offers** | Option<[**Vec<models::OfferType>**](OfferType.md)> | A list of offers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


