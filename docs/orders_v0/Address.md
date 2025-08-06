# Address

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name. | 
**company_name** | Option<**String**> | The company name of the recipient.  **Note**: This attribute is only available for shipping address. | [optional]
**address_line1** | Option<**String**> | The street address. | [optional]
**address_line2** | Option<**String**> | Additional street address information, if required. | [optional]
**address_line3** | Option<**String**> | Additional street address information, if required. | [optional]
**city** | Option<**String**> | The city. | [optional]
**county** | Option<**String**> | The county. | [optional]
**district** | Option<**String**> | The district. | [optional]
**state_or_region** | Option<**String**> | The state or region. | [optional]
**municipality** | Option<**String**> | The municipality. | [optional]
**postal_code** | Option<**String**> | The postal code. | [optional]
**country_code** | Option<**String**> | The country code. A two-character country code, in ISO 3166-1 alpha-2 format. | [optional]
**phone** | Option<**String**> | The phone number of the buyer.  **Note**:  1. This attribute is only available for shipping address. 2. In some cases, the buyer phone number is suppressed:  a. Phone is suppressed for all `AFN` (fulfilled by Amazon) orders. b. Phone is suppressed for the shipped `MFN` (fulfilled by seller) order when the current date is past the Latest Delivery Date. | [optional]
**extended_fields** | Option<[**models::AddressExtendedFields**](AddressExtendedFields.md)> |  | [optional]
**address_type** | Option<**String**> | The address type of the shipping address. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


