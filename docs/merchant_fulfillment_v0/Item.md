# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_item_id** | **String** | An Amazon-defined identifier for an individual item in an order. | 
**quantity** | **i32** | The number of items. | 
**item_weight** | Option<[**models::Weight**](Weight.md)> |  | [optional]
**item_description** | Option<**String**> | The description of the item. | [optional]
**transparency_code_list** | Option<**Vec<String>**> | A list of transparency codes. | [optional]
**item_level_seller_inputs_list** | Option<[**Vec<models::AdditionalSellerInputs>**](AdditionalSellerInputs.md)> | A list of additional seller input pairs required to purchase shipping. | [optional]
**liquid_volume** | Option<[**models::LiquidVolume**](LiquidVolume.md)> |  | [optional]
**is_hazmat** | Option<**bool**> | When true, the item qualifies as hazardous materials (hazmat). Defaults to false. | [optional]
**dangerous_goods_details** | Option<[**models::DangerousGoodsDetails**](DangerousGoodsDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


