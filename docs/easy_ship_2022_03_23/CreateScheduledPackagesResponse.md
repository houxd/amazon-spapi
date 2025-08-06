# CreateScheduledPackagesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scheduled_packages** | Option<[**Vec<models::Package>**](Package.md)> | A list of packages. Refer to the `Package` object. | [optional]
**rejected_orders** | Option<[**Vec<models::RejectedOrder>**](RejectedOrder.md)> | A list of orders we couldn't scheduled on your behalf. Each element contains the reason and details on the error. | [optional]
**printable_documents_url** | Option<**String**> | A pre-signed URL for the zip document containing the shipping labels and the documents enabled for your marketplace. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


