# SkuPrepInstructions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | Option<**String**> | The seller SKU of the item. | [optional]
**asin** | Option<**String**> | The Amazon Standard Identification Number (ASIN) of the item. | [optional]
**barcode_instruction** | Option<[**models::BarcodeInstruction**](BarcodeInstruction.md)> |  | [optional]
**prep_guidance** | Option<[**models::PrepGuidance**](PrepGuidance.md)> |  | [optional]
**prep_instruction_list** | Option<[**Vec<models::PrepInstruction>**](PrepInstruction.md)> | A list of preparation instructions to help with item sourcing decisions. | [optional]
**amazon_prep_fees_details_list** | Option<[**Vec<models::AmazonPrepFeesDetails>**](AmazonPrepFeesDetails.md)> | A list of preparation instructions and fees for Amazon to prep goods for shipment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


