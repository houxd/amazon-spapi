# BrowseNodeReviewTopic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**topic** | **String** | The name browse node review topic. | 
**browse_node_metrics** | [**models::BrowseNodeReviewTopicMetrics**](BrowseNodeReviewTopicMetrics.md) |  | 
**review_snippets** | Option<**Vec<String>**> | A list of up to three snippets from reviews that contain the topic. This value is `null` if there aren't enough review snippets for the topic. | [optional]
**subtopics** | Option<[**Vec<models::BrowseNodeSubtopic>**](BrowseNodeSubtopic.md)> | A list of the five subtopics that occur most frequently. This value is `null` if there are no subtopics. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


