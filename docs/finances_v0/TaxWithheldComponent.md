# TaxWithheldComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tax_collection_model** | Option<**String**> | The tax collection model applied to the item.  Possible values:  * MarketplaceFacilitator - Tax is withheld and remitted to the taxing authority by Amazon on behalf of the seller.  * Standard - Tax is paid to the seller and not remitted to the taxing authority by Amazon. | [optional]
**taxes_withheld** | Option<[**Vec<models::ChargeComponent>**](ChargeComponent.md)> | A list of charge information on the seller's account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


