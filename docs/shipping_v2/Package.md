# Package

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dimensions** | [**models::Dimensions**](Dimensions.md) |  | 
**weight** | [**models::Weight**](Weight.md) |  | 
**insured_value** | [**models::Currency**](Currency.md) |  | 
**is_hazmat** | Option<**bool**> | When true, the package contains hazardous materials. Defaults to false. | [optional]
**seller_display_name** | Option<**String**> | The seller name displayed on the label. | [optional]
**charges** | Option<[**Vec<models::ChargeComponent>**](ChargeComponent.md)> | A list of charges based on the shipping service charges applied on a package. | [optional]
**package_client_reference_id** | **String** | A client provided unique identifier for a package being shipped. This value should be saved by the client to pass as a parameter to the getShipmentDocuments operation. | 
**items** | [**Vec<models::Item>**](Item.md) | A list of items. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


