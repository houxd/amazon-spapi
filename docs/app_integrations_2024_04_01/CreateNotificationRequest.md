# CreateNotificationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**template_id** | **String** | The unique identifier of the notification template you used to onboard your application. | 
**notification_parameters** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | The dynamic parameters required by the notification templated specified by `templateId`. | 
**marketplace_id** | Option<**String**> | An encrypted marketplace identifier for the posted notification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


