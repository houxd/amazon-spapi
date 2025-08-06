# StandardComparisonProductBlock

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**position** | **i32** | The rank or index of this comparison product block within the module. Different blocks cannot occupy the same position within a single module. | 
**image** | Option<[**models::ImageComponent**](ImageComponent.md)> |  | [optional]
**title** | Option<**String**> | The comparison product title. | [optional]
**asin** | Option<**String**> | The Amazon Standard Identification Number (ASIN). | [optional]
**highlight** | Option<**bool**> | When true, indicates that this content block is visually highlighted. | [optional]
**metrics** | Option<[**Vec<models::PlainTextItem>**](PlainTextItem.md)> | Comparison metrics for the product. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


