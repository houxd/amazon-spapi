# ListOfferMetricsRequestFilters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregation_frequency** | Option<[**models::AggregationFrequency**](AggregationFrequency.md)> |  | [optional]
**time_interval** | [**models::TimeInterval**](TimeInterval.md) |  | 
**time_period_type** | [**models::TimePeriodType**](TimePeriodType.md) |  | 
**marketplace_id** | **String** | The marketplace identifier. The supported marketplaces for both sellers and vendors are US, CA, ES, UK, FR, IT, IN, DE and JP. The supported marketplaces for vendors only are BR, AU, MX, AE and NL. Refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids) to find the identifier for the marketplace. | 
**program_types** | [**Vec<models::ProgramType>**](ProgramType.md) | A list of replenishment program types. | 
**asins** | Option<**Vec<String>**> | A list of Amazon Standard Identification Numbers (ASINs). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


