# GetRatesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ship_to** | Option<[**models::Address**](Address.md)> |  | [optional]
**ship_from** | [**models::Address**](Address.md) |  | 
**return_to** | Option<[**models::Address**](Address.md)> |  | [optional]
**ship_date** | Option<**String**> | The ship date and time (the requested pickup). This defaults to the current date and time. | [optional]
**shipper_instruction** | Option<[**models::ShipperInstruction**](ShipperInstruction.md)> |  | [optional]
**packages** | [**Vec<models::Package>**](Package.md) | A list of packages to be shipped through a shipping service offering. | 
**value_added_services** | Option<[**models::ValueAddedServiceDetails**](ValueAddedServiceDetails.md)> |  | [optional]
**tax_details** | Option<[**Vec<models::TaxDetail>**](TaxDetail.md)> | A list of tax detail information. | [optional]
**channel_details** | [**models::ChannelDetails**](ChannelDetails.md) |  | 
**client_reference_details** | Option<[**Vec<models::ClientReferenceDetail>**](ClientReferenceDetail.md)> | Object to pass additional information about the MCI Integrator shipperType: List of ClientReferenceDetail | [optional]
**shipment_type** | Option<[**models::ShipmentType**](ShipmentType.md)> |  | [optional]
**destination_access_point_details** | Option<[**models::AccessPointDetails**](AccessPointDetails.md)> |  | [optional]
**carrier_accounts** | Option<[**Vec<models::CarrierAccount>**](CarrierAccount.md)> | A list of CarrierAccounts | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


