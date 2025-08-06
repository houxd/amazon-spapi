# Summary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_offer_count** | **i32** | The number of unique offers contained in NumberOfOffers. | 
**number_of_offers** | Option<[**Vec<models::OfferCountType>**](OfferCountType.md)> | A list that contains the total number of offers information for given conditions and fulfillment channels. | [optional]
**lowest_prices** | Option<[**Vec<models::LowestPriceType>**](LowestPriceType.md)> | A list of the lowest prices. | [optional]
**buy_box_prices** | Option<[**Vec<models::BuyBoxPriceType>**](BuyBoxPriceType.md)> | A list of the Buy Box prices. | [optional]
**list_price** | Option<[**models::MoneyType**](MoneyType.md)> |  | [optional]
**competitive_price_threshold** | Option<[**models::MoneyType**](MoneyType.md)> |  | [optional]
**suggested_lower_price_plus_shipping** | Option<[**models::MoneyType**](MoneyType.md)> |  | [optional]
**sales_rankings** | Option<[**Vec<models::SalesRankType>**](SalesRankType.md)> | A list of sales rank information for the item, by category. | [optional]
**buy_box_eligible_offers** | Option<[**Vec<models::OfferCountType>**](OfferCountType.md)> | A list that contains the total number of offers that are eligible for the Buy Box for the given conditions and fulfillment channels. | [optional]
**offers_available_time** | Option<**String**> | When the status is ActiveButTooSoonForProcessing, this is the time when the offers will be available for processing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


