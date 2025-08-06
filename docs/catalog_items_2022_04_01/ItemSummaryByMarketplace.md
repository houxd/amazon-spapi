# ItemSummaryByMarketplace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | Amazon marketplace identifier. To find the ID for your marketplace, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | 
**adult_product** | Option<**bool**> | When `true`, the Amazon catalog item is intended for an adult audience or is sexual in nature. | [optional]
**autographed** | Option<**bool**> | When `true`, the Amazon catalog item is autographed. | [optional]
**brand** | Option<**String**> | Name of the brand that is associated with the Amazon catalog item. | [optional]
**browse_classification** | Option<[**models::ItemBrowseClassification**](ItemBrowseClassification.md)> |  | [optional]
**color** | Option<**String**> | The color that is associated with the Amazon catalog item. | [optional]
**contributors** | Option<[**Vec<models::ItemContributor>**](ItemContributor.md)> | Individual contributors to the creation of the item, such as the authors or actors. | [optional]
**item_classification** | Option<**String**> | Classification type that is associated with the Amazon catalog item. | [optional]
**item_name** | Option<**String**> | The name that is associated with the Amazon catalog item. | [optional]
**manufacturer** | Option<**String**> | The name of the manufacturer that is associated with the Amazon catalog item. | [optional]
**memorabilia** | Option<**bool**> | When true, the item is classified as memorabilia. | [optional]
**model_number** | Option<**String**> | The model number that is associated with the Amazon catalog item. | [optional]
**package_quantity** | Option<**i32**> | The quantity of the Amazon catalog item within one package. | [optional]
**part_number** | Option<**String**> | The part number that is associated with the Amazon catalog item. | [optional]
**release_date** | Option<[**String**](string.md)> | The earliest date on which the Amazon catalog item can be shipped to customers. | [optional]
**size** | Option<**String**> | The name of the size of the Amazon catalog item. | [optional]
**style** | Option<**String**> | The name of the style that is associated with the Amazon catalog item. | [optional]
**trade_in_eligible** | Option<**bool**> | When true, the Amazon catalog item is eligible for trade-in. | [optional]
**website_display_group** | Option<**String**> | The identifier of the website display group that is associated with the Amazon catalog item. | [optional]
**website_display_group_name** | Option<**String**> | The display name of the website display group that is associated with the Amazon catalog item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


