# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**selling_partner_metadata** | Option<[**models::SellingPartnerMetadata**](SellingPartnerMetadata.md)> |  | [optional]
**related_identifiers** | Option<[**Vec<models::RelatedIdentifier>**](RelatedIdentifier.md)> | Related business identifiers of the transaction. | [optional]
**transaction_type** | Option<**String**> | The type of transaction.  Possible values:  * Shipment | [optional]
**transaction_id** | Option<**String**> | The unique identifier for the transaction. | [optional]
**transaction_status** | Option<**String**> | The status of the transaction.  **Possible values:**  * `DEFERRED`: the transaction is currently deferred. * `RELEASED`: the transaction is currently released. * `DEFERRED_RELEASED`: the transaction was deferred in the past, but is now released. The status of a deferred transaction is updated to `DEFERRED_RELEASED` when the transaction is released. | [optional]
**description** | Option<**String**> | Describes the reasons for the transaction.  Example: 'Order Payment','Refund Order' | [optional]
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**total_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**marketplace_details** | Option<[**models::MarketplaceDetails**](MarketplaceDetails.md)> |  | [optional]
**items** | Option<[**Vec<models::Item>**](Item.md)> | List of items in the transaction | [optional]
**contexts** | Option<[**Vec<models::Context>**](Context.md)> | List of additional Information about the item. | [optional]
**breakdowns** | Option<[**Vec<models::Breakdown>**](Breakdown.md)> | A list of breakdowns that detail how the total amount is calculated for the transaction. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


