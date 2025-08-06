# BuyerInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**buyer_email** | Option<**String**> | The anonymized email address of the buyer. | [optional]
**buyer_name** | Option<**String**> | The buyer name or the recipient name. | [optional]
**buyer_county** | Option<**String**> | The county of the buyer.  **Note**: This attribute is only available in the Brazil marketplace. | [optional]
**buyer_tax_info** | Option<[**models::BuyerTaxInfo**](BuyerTaxInfo.md)> |  | [optional]
**purchase_order_number** | Option<**String**> | The purchase order (PO) number entered by the buyer at checkout. Only returned for orders where the buyer entered a PO number at checkout. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


