# ApiV1EmployeesControllerPeriodUpdateBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 更新対象事業所ID（必須） | 
**year** | Option<**i32**> | 更新対象年 - 給与計算対象の従業員情報の場合は必須になります。 | [optional]
**month** | Option<**i32**> | 更新対象月 - 給与計算対象の従業員情報の場合は必須になります。 | [optional]
**employee** | [**crate::models::ApiV1EmployeeUpdateRequestSerializer**](ApiV1EmployeeUpdateRequestSerializer.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


