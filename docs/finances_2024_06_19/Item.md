# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description of items in the transaction | [optional]
**related_identifiers** | Option<[**Vec<models::ItemRelatedIdentifier>**](ItemRelatedIdentifier.md)> | Related Business identifiers of the item in Transaction. | [optional]
**total_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**breakdowns** | Option<[**Vec<models::Breakdown>**](Breakdown.md)> | A list of breakdowns that detail how the total amount is calculated for the transaction. | [optional]
**contexts** | Option<[**Vec<models::Context>**](Context.md)> | List of additional Information about the item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


