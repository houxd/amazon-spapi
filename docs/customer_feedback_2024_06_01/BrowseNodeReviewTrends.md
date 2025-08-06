# BrowseNodeReviewTrends

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**positive_topics** | Option<[**Vec<models::BrowseNodeReviewTrend>**](BrowseNodeReviewTrend.md)> | The trends of the most positive review topics. The percentage of reviews that contain the topic across all products in the requested browse node determine the topic's placement in the list. This value is `null` if there aren't enough positive reviews for the requested browse node.  **Max length:** 10 | [optional]
**negative_topics** | Option<[**Vec<models::BrowseNodeReviewTrend>**](BrowseNodeReviewTrend.md)> | The trends of the most negative review topics. The percentage of reviews that contain the topic across all products in the requested browse node determine the topic's placement in the list. This value is `null` if there aren't enough positive reviews for the requested browse node.  **Max length:** 10 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


