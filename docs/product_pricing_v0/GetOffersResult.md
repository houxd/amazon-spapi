# GetOffersResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. | 
**asin** | Option<**String**> | The Amazon Standard Identification Number (ASIN) of the item. | [optional]
**sku** | Option<**String**> | The stock keeping unit (SKU) of the item. | [optional]
**item_condition** | [**models::ConditionType**](ConditionType.md) |  | 
**status** | **String** | The status of the operation. | 
**identifier** | [**models::ItemIdentifier**](ItemIdentifier.md) |  | 
**summary** | [**models::Summary**](Summary.md) |  | 
**offers** | [**Vec<models::OfferDetail>**](OfferDetail.md) | A list of offer details. The list is the same length as the TotalOfferCount in the Summary or 20, whichever is less. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


