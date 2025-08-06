# EventFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregation_settings** | Option<[**models::AggregationSettings**](AggregationSettings.md)> |  | [optional]
**marketplace_ids** | Option<**Vec<String>**> | A list of marketplace identifiers to subscribe to (for example: ATVPDKIKX0DER). To receive notifications in every marketplace, do not provide this list. | [optional]
**order_change_types** | Option<[**Vec<models::OrderChangeTypeEnum>**](OrderChangeTypeEnum.md)> | A list of order change types to subscribe to (for example: `BuyerRequestedChange`). To receive notifications of all change types, do not provide this list. | [optional]
**event_filter_type** | **String** | An `eventFilterType` value that is supported by the specific `notificationType`. This is used by the subscription service to determine the type of event filter. Refer to [Notification Type Values](https://developer-docs.amazon.com/sp-api/docs/notification-type-values) to determine if an `eventFilterType` is supported. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


