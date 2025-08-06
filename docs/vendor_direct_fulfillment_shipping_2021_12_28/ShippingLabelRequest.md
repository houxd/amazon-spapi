# ShippingLabelRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The purchase order number of the order for which to create a shipping label. | 
**selling_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**ship_from_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**containers** | Option<[**Vec<models::Container>**](Container.md)> | A list of the packages in this shipment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


