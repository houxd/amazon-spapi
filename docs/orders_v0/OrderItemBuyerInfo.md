# OrderItemBuyerInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_item_id** | **String** | An Amazon-defined order item identifier. | 
**buyer_customized_info** | Option<[**models::BuyerCustomizedInfoDetail**](BuyerCustomizedInfoDetail.md)> |  | [optional]
**gift_wrap_price** | Option<[**models::Money**](Money.md)> |  | [optional]
**gift_wrap_tax** | Option<[**models::Money**](Money.md)> |  | [optional]
**gift_message_text** | Option<**String**> | A gift message provided by the buyer.  **Note**: This attribute is only available for MFN (fulfilled by seller) orders. | [optional]
**gift_wrap_level** | Option<**String**> | The gift wrap level specified by the buyer. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


