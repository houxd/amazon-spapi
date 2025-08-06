# SafetReimbursementEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**safet_claim_id** | Option<**String**> | A SAFE-T claim identifier. | [optional]
**reimbursed_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**reason_code** | Option<**String**> | Indicates why the seller was reimbursed. | [optional]
**safet_reimbursement_item_list** | Option<[**Vec<models::SafetReimbursementItem>**](SAFETReimbursementItem.md)> | A list of SAFETReimbursementItems. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


