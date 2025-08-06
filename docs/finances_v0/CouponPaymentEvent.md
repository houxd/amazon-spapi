# CouponPaymentEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**coupon_id** | Option<**String**> | A coupon identifier. | [optional]
**seller_coupon_description** | Option<**String**> | The description provided by the seller when they created the coupon. | [optional]
**clip_or_redemption_count** | Option<**i64**> | The number of coupon clips or redemptions. | [optional]
**payment_event_id** | Option<**String**> | A payment event identifier. | [optional]
**fee_component** | Option<[**models::FeeComponent**](FeeComponent.md)> |  | [optional]
**charge_component** | Option<[**models::ChargeComponent**](ChargeComponent.md)> |  | [optional]
**total_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


