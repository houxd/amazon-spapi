# PackageDocumentDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**package_client_reference_id** | **String** | A client provided unique identifier for a package being shipped. This value should be saved by the client to pass as a parameter to the getShipmentDocuments operation. | 
**package_documents** | [**Vec<models::PackageDocument>**](PackageDocument.md) | A list of documents related to a package. | 
**tracking_id** | Option<**String**> | The carrier generated identifier for a package in a purchased shipment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


