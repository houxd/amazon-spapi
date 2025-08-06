# CreateMarketplaceItemLabelsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**height** | Option<**f64**> | The height of the item label. | [optional]
**label_type** | [**models::LabelPrintType**](LabelPrintType.md) |  | 
**locale_code** | Option<**String**> | The locale code constructed from ISO 639 language code and ISO 3166-1 alpha-2 standard of country codes separated by an underscore character. | [optional][default to en_US]
**marketplace_id** | **String** | The Marketplace ID. For a list of possible values, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | 
**msku_quantities** | [**Vec<models::MskuQuantity>**](MskuQuantity.md) | Represents the quantity of an MSKU to print item labels for. | 
**page_type** | Option<[**models::ItemLabelPageType**](ItemLabelPageType.md)> |  | [optional]
**width** | Option<**f64**> | The width of the item label. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


