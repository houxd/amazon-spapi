# GetFeatureSkuResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | The requested marketplace. | 
**feature_name** | **String** | The name of the feature. | 
**is_eligible** | **bool** | When true, the seller SKU is eligible for the requested feature. | 
**ineligible_reasons** | Option<**Vec<String>**> | A list of one or more reasons that the seller SKU is ineligibile for the feature.  Possible values: * `MERCHANT_NOT_ENROLLED` - The merchant isn't enrolled for the feature. * `SKU_NOT_ELIGIBLE` - The SKU doesn't reside in a warehouse that supports the feature. * `INVALID_SKU` - There is an issue with the SKU provided. | [optional]
**sku_info** | Option<[**models::FeatureSku**](FeatureSku.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


