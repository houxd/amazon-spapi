# ItemReviewTopics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**positive_topics** | Option<[**Vec<models::ItemReviewTopic>**](ItemReviewTopic.md)> | A list of the most positive review topics. When the `sortBy` query parameter is set to `MENTIONS`, the number of reviews that mention the topic determines the topic's placement in the list. When `sortBy` is set to `STAR_RATING_IMPACT`, the effect that the topic has on the star rating of the item determines placement in the list. This value is `null` if there are not enough positive reviews for the specified ASIN.  **Max length:** 10 | [optional]
**negative_topics** | Option<[**Vec<models::ItemReviewTopic>**](ItemReviewTopic.md)> | A list of the most negative review topics. When the `sortBy` query parameter is set to `MENTIONS`, the number of reviews that mention the topic determines the topic's placement in the list. When `sortBy` is set to `STAR_RATING_IMPACT`, the effect that the topic has on the star rating of the item determines placement in the list. This value is `null` if there are not enough negative reviews for the specified ASIN.  **Max length:** 10 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


