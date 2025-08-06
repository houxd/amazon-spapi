# ItemReviewTopic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**topic** | **String** | The name of the item review topic. | 
**asin_metrics** | [**models::ItemReviewTopicMetrics**](ItemReviewTopicMetrics.md) |  | 
**parent_asin_metrics** | Option<[**models::ItemReviewTopicMetrics**](ItemReviewTopicMetrics.md)> |  | [optional]
**browse_node_metrics** | Option<[**models::ItemReviewBrowseNodeMetrics**](ItemReviewBrowseNodeMetrics.md)> |  | [optional]
**child_asin_metrics** | Option<[**models::ChildAsinMetrics**](ChildAsinMetrics.md)> |  | [optional]
**review_snippets** | Option<**Vec<String>**> | A list of up to three snippets from reviews that contain the topic. This value is `null` if there aren't enough review snippets for the topic. | [optional]
**subtopics** | Option<[**Vec<models::ItemReviewSubtopic>**](ItemReviewSubtopic.md)> | A list of up to five top subtopics for the topic. The percentage of customer reviews that mention the subtopic determine the topic's placement in the list. This value is `null` if there are no subtopics. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


