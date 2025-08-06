# OfferIdentifier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | The marketplace ID is the globally unique identifier of a marketplace. To find the ID for your marketplace, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | 
**seller_id** | Option<**String**> | The seller identifier for the offer. | [optional]
**sku** | Option<**String**> | The seller SKU of the item. This will only be present for the target offer, which belongs to the requesting seller. | [optional]
**asin** | **String** | The ASIN of the item. | 
**fulfillment_type** | Option<[**models::FulfillmentType**](FulfillmentType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


