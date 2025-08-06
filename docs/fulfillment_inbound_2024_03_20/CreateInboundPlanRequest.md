# CreateInboundPlanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_marketplaces** | **Vec<String>** | Marketplaces where the items need to be shipped to. Currently only one marketplace can be selected in this request. | 
**items** | [**Vec<models::ItemInput>**](ItemInput.md) | Items included in this plan. | 
**name** | Option<**String**> | Name for the Inbound Plan. If one isn't provided, a default name will be provided. | [optional]
**source_address** | [**models::AddressInput**](AddressInput.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


