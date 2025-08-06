# TimeInterval

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date** | **String** | When this object is used as a request parameter, the specified `startDate` is adjusted based on the aggregation frequency.  * For `WEEK` the metric is computed from the first day of the week (Sunday, based on ISO 8601) that contains the `startDate`. * For `MONTH` the metric is computed from the first day of the month that contains the `startDate`. * For `QUARTER` the metric is computed from the first day of the quarter that contains the `startDate`. * For `YEAR` the metric is computed from the first day of the year that contains the `startDate`. | 
**end_date** | **String** | When this object is used as a request parameter, the specified `endDate` is adjusted based on the aggregation frequency.  * For `WEEK` the metric is computed up to the last day of the week (Sunday, based on ISO 8601) that contains the `endDate`. * For `MONTH`, the metric is computed up to the last day that contains the `endDate`. * For `QUARTER` the metric is computed up to the last day of the quarter that contains the `endDate`. * For `YEAR` the metric is computed up to the last day of the year that contains the `endDate`.  Note: The end date may be adjusted to a lower value based on the data available in our system. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


