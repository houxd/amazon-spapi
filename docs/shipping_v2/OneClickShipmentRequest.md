# OneClickShipmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ship_to** | Option<[**models::Address**](Address.md)> |  | [optional]
**ship_from** | [**models::Address**](Address.md) |  | 
**return_to** | Option<[**models::Address**](Address.md)> |  | [optional]
**ship_date** | Option<**String**> | The ship date and time (the requested pickup). This defaults to the current date and time. | [optional]
**goods_owner** | Option<[**models::GoodsOwner**](GoodsOwner.md)> |  | [optional]
**packages** | [**Vec<models::Package>**](Package.md) | A list of packages to be shipped through a shipping service offering. | 
**value_added_services_details** | Option<[**Vec<models::OneClickShipmentValueAddedService>**](OneClickShipmentValueAddedService.md)> | The value-added services to be added to a shipping service purchase. | [optional]
**tax_details** | Option<[**Vec<models::TaxDetail>**](TaxDetail.md)> | A list of tax detail information. | [optional]
**channel_details** | [**models::ChannelDetails**](ChannelDetails.md) |  | 
**label_specifications** | [**models::RequestedDocumentSpecification**](RequestedDocumentSpecification.md) |  | 
**service_selection** | [**models::ServiceSelection**](ServiceSelection.md) |  | 
**shipper_instruction** | Option<[**models::ShipperInstruction**](ShipperInstruction.md)> |  | [optional]
**destination_access_point_details** | Option<[**models::AccessPointDetails**](AccessPointDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


