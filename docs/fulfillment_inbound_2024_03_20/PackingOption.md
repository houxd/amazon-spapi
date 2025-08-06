# PackingOption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**discounts** | [**Vec<models::Incentive>**](Incentive.md) | Discount for the offered option. | 
**expiration** | Option<**String**> | The time at which this packing option is no longer valid. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) datetime format with pattern `yyyy-MM-ddTHH:mm:ss.sssZ`. | [optional]
**fees** | [**Vec<models::Incentive>**](Incentive.md) | Fee for the offered option. | 
**packing_groups** | **Vec<String>** | Packing group IDs. | 
**packing_option_id** | **String** | Identifier of a packing option. | 
**status** | **String** | The status of the packing option. Possible values: `OFFERED`, `ACCEPTED`, `EXPIRED`. | 
**supported_configurations** | [**Vec<models::PackingConfiguration>**](PackingConfiguration.md) | A list of possible configurations for this option. | 
**supported_shipping_configurations** | [**Vec<models::ShippingConfiguration>**](ShippingConfiguration.md) | **This field is deprecated**. Use the `shippingRequirements` property under `supportedConfigurations` instead. List of supported shipping modes. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


