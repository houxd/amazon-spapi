# CompetitivePriceType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**competitive_price_id** | **String** | The pricing model for each price that is returned.  Possible values:  * 1 - New Buy Box Price. * 2 - Used Buy Box Price. | 
**price** | [**models::PriceType**](PriceType.md) |  | 
**condition** | Option<**String**> | Indicates the condition of the item whose pricing information is returned. Possible values are: New, Used, Collectible, Refurbished, or Club. | [optional]
**subcondition** | Option<**String**> | Indicates the subcondition of the item whose pricing information is returned. Possible values are: New, Mint, Very Good, Good, Acceptable, Poor, Club, OEM, Warranty, Refurbished Warranty, Refurbished, Open Box, or Other. | [optional]
**offer_type** | Option<[**models::OfferCustomerType**](OfferCustomerType.md)> |  | [optional]
**quantity_tier** | Option<**i32**> | Indicates at what quantity this price becomes active. | [optional]
**quantity_discount_type** | Option<[**models::QuantityDiscountType**](QuantityDiscountType.md)> |  | [optional]
**seller_id** | Option<**String**> | The seller identifier for the offer. | [optional]
**belongs_to_requester** | Option<**bool**> |  Indicates whether or not the pricing information is for an offer listing that belongs to the requester. The requester is the seller associated with the SellerId that was submitted with the request. Possible values are: true and false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


