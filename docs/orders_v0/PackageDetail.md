# PackageDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**package_reference_id** | **String** | A seller-supplied identifier that uniquely identifies a package within the scope of an order. Only positive numeric values are supported. | 
**carrier_code** | **String** | Identifies the carrier that will deliver the package. This field is required for all marketplaces. For more information, refer to the [`CarrierCode` announcement](https://developer-docs.amazon.com/sp-api/changelog/carriercode-value-required-in-shipment-confirmations-for-br-mx-ca-sg-au-in-jp-marketplaces). | 
**carrier_name** | Option<**String**> | Carrier name that will deliver the package. Required when `carrierCode` is \"Other\"  | [optional]
**shipping_method** | Option<**String**> | Ship method to be used for shipping the order. | [optional]
**tracking_number** | **String** | The tracking number used to obtain tracking and delivery information. | 
**ship_date** | **String** | The shipping date for the package. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> date/time format. | 
**ship_from_supply_source_id** | Option<**String**> | The unique identifier for the supply source. | [optional]
**order_items** | [**Vec<models::ConfirmShipmentOrderItem>**](ConfirmShipmentOrderItem.md) | A list of order items. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


