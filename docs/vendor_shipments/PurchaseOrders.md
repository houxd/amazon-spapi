# PurchaseOrders

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | Option<**String**> | Purchase order numbers involved in this shipment, list all the PO that are involved as part of this shipment. | [optional]
**purchase_order_date** | Option<**String**> | Purchase order numbers involved in this shipment, list all the PO that are involved as part of this shipment. | [optional]
**ship_window** | Option<**String**> | Date range in which shipment is expected for these purchase orders. | [optional]
**items** | Option<[**Vec<models::PurchaseOrderItems>**](PurchaseOrderItems.md)> | A list of the items that are associated to the PO in this transport and their associated details. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


