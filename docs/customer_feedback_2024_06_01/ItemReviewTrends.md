# ItemReviewTrends

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**positive_topics** | Option<[**Vec<models::ItemReviewTrend>**](ItemReviewTrend.md)> | A list of the most positive review topics. The percentage of reviews that contain the topic determines the topic's placement in the list. This value is `null` if there are not enough positive reviews for the specified ASIN.  **Max length:** 10 | [optional]
**negative_topics** | Option<[**Vec<models::ItemReviewTrend>**](ItemReviewTrend.md)> | A list of the most negative review topics. The percentage of reviews that contain the topic determines the topic's placement in the list. This value is `null` if there are not enough negative reviews for the specified ASIN.  **Max length:** 10 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


