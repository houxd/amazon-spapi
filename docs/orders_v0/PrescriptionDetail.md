# PrescriptionDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prescription_id** | **String** | The identifier for the prescription used to verify the regulated product. | 
**expiration_date** | **String** | The expiration date of the prescription used to verify the regulated product, in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date time format. | 
**written_quantity** | **i32** | The number of units in each fill as provided in the prescription. | 
**total_refills_authorized** | **i32** | The total number of refills written in the original prescription used to verify the regulated product. If a prescription originally had no refills, this value must be 0. | 
**refills_remaining** | **i32** | The number of refills remaining for the prescription used to verify the regulated product. If a prescription originally had 10 total refills, this value must be `10` for the first order, `9` for the second order, and `0` for the eleventh order. If a prescription originally had no refills, this value must be 0. | 
**clinic_id** | **String** | The identifier for the clinic which provided the prescription used to verify the regulated product. | 
**usage_instructions** | **String** | The instructions for the prescription as provided by the approver of the regulated product. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


