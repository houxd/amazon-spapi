# Box

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**box_id** | Option<**String**> | The ID provided by Amazon that identifies a given box. This ID is comprised of the external shipment ID (which is generated after transportation has been confirmed) and the index of the box. | [optional]
**content_information_source** | Option<[**models::BoxContentInformationSource**](BoxContentInformationSource.md)> |  | [optional]
**destination_region** | Option<[**models::Region**](Region.md)> |  | [optional]
**dimensions** | Option<[**models::Dimensions**](Dimensions.md)> |  | [optional]
**external_container_identifier** | Option<**String**> | The external identifier for this container / box. | [optional]
**external_container_identifier_type** | Option<**String**> | Type of the external identifier used. Can be: `AMAZON`, `SSCC`. | [optional]
**items** | Option<[**Vec<models::Item>**](Item.md)> | Items contained within the box. | [optional]
**package_id** | **String** | Primary key to uniquely identify a Package (Box or Pallet). | 
**quantity** | Option<**i32**> | The number of containers where all other properties like weight or dimensions are identical. | [optional]
**template_name** | Option<**String**> | Template name of the box. | [optional]
**weight** | Option<[**models::Weight**](Weight.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


