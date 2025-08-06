# PrepDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label_owner** | Option<[**models::LabelOwner**](LabelOwner.md)> |  | [optional]
**prep_category** | Option<[**models::PrepCategory**](PrepCategory.md)> |  | [optional]
**prep_instructions** | Option<[**Vec<models::PrepInstruction>**](PrepInstruction.md)> | Contains information about the preparation of the inbound products. The system auto-generates this field with the use of the `prepCategory`, and if you attempt to pass a value for this field, the system will ignore it. | [optional]
**prep_owner** | Option<[**models::PrepOwner**](PrepOwner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


