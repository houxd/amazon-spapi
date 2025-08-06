# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | **String** | Amazon Standard Identification Number (ASIN) is the unique identifier for an item in the Amazon catalog. | 
**attributes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | A JSON object that contains structured item attribute data keyed by attribute name. Catalog item attributes are available only to brand owners and conform to the related product type definitions available in the Selling Partner API for Product Type Definitions. | [optional]
**identifiers** | Option<[**Vec<models::ItemIdentifiersByMarketplace>**](ItemIdentifiersByMarketplace.md)> | Identifiers associated with the item in the Amazon catalog, such as UPC and EAN identifiers. | [optional]
**images** | Option<[**Vec<models::ItemImagesByMarketplace>**](ItemImagesByMarketplace.md)> | Images for an item in the Amazon catalog. All image variants are provided to brand owners. Otherwise, a thumbnail of the \"MAIN\" image variant is provided. | [optional]
**product_types** | Option<[**Vec<models::ItemProductTypeByMarketplace>**](ItemProductTypeByMarketplace.md)> | Product types associated with the Amazon catalog item. | [optional]
**sales_ranks** | Option<[**Vec<models::ItemSalesRanksByMarketplace>**](ItemSalesRanksByMarketplace.md)> | Sales ranks of an Amazon catalog item. | [optional]
**summaries** | Option<[**Vec<models::ItemSummaryByMarketplace>**](ItemSummaryByMarketplace.md)> | Summary details of an Amazon catalog item. | [optional]
**variations** | Option<[**Vec<models::ItemVariationsByMarketplace>**](ItemVariationsByMarketplace.md)> | Variation details by marketplace for an Amazon catalog item (variation relationships). | [optional]
**vendor_details** | Option<[**Vec<models::ItemVendorDetailsByMarketplace>**](ItemVendorDetailsByMarketplace.md)> | Vendor details associated with an Amazon catalog item. Vendor details are available to vendors only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


