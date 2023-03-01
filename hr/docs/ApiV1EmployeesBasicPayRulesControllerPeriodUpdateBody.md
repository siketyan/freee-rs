# ApiV1EmployeesBasicPayRulesControllerPeriodUpdateBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 更新対象事業所ID（必須） | 
**year** | **i32** | 更新対象年（必須） | 
**month** | **i32** | 更新対象月（必須）<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が更新されます<br> 翌月払いの従業員の2022/01の従業員情報を更新する場合は、year=2021,month=12を指定してください。<br> | 
**employee_basic_pay_rule** | [**crate::models::ApiV1EmployeesBasicPayRuleUpdateRequestSerializer**](ApiV1EmployeesBasicPayRuleUpdateRequestSerializer.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


