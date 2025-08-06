# Shipment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | **String** | The unique shipment identifier. | 
**client_reference_id** | **String** | Client reference id. | 
**ship_from** | [**models::Address**](Address.md) |  | 
**ship_to** | [**models::Address**](Address.md) |  | 
**accepted_rate** | Option<[**models::AcceptedRate**](AcceptedRate.md)> |  | [optional]
**shipper** | Option<[**models::Party**](Party.md)> |  | [optional]
**containers** | [**Vec<models::Container>**](Container.md) | A list of container. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


