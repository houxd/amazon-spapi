# ImportDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**method_of_payment** | Option<**String**> | This is used for import purchase orders only. If the recipient requests, this field will contain the shipment method of payment. | [optional]
**seal_number** | Option<**String**> | The container's seal number. | [optional]
**route** | Option<[**models::Route**](Route.md)> |  | [optional]
**import_containers** | Option<**String**> | Types and numbers of container(s) for import purchase orders. Can be a comma-separated list if shipment has multiple containers. | [optional]
**billable_weight** | Option<[**models::Weight**](Weight.md)> |  | [optional]
**estimated_ship_by_date** | Option<**String**> | Date on which the shipment is expected to be shipped. This value should not be in the past and not more than 60 days out in the future. | [optional]
**handling_instructions** | Option<**String**> | Identification of the instructions on how specified item/carton/pallet should be handled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


