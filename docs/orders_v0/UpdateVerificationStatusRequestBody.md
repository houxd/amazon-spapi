# UpdateVerificationStatusRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<[**models::VerificationStatus**](VerificationStatus.md)> |  | [optional]
**external_reviewer_id** | **String** | The identifier of the order's regulated information reviewer. | 
**rejection_reason_id** | Option<**String**> | The unique identifier of the rejection reason used for rejecting the order's regulated information. Only required if the new status is rejected. | [optional]
**verification_details** | Option<[**models::VerificationDetails**](VerificationDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


