# DebtRecoveryEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**debt_recovery_type** | Option<**String**> | The debt recovery type.  Possible values:  * DebtPayment  * DebtPaymentFailure  * DebtAdjustment | [optional]
**recovery_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**over_payment_credit** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**debt_recovery_item_list** | Option<[**Vec<models::DebtRecoveryItem>**](DebtRecoveryItem.md)> | A list of debt recovery item information. | [optional]
**charge_instrument_list** | Option<[**Vec<models::ChargeInstrument>**](ChargeInstrument.md)> | A list of payment instruments. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


