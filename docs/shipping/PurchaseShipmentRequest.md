# PurchaseShipmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_reference_id** | **String** | Client reference id. | 
**ship_to** | [**models::Address**](Address.md) |  | 
**ship_from** | [**models::Address**](Address.md) |  | 
**ship_date** | Option<**String**> | The start date and time. This defaults to the current date and time. | [optional]
**service_type** | [**models::ServiceType**](ServiceType.md) |  | 
**containers** | [**Vec<models::Container>**](Container.md) | A list of container. | 
**label_specification** | [**models::LabelSpecification**](LabelSpecification.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


