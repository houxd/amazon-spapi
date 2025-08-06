# SegmentedFeaturedOffer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_id** | **String** | The seller identifier for the offer. | 
**condition** | [**models::Condition**](Condition.md) |  | 
**sub_condition** | Option<**String**> | The item subcondition of the offer. | [optional]
**fulfillment_type** | [**models::FulfillmentType**](FulfillmentType.md) |  | 
**listing_price** | [**models::MoneyType**](MoneyType.md) |  | 
**shipping_options** | Option<[**Vec<models::ShippingOption>**](ShippingOption.md)> | A list of shipping options associated with this offer | [optional]
**points** | Option<[**models::Points**](Points.md)> |  | [optional]
**prime_details** | Option<[**models::PrimeDetails**](PrimeDetails.md)> |  | [optional]
**featured_offer_segments** | [**Vec<models::FeaturedOfferSegment>**](FeaturedOfferSegment.md) | The list of segment information in which the offer is featured. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


