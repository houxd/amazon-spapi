# TransportationOption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**carrier** | [**models::Carrier**](Carrier.md) |  | 
**carrier_appointment** | Option<[**models::CarrierAppointment**](CarrierAppointment.md)> |  | [optional]
**preconditions** | **Vec<String>** | Identifies a list of preconditions for confirming the transportation option. | 
**quote** | Option<[**models::Quote**](Quote.md)> |  | [optional]
**shipment_id** | **String** | Identifier of a shipment. A shipment contains the boxes and units being inbounded. | 
**shipping_mode** | **String** | Mode of shipment transportation that this option will provide.  Possible values: `GROUND_SMALL_PARCEL`, `FREIGHT_LTL`, `FREIGHT_FTL_PALLET`, `FREIGHT_FTL_NONPALLET`, `OCEAN_LCL`, `OCEAN_FCL`, `AIR_SMALL_PARCEL`, `AIR_SMALL_PARCEL_EXPRESS`. | 
**shipping_solution** | **String** | Shipping program for the option. Possible values: `AMAZON_PARTNERED_CARRIER`, `USE_YOUR_OWN_CARRIER`. | 
**transportation_option_id** | **String** | Identifier of a transportation option. A transportation option represent one option for how to send a shipment. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


