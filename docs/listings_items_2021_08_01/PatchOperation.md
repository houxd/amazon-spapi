# PatchOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**op** | **String** | Type of JSON Patch operation. Supported JSON Patch operations include `add`, `replace`, `merge` and `delete`. Refer to <https://tools.ietf.org/html/rfc6902>. | 
**path** | **String** | JSON Pointer path of the element to patch. Refer to [JavaScript Object Notation (JSON) Patch](https://tools.ietf.org/html/rfc6902) for more information. | 
**value** | Option<[**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md)> | JSON value to `add`, `replace`, `merge` or `delete`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


