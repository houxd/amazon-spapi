# Issue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | An issue code that identifies the type of issue. | 
**message** | **String** | A message that describes the issue. | 
**severity** | **String** | The severity of the issue. | 
**attribute_names** | Option<**Vec<String>**> | The names of the attributes associated with the issue, if applicable. | [optional]
**categories** | **Vec<String>** | List of issue categories.   Possible values:   * 'INVALID_ATTRIBUTE' - Indicating an invalid attribute in the listing.   * 'MISSING_ATTRIBUTE' - Highlighting a missing attribute in the listing.   * 'INVALID_IMAGE' - Signifying an invalid image in the listing.   * 'MISSING_IMAGE' - Noting the absence of an image in the listing.   * 'INVALID_PRICE' - Pertaining to issues with the listing's price-related attributes.   * 'MISSING_PRICE' - Pointing out the absence of a price attribute in the listing.   * 'DUPLICATE' - Identifying listings with potential duplicate problems, such as this ASIN potentially being a duplicate of another ASIN.   * 'QUALIFICATION_REQUIRED' - Indicating that the listing requires qualification-related approval. | 
**enforcements** | Option<[**models::IssueEnforcements**](IssueEnforcements.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


