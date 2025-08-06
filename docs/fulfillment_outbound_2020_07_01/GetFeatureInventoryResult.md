# GetFeatureInventoryResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | The requested marketplace. | 
**feature_name** | **String** | The name of the feature. | 
**next_token** | Option<**String**> | When present and not empty, pass this string token in the next request to return the next response page. | [optional]
**feature_skus** | Option<[**Vec<models::FeatureSku>**](FeatureSku.md)> | An array of SKUs eligible for this feature and the quantity available. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


