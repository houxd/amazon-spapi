# ListOffersRequestFilters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | The marketplace identifier. The supported marketplaces for both sellers and vendors are US, CA, ES, UK, FR, IT, IN, DE and JP. The supported marketplaces for vendors only are BR, AU, MX, AE and NL. Refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids) to find the identifier for the marketplace. | 
**skus** | Option<**Vec<String>**> | A list of SKUs to filter. This filter is only supported for sellers and not for vendors. | [optional]
**asins** | Option<**Vec<String>**> | A list of Amazon Standard Identification Numbers (ASINs). | [optional]
**eligibilities** | Option<[**Vec<models::EligibilityStatus>**](EligibilityStatus.md)> | A list of eligibilities associated with an offer. | [optional]
**preferences** | Option<[**models::Preference**](Preference.md)> |  | [optional]
**promotions** | Option<[**models::Promotion**](Promotion.md)> |  | [optional]
**program_types** | [**Vec<models::ProgramType>**](ProgramType.md) | A list of replenishment program types. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


