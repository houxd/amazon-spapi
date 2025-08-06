# ShipmentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | Option<**String**> | The seller SKU of the item. The seller SKU is qualified by the seller's seller ID, which is included with every call to the Selling Partner API. | [optional]
**order_item_id** | Option<**String**> | An Amazon-defined order item identifier. | [optional]
**order_adjustment_item_id** | Option<**String**> | An Amazon-defined order adjustment identifier defined for refunds, guarantee claims, and chargeback events. | [optional]
**quantity_shipped** | Option<**i32**> | The number of items shipped. | [optional]
**item_charge_list** | Option<[**Vec<models::ChargeComponent>**](ChargeComponent.md)> | A list of charge information on the seller's account. | [optional]
**item_charge_adjustment_list** | Option<[**Vec<models::ChargeComponent>**](ChargeComponent.md)> | A list of charge information on the seller's account. | [optional]
**item_fee_list** | Option<[**Vec<models::FeeComponent>**](FeeComponent.md)> | A list of fee component information. | [optional]
**item_fee_adjustment_list** | Option<[**Vec<models::FeeComponent>**](FeeComponent.md)> | A list of fee component information. | [optional]
**item_tax_withheld_list** | Option<[**Vec<models::TaxWithheldComponent>**](TaxWithheldComponent.md)> | A list of information about taxes withheld. | [optional]
**promotion_list** | Option<[**Vec<models::Promotion>**](Promotion.md)> | A list of promotions. | [optional]
**promotion_adjustment_list** | Option<[**Vec<models::Promotion>**](Promotion.md)> | A list of promotions. | [optional]
**cost_of_points_granted** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**cost_of_points_returned** | Option<[**models::Currency**](Currency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


