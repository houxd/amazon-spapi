# CreateContainerLabelRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**selling_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**ship_from_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**carrier_id** | [**models::CarrierId**](CarrierId.md) |  | 
**vendor_container_id** | **String** | The unique, vendor-provided identifier for the container. | 
**packages** | [**Vec<models::Package>**](Package.md) | An array of package objects in a container. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


