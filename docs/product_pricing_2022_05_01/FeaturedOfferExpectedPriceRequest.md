# FeaturedOfferExpectedPriceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uri** | **String** | The URI associated with an individual request within a batch. For `FeaturedOfferExpectedPrice`, this is `/products/pricing/2022-05-01/offer/featuredOfferExpectedPrice`. | 
**method** | [**models::HttpMethod**](HttpMethod.md) |  | 
**body** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Additional HTTP body information that is associated with an individual request within a batch. | [optional]
**headers** | Option<**std::collections::HashMap<String, String>**> | A mapping of additional HTTP headers to send or receive for an individual request within a batch. | [optional]
**marketplace_id** | **String** | The marketplace ID is the globally unique identifier of a marketplace. To find the ID for your marketplace, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | 
**sku** | **String** | The seller SKU of the item. | 
**segment** | Option<[**models::Segment**](Segment.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


