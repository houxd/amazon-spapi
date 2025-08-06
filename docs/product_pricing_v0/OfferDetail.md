# OfferDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**my_offer** | Option<**bool**> | When true, this is the seller's offer. | [optional]
**offer_type** | Option<[**models::OfferCustomerType**](OfferCustomerType.md)> |  | [optional]
**sub_condition** | **String** | The subcondition of the item. Subcondition values: New, Mint, Very Good, Good, Acceptable, Poor, Club, OEM, Warranty, Refurbished Warranty, Refurbished, Open Box, or Other. | 
**seller_id** | Option<**String**> | The seller identifier for the offer. | [optional]
**condition_notes** | Option<**String**> | Information about the condition of the item. | [optional]
**seller_feedback_rating** | Option<[**models::SellerFeedbackType**](SellerFeedbackType.md)> |  | [optional]
**shipping_time** | [**models::DetailedShippingTimeType**](DetailedShippingTimeType.md) |  | 
**listing_price** | [**models::MoneyType**](MoneyType.md) |  | 
**quantity_discount_prices** | Option<[**Vec<models::QuantityDiscountPriceType>**](QuantityDiscountPriceType.md)> | List of `QuantityDiscountPrice` that contains item's pricing information when buy in bulk. | [optional]
**points** | Option<[**models::Points**](Points.md)> |  | [optional]
**shipping** | [**models::MoneyType**](MoneyType.md) |  | 
**ships_from** | Option<[**models::ShipsFromType**](ShipsFromType.md)> |  | [optional]
**is_fulfilled_by_amazon** | **bool** | When true, the offer is fulfilled by Amazon. | 
**prime_information** | Option<[**models::PrimeInformationType**](PrimeInformationType.md)> |  | [optional]
**is_buy_box_winner** | Option<**bool**> | When true, the offer is currently in the Buy Box. There can be up to two Buy Box winners at any time per ASIN, one that is eligible for Prime and one that is not eligible for Prime. | [optional]
**is_featured_merchant** | Option<**bool**> | When true, the seller of the item is eligible to win the Buy Box. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


