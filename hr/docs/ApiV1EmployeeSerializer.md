# ApiV1EmployeeSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 従業員ID | [optional]
**company_id** | Option<**i32**> | 事業所ID | [optional]
**num** | Option<**String**> | 従業員番号 | [optional]
**display_name** | Option<**String**> | 従業員名（表示名） | [optional]
**base_pension_num** | Option<**String**> | 基礎年金番号 | [optional]
**employment_insurance_reference_number** | Option<**String**> | 被保険者番号（雇用保険） | [optional]
**birth_date** | Option<[**String**](string.md)> | 生年月日 | [optional]
**entry_date** | Option<[**String**](string.md)> | 入社日 | [optional]
**retire_date** | Option<[**String**](string.md)> | 退職日 | [optional]
**user_id** | Option<**i32**> |  | [optional]
**profile_rule** | Option<[**crate::models::ApiV1EmployeesProfileRuleSerializer**](ApiV1EmployeesProfileRuleSerializer.md)> |  | [optional]
**health_insurance_rule** | Option<[**crate::models::ApiV1EmployeesHealthInsuranceRuleSerializer**](ApiV1EmployeesHealthInsuranceRuleSerializer.md)> |  | [optional]
**welfare_pension_insurance_rule** | Option<[**crate::models::ApiV1EmployeesWelfarePensionInsuranceRuleSerializer**](ApiV1EmployeesWelfarePensionInsuranceRuleSerializer.md)> |  | [optional]
**dependent_rules** | Option<[**Vec<crate::models::ApiV1EmployeesDependentRuleSerializer>**](ApiV1EmployeesDependentRuleSerializer.md)> | 扶養親族 | [optional]
**bank_account_rule** | Option<[**crate::models::ApiV1EmployeesBankAccountRuleSerializer**](ApiV1EmployeesBankAccountRuleSerializer.md)> |  | [optional]
**basic_pay_rule** | Option<[**crate::models::ApiV1EmployeesBasicPayRuleSerializer**](ApiV1EmployeesBasicPayRuleSerializer.md)> |  | [optional]
**payroll_calculation** | Option<**bool**> | 給与計算対象従業員の場合trueを返します | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


