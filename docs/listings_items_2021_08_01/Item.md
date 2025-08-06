# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sku** | **String** | A selling partner provided identifier for an Amazon listing. | 
**summaries** | Option<[**Vec<models::ItemSummaryByMarketplace>**](ItemSummaryByMarketplace.md)> | Summary details of a listings item. | [optional]
**attributes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | A JSON object containing structured listings item attribute data keyed by attribute name. | [optional]
**issues** | Option<[**Vec<models::Issue>**](Issue.md)> | The issues associated with the listings item. | [optional]
**offers** | Option<[**Vec<models::ItemOfferByMarketplace>**](ItemOfferByMarketplace.md)> | Offer details for the listings item. | [optional]
**fulfillment_availability** | Option<[**Vec<models::FulfillmentAvailability>**](FulfillmentAvailability.md)> | The fulfillment availability for the listings item. | [optional]
**procurement** | Option<[**Vec<models::ItemProcurement>**](ItemProcurement.md)> | The vendor procurement information for the listings item. | [optional]
**relationships** | Option<[**Vec<models::ItemRelationshipsByMarketplace>**](ItemRelationshipsByMarketplace.md)> | Relationships for a listing item, by marketplace (for example, variations). | [optional]
**product_types** | Option<[**Vec<models::ItemProductTypeByMarketplace>**](ItemProductTypeByMarketplace.md)> | Product types for a listing item, by marketplace. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


