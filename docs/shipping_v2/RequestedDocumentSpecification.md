# RequestedDocumentSpecification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format** | [**models::DocumentFormat**](DocumentFormat.md) |  | 
**size** | [**models::DocumentSize**](DocumentSize.md) |  | 
**dpi** | Option<**i32**> | The dots per inch (DPI) value used in printing. This value represents a measure of the resolution of the document. | [optional]
**page_layout** | Option<**String**> | Indicates the position of the label on the paper. Should be the same value as returned in getRates response. | [optional]
**need_file_joining** | **bool** | When true, files should be stitched together. Otherwise, files should be returned separately. Defaults to false. | 
**requested_document_types** | [**Vec<models::DocumentType>**](DocumentType.md) | A list of the document types requested. | 
**requested_label_customization** | Option<[**models::RequestedLabelCustomization**](RequestedLabelCustomization.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


