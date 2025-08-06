# ItemSalesRanksByMarketplace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | Amazon marketplace identifier. To find the ID for your marketplace, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | 
**classification_ranks** | Option<[**Vec<models::ItemClassificationSalesRank>**](ItemClassificationSalesRank.md)> | Sales ranks of an Amazon catalog item for a `marketplaceId`, grouped by classification. | [optional]
**display_group_ranks** | Option<[**Vec<models::ItemDisplayGroupSalesRank>**](ItemDisplayGroupSalesRank.md)> | Sales ranks of an Amazon catalog item for a `marketplaceId`, grouped by website display group. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


