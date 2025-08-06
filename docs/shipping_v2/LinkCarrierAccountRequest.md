# LinkCarrierAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_reference_details** | Option<[**Vec<models::ClientReferenceDetail>**](ClientReferenceDetail.md)> | Object to pass additional information about the MCI Integrator shipperType: List of ClientReferenceDetail | [optional]
**carrier_account_type** | **String** | CarrierAccountType  associated with account. | 
**carrier_account_attributes** | [**Vec<models::CarrierAccountAttribute>**](CarrierAccountAttribute.md) | A list of all attributes required by the carrier in order to successfully link the merchant's account | 
**encrypted_carrier_account_attributes** | Option<[**Vec<models::CarrierAccountAttribute>**](CarrierAccountAttribute.md)> | A list of all attributes required by the carrier in order to successfully link the merchant's account | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


