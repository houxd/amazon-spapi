# Rate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rate_id** | **String** | An identifier for the rate (shipment offering) provided by a shipping service provider. | 
**carrier_id** | **String** | The carrier identifier for the offering, provided by the carrier. | 
**carrier_name** | **String** | The carrier name for the offering. | 
**service_id** | **String** | An identifier for the shipping service. | 
**service_name** | **String** | The name of the shipping service. | 
**billed_weight** | Option<[**models::Weight**](Weight.md)> |  | [optional]
**total_charge** | [**models::Currency**](Currency.md) |  | 
**promise** | [**models::Promise**](Promise.md) |  | 
**supported_document_specifications** | [**Vec<models::SupportedDocumentSpecification>**](SupportedDocumentSpecification.md) | A list of the document specifications supported for a shipment service offering. | 
**available_value_added_service_groups** | Option<[**Vec<models::AvailableValueAddedServiceGroup>**](AvailableValueAddedServiceGroup.md)> | A list of value-added services available for a shipping service offering. | [optional]
**requires_additional_inputs** | **bool** | When true, indicates that additional inputs are required to purchase this shipment service. You must then call the getAdditionalInputs operation to return the JSON schema to use when providing the additional inputs to the purchaseShipment operation. | 
**rate_item_list** | Option<[**Vec<models::RateItem>**](RateItem.md)> | A list of RateItem | [optional]
**payment_type** | Option<[**models::PaymentType**](PaymentType.md)> |  | [optional]
**benefits** | Option<[**models::Benefits**](Benefits.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


