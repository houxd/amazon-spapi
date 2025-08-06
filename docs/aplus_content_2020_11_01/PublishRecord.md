# PublishRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | The marketplace ID is the globally unique identifier of a marketplace. To find the ID for your marketplace, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | 
**locale** | **String** | The IETF language tag, which supports the primary language subtag and one secondary language subtag. The secondary language subtag is usually a regional designation. This doesn't support subtags other than the primary and secondary subtags. **Pattern:** ^[a-z]{2,}-[A-Z0-9]{2,}$ | 
**asin** | **String** | The Amazon Standard Identification Number (ASIN). | 
**content_type** | [**models::ContentType**](ContentType.md) |  | 
**content_sub_type** | Option<**String**> | The A+ Content document subtype. This represents a special-purpose type of an A+ Content document. Not every A+ Content document type has a subtype, and subtypes can change at any time. | [optional]
**content_reference_key** | **String** | A unique reference key for the A+ Content document. A content reference key cannot form a permalink and might change in the future. A content reference key is not guaranteed to match any A+ content identifier. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


