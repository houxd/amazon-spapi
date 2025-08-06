# DestinationDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_address** | Option<[**models::Address**](Address.md)> |  | [optional]
**destination_region** | Option<**String**> | Assigned region where the order will be shipped. This can differ from what was passed as preference. AWD currently supports following region IDs: [us-west, us-east, us-southcentral, us-southeast] | [optional]
**shipment_id** | Option<**String**> | Unique ID of the confirmed shipment being shipped to the assigned destination. This will be available only after an inbound order is confirmed and can be used to track the shipment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


