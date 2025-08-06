# GetRatesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ship_to** | [**models::Address**](Address.md) |  | 
**ship_from** | [**models::Address**](Address.md) |  | 
**service_types** | [**Vec<models::ServiceType>**](ServiceType.md) | A list of service types that can be used to send the shipment. | 
**ship_date** | Option<**String**> | The start date and time. This defaults to the current date and time. | [optional]
**container_specifications** | [**Vec<models::ContainerSpecification>**](ContainerSpecification.md) | A list of container specifications. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


