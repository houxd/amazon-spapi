# ItemQuantity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **i32** | Amount of units shipped for a specific item at a shipment level. If the item is present only in certain cartons or pallets within the shipment, please provide this at the appropriate carton or pallet level. | 
**unit_of_measure** | **String** | Unit of measure for the shipped quantity. | 
**unit_size** | Option<**i32**> | The case size, in the event that we ordered using cases. Otherwise, 1. | [optional]
**total_weight** | Option<[**models::TotalWeight**](TotalWeight.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


