# InboundOrderCreationData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external_reference_id** | Option<**String**> | Reference ID that can be used to correlate the order with partner resources. | [optional]
**origin_address** | [**models::Address**](Address.md) |  | 
**packages_to_inbound** | [**Vec<models::DistributionPackageQuantity>**](DistributionPackageQuantity.md) | List of packages to be inbounded. | 
**preferences** | Option<[**models::InboundPreferences**](InboundPreferences.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


