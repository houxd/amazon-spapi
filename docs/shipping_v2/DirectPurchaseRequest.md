# DirectPurchaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ship_to** | Option<[**models::Address**](Address.md)> |  | [optional]
**ship_from** | Option<[**models::Address**](Address.md)> |  | [optional]
**return_to** | Option<[**models::Address**](Address.md)> |  | [optional]
**packages** | Option<[**Vec<models::Package>**](Package.md)> | A list of packages to be shipped through a shipping service offering. | [optional]
**channel_details** | [**models::ChannelDetails**](ChannelDetails.md) |  | 
**label_specifications** | Option<[**models::RequestedDocumentSpecification**](RequestedDocumentSpecification.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


