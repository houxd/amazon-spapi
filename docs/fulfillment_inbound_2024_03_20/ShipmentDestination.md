# ShipmentDestination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<[**models::Address**](Address.md)> |  | [optional]
**destination_type** | **String** | The type of destination for this shipment. Possible values: `AMAZON_OPTIMIZED`, `AMAZON_WAREHOUSE`. | 
**warehouse_id** | Option<**String**> | The warehouse that the shipment should be sent to. Empty if the destination type is `AMAZON_OPTIMIZED`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


