# ServiceFeeEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | Option<**String**> | An Amazon-defined identifier for an order. | [optional]
**fee_reason** | Option<**String**> | A short description of the service fee reason. | [optional]
**fee_list** | Option<[**Vec<models::FeeComponent>**](FeeComponent.md)> | A list of fee component information. | [optional]
**seller_sku** | Option<**String**> | The seller SKU of the item. The seller SKU is qualified by the seller's seller ID, which is included with every call to the Selling Partner API. | [optional]
**fn_sku** | Option<**String**> | A unique identifier assigned by Amazon to products stored in and fulfilled from an Amazon fulfillment center. | [optional]
**fee_description** | Option<**String**> | A short description of the service fee event. | [optional]
**asin** | Option<**String**> | The Amazon Standard Identification Number (ASIN) of the item. | [optional]
**store_name** | Option<**String**> | The name of the store where the event occurred. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


