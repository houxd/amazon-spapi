# \DefinitionsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_definitions_product_type**](DefinitionsApi.md#get_definitions_product_type) | **GET** /definitions/2020-09-01/productTypes/{productType} | 
[**search_definitions_product_types**](DefinitionsApi.md#search_definitions_product_types) | **GET** /definitions/2020-09-01/productTypes | 



## get_definitions_product_type

> models::ProductTypeDefinition get_definitions_product_type(product_type, marketplace_ids, seller_id, product_type_version, requirements, requirements_enforced, locale)


Retrieve an Amazon product type definition.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_type** | **String** | The Amazon product type name. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. Note: This parameter is limited to one marketplaceId at this time. | [required] |
**seller_id** | Option<**String**> | A selling partner identifier. When provided, seller-specific requirements and values are populated within the product type definition schema, such as brand names associated with the selling partner. |  |
**product_type_version** | Option<**String**> | The version of the Amazon product type to retrieve. Defaults to \"LATEST\",. Prerelease versions of product type definitions may be retrieved with \"RELEASE_CANDIDATE\". If no prerelease version is currently available, the \"LATEST\" live version will be provided. |  |[default to LATEST]
**requirements** | Option<**String**> | The name of the requirements set to retrieve requirements for. |  |[default to LISTING]
**requirements_enforced** | Option<**String**> | Identifies if the required attributes for a requirements set are enforced by the product type definition schema. Non-enforced requirements enable structural validation of individual attributes without all the required attributes being present (such as for partial updates). |  |[default to ENFORCED]
**locale** | Option<**String**> | Locale for retrieving display labels and other presentation details. Defaults to the default language of the first marketplace in the request. |  |[default to DEFAULT]

### Return type

[**models::ProductTypeDefinition**](ProductTypeDefinition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_definitions_product_types

> models::ProductTypeList search_definitions_product_types(marketplace_ids, keywords, item_name, locale, search_locale)


Search for and return a list of Amazon product types that have definitions available.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**keywords** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of keywords to search product types. **Note:** Cannot be used with `itemName`. |  |
**item_name** | Option<**String**> | The title of the ASIN to get the product type recommendation. **Note:** Cannot be used with `keywords`. |  |
**locale** | Option<**String**> | The locale for the display names in the response. Defaults to the primary locale of the marketplace. |  |
**search_locale** | Option<**String**> | The locale used for the `keywords` and `itemName` parameters. Defaults to the primary locale of the marketplace. |  |

### Return type

[**models::ProductTypeList**](ProductTypeList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

