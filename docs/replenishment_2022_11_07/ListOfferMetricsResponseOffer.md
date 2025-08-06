# ListOfferMetricsResponseOffer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | Option<**String**> | The Amazon Standard Identification Number (ASIN). | [optional]
**not_delivered_due_to_oos** | Option<**f64**> | The percentage of items that were not shipped out of the total shipped units over a period of time due to being out of stock. Applicable to PERFORMANCE timePeriodType. | [optional]
**total_subscriptions_revenue** | Option<**f64**> | The revenue generated from subscriptions over a period of time. Applicable to PERFORMANCE timePeriodType. | [optional]
**shipped_subscription_units** | Option<**f64**> | The number of units shipped to the subscribers over a period of time. Applicable to PERFORMANCE timePeriodType. | [optional]
**active_subscriptions** | Option<**f64**> | The number of active subscriptions present at the end of the period. Applicable to PERFORMANCE timePeriodType. | [optional]
**revenue_penetration** | Option<**f64**> | The percentage of total program revenue out of total product revenue. Applicable to PERFORMANCE timePeriodType. | [optional]
**lost_revenue_due_to_oos** | Option<**f64**> | The revenue that would have been generated had there not been out of stock. Applicable to PERFORMANCE timePeriodType. | [optional]
**coupons_revenue_penetration** | Option<**f64**> | The percentage of revenue from ASINs with coupons out of total revenue from all ASINs. Applicable to PERFORMANCE timePeriodType. | [optional]
**share_of_coupon_subscriptions** | Option<**f64**> | The percentage of new subscriptions acquired through coupons. Applicable to PERFORMANCE timePeriodType. | [optional]
**next30_day_total_subscriptions_revenue** | Option<**f64**> | The forecasted total subscription revenue for the next 30 days. Applicable to FORECAST timePeriodType. | [optional]
**next60_day_total_subscriptions_revenue** | Option<**f64**> | The forecasted total subscription revenue for the next 60 days. Applicable to FORECAST timePeriodType. | [optional]
**next90_day_total_subscriptions_revenue** | Option<**f64**> | The forecasted total subscription revenue for the next 90 days. Applicable to FORECAST timePeriodType. | [optional]
**next30_day_shipped_subscription_units** | Option<**f64**> | The forecasted shipped subscription units for the next 30 days. Applicable to FORECAST timePeriodType. | [optional]
**next60_day_shipped_subscription_units** | Option<**f64**> | The forecasted shipped subscription units for the next 60 days. Applicable to FORECAST timePeriodType. | [optional]
**next90_day_shipped_subscription_units** | Option<**f64**> | The forecasted shipped subscription units for the next 90 days. Applicable to FORECAST timePeriodType. | [optional]
**time_interval** | Option<[**models::TimeInterval**](TimeInterval.md)> |  | [optional]
**currency_code** | Option<**String**> | The currency code in ISO 4217 format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


