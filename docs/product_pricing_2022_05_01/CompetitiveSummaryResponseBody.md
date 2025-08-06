# CompetitiveSummaryResponseBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | **String** | The ASIN of the item. | 
**marketplace_id** | **String** | The marketplace ID is the globally unique identifier of a marketplace. To find the ID for your marketplace, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | 
**featured_buying_options** | Option<[**Vec<models::FeaturedBuyingOption>**](FeaturedBuyingOption.md)> | A list of featured buying options for the specified ASIN `marketplaceId` combination. | [optional]
**lowest_priced_offers** | Option<[**Vec<models::LowestPricedOffer>**](LowestPricedOffer.md)> | A list of lowest priced offers for the specified ASIN `marketplaceId` combination. | [optional]
**reference_prices** | Option<[**Vec<models::ReferencePrice>**](ReferencePrice.md)> | A list of reference prices for the specified ASIN `marketplaceId` combination. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | A list of error responses that are returned when a request is unsuccessful. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


