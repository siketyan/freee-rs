# ApiV1EmployeesProfileRulesControllerPeriodUpdateBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 更新対象事業所ID（必須） | 
**year** | **i32** | 更新対象年（必須） | 
**month** | **i32** | 更新対象月（必須）<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が更新されます。 | 
**employee_profile_rule** | [**crate::models::ApiV1EmployeesProfileRuleUpdateRequestSerializer**](ApiV1EmployeesProfileRuleUpdateRequestSerializer.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


