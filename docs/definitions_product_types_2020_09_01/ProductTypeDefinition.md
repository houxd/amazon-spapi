# ProductTypeDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta_schema** | Option<[**models::SchemaLink**](SchemaLink.md)> |  | [optional]
**schema** | [**models::SchemaLink**](SchemaLink.md) |  | 
**requirements** | **String** | Name of the requirements set represented in this product type definition. | 
**requirements_enforced** | **String** | Identifies if the required attributes for a requirements set are enforced by the product type definition schema. Non-enforced requirements enable structural validation of individual attributes without all of the required attributes being present (such as for partial updates). | 
**property_groups** | [**std::collections::HashMap<String, models::PropertyGroup>**](PropertyGroup.md) | Mapping of property group names to property groups. Property groups represent logical groupings of schema properties that can be used for display or informational purposes. | 
**locale** | **String** | Locale of the display elements contained in the product type definition. | 
**marketplace_ids** | **Vec<String>** | Amazon marketplace identifiers for which the product type definition is applicable. | 
**product_type** | **String** | The name of the Amazon product type that this product type definition applies to. | 
**display_name** | **String** | Human-readable and localized description of the Amazon product type. | 
**product_type_version** | [**models::ProductTypeVersion**](ProductTypeVersion.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


