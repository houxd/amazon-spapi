# Vehicle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**make** | **String** | Vehicle Brand. | 
**model** | **String** | Specific model of a vehicle. | 
**variant_name** | Option<**String**> | Name of the vehicle variant. | [optional]
**body_style** | Option<**String**> | Body style of vehicle (example: Hatchback, Cabriolet). | [optional]
**drive_type** | Option<**String**> | Drive type of vehicle(example: Rear wheel drive). | [optional]
**energy** | Option<**String**> | Energy Source for the vehicle(example: Petrol) | [optional]
**engine_output** | Option<[**Vec<models::EngineOutput>**](EngineOutput.md)> | Engine output of vehicle. | [optional]
**manufacturing_start_date** | Option<[**models::MonthAndYear**](MonthAndYear.md)> |  | [optional]
**manufacturing_stop_date** | Option<[**models::MonthAndYear**](MonthAndYear.md)> |  | [optional]
**last_processed_date** | Option<**String**> | The date on which the vehicle was last updated, in ISO-8601 date/time format. | [optional]
**status** | Option<[**models::VehicleStatusInCatalog**](VehicleStatusInCatalog.md)> |  | [optional]
**identifiers** | [**Vec<models::VehicleIdentifiers>**](VehicleIdentifiers.md) | Identifiers that can be used to identify the vehicle uniquely | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


