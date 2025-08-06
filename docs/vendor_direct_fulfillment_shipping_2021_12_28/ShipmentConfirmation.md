# ShipmentConfirmation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | Purchase order number corresponding to the shipment. | 
**shipment_details** | [**models::ShipmentDetails**](ShipmentDetails.md) |  | 
**selling_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**ship_from_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**items** | [**Vec<models::Item>**](Item.md) | Provide the details of the items in this shipment. If any of the item details field is common at a package or a pallet level, then provide them at the corresponding package. | 
**containers** | Option<[**Vec<models::Container>**](Container.md)> | Provide the details of the items in this shipment. If any of the item details field is common at a package or a pallet level, then provide them at the corresponding package. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


