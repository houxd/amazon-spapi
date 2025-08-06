# InventoryListing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inventory** | [**Vec<models::InventorySummary>**](InventorySummary.md) | List of inventory summaries. | 
**next_token** | Option<**String**> | A token that is used to retrieve the next page of results. The response includes `nextToken` when the number of results exceeds the specified `maxResults` value. To get the next page of results, call the operation with this token and include the same arguments as the call that produced the token. To get a complete list, call this operation until `nextToken` is null. Note that this operation can return empty pages. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


