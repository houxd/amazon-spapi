# InboundOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | Date when this order was created. | 
**destination_details** | Option<[**models::DestinationDetails**](DestinationDetails.md)> |  | [optional]
**external_reference_id** | Option<**String**> | Reference ID that can be used to correlate the order with partner resources. | [optional]
**order_id** | **String** | Inbound order ID. | 
**order_status** | [**models::InboundStatus**](InboundStatus.md) |  | 
**origin_address** | [**models::Address**](Address.md) |  | 
**packages_to_inbound** | [**Vec<models::DistributionPackageQuantity>**](DistributionPackageQuantity.md) | List of packages to be inbounded. | 
**preferences** | Option<[**models::InboundPreferences**](InboundPreferences.md)> |  | [optional]
**updated_at** | Option<**String**> | Date when this order was last updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


