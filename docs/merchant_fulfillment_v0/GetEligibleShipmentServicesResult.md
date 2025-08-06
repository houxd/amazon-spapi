# GetEligibleShipmentServicesResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipping_service_list** | [**Vec<models::ShippingService>**](ShippingService.md) | A list of shipping services offers. | 
**rejected_shipping_service_list** | Option<[**Vec<models::RejectedShippingService>**](RejectedShippingService.md)> | List of services that are for some reason unavailable for this request | [optional]
**temporarily_unavailable_carrier_list** | Option<[**Vec<models::TemporarilyUnavailableCarrier>**](TemporarilyUnavailableCarrier.md)> | A list of temporarily unavailable carriers. | [optional]
**terms_and_conditions_not_accepted_carrier_list** | Option<[**Vec<models::TermsAndConditionsNotAcceptedCarrier>**](TermsAndConditionsNotAcceptedCarrier.md)> | List of carriers whose terms and conditions were not accepted by the seller. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


