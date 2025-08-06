# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | **String** | The unique identifier of an item in the Amazon catalog. | 
**attributes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | A JSON object containing structured item attribute data that is keyed by attribute name. Catalog item attributes conform to the related Amazon product type definitions that you can get from the [Product Type Definitions API](https://developer-docs.amazon.com/sp-api/reference/product-type-definitions-v2020-09-01). | [optional]
**classifications** | Option<[**Vec<models::ItemBrowseClassificationsByMarketplace>**](ItemBrowseClassificationsByMarketplace.md)> | An array of classifications (browse nodes) that is associated with the item in the Amazon catalog, grouped by `marketplaceId`. | [optional]
**dimensions** | Option<[**Vec<models::ItemDimensionsByMarketplace>**](ItemDimensionsByMarketplace.md)> | An array of dimensions that are associated with the item in the Amazon catalog, grouped by `marketplaceId`. | [optional]
**identifiers** | Option<[**Vec<models::ItemIdentifiersByMarketplace>**](ItemIdentifiersByMarketplace.md)> | Identifiers associated with the item in the Amazon catalog, such as UPC and EAN identifiers. | [optional]
**images** | Option<[**Vec<models::ItemImagesByMarketplace>**](ItemImagesByMarketplace.md)> | The images for an item in the Amazon catalog. | [optional]
**product_types** | Option<[**Vec<models::ItemProductTypeByMarketplace>**](ItemProductTypeByMarketplace.md)> | Product types that are associated with the Amazon catalog item. | [optional]
**relationships** | Option<[**Vec<models::ItemRelationshipsByMarketplace>**](ItemRelationshipsByMarketplace.md)> | Relationships grouped by `marketplaceId` for an Amazon catalog item (for example, variations). | [optional]
**sales_ranks** | Option<[**Vec<models::ItemSalesRanksByMarketplace>**](ItemSalesRanksByMarketplace.md)> | Sales ranks of an Amazon catalog item. | [optional]
**summaries** | Option<[**Vec<models::ItemSummaryByMarketplace>**](ItemSummaryByMarketplace.md)> | Summaries of Amazon catalog items. | [optional]
**vendor_details** | Option<[**Vec<models::ItemVendorDetailsByMarketplace>**](ItemVendorDetailsByMarketplace.md)> | The vendor details that are associated with an Amazon catalog item. Vendor details are only available to vendors. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


