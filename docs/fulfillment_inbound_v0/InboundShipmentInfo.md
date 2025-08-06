# InboundShipmentInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | Option<**String**> | The shipment identifier submitted in the request. | [optional]
**shipment_name** | Option<**String**> | The name for the inbound shipment. | [optional]
**ship_from_address** | [**models::Address**](Address.md) |  | 
**destination_fulfillment_center_id** | Option<**String**> | An Amazon fulfillment center identifier created by Amazon. | [optional]
**shipment_status** | Option<[**models::ShipmentStatus**](ShipmentStatus.md)> |  | [optional]
**label_prep_type** | Option<[**models::LabelPrepType**](LabelPrepType.md)> |  | [optional]
**are_cases_required** | **bool** | Indicates whether or not an inbound shipment contains case-packed boxes. When AreCasesRequired = true for an inbound shipment, all items in the inbound shipment must be case packed. | 
**confirmed_need_by_date** | Option<[**String**](string.md)> | Type containing date in string format | [optional]
**box_contents_source** | Option<[**models::BoxContentsSource**](BoxContentsSource.md)> |  | [optional]
**estimated_box_contents_fee** | Option<[**models::BoxContentsFeeDetails**](BoxContentsFeeDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


