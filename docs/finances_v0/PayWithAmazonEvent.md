# PayWithAmazonEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_order_id** | Option<**String**> | An order identifier that is specified by the seller. | [optional]
**transaction_posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**business_object_type** | Option<**String**> | The type of business object. | [optional]
**sales_channel** | Option<**String**> | The sales channel for the transaction. | [optional]
**charge** | Option<[**models::ChargeComponent**](ChargeComponent.md)> |  | [optional]
**fee_list** | Option<[**Vec<models::FeeComponent>**](FeeComponent.md)> | A list of fee component information. | [optional]
**payment_amount_type** | Option<**String**> | The type of payment.  Possible values:  * Sales | [optional]
**amount_description** | Option<**String**> | A short description of this payment event. | [optional]
**fulfillment_channel** | Option<**String**> | The fulfillment channel.  Possible values:  * AFN - Amazon Fulfillment Network (Fulfillment by Amazon)  * MFN - Merchant Fulfillment Network (self-fulfilled) | [optional]
**store_name** | Option<**String**> | The store name where the event occurred. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


