# CompetitiveSummaryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | **String** | The ASIN of the item. | 
**marketplace_id** | **String** | The marketplace ID is the globally unique identifier of a marketplace. To find the ID for your marketplace, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | 
**included_data** | [**Vec<models::CompetitiveSummaryIncludedData>**](CompetitiveSummaryIncludedData.md) | The list of requested competitive pricing data for the product. | 
**lowest_priced_offers_inputs** | Option<[**Vec<models::LowestPricedOffersInput>**](LowestPricedOffersInput.md)> | The list of `lowestPricedOffersInput` parameters that are used to build `lowestPricedOffers` in the response. This attribute is only valid if `lowestPricedOffers` is requested in `includedData` | [optional]
**method** | [**models::HttpMethod**](HttpMethod.md) |  | 
**uri** | **String** | The URI associated with the individual APIs that are called as part of the batch request. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


