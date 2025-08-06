# Business

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The registered business name. | 
**registered_business_address** | [**models::Address**](Address.md) |  | 
**company_registration_number** | Option<**String**> | The seller's company registration number, if applicable. This field will be absent for individual sellers and sole proprietorships. | [optional]
**company_tax_identification_number** | Option<**String**> | The seller's company tax identification number, if applicable. This field will be present for certain business types only, such as sole proprietorships. | [optional]
**non_latin_name** | Option<**String**> | The non-Latin script version of the registered business name, if applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


