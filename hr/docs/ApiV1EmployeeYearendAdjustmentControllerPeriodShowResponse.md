# ApiV1EmployeeYearendAdjustmentControllerPeriodShowResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**employee** | Option<[**crate::models::ApiV1EmployeeYearendAdjustmentEmployeeSerializer**](ApiV1EmployeeYearendAdjustmentEmployeeSerializer.md)> |  | [optional]
**dependents** | Option<[**Vec<crate::models::ApiV1EmployeeYearendAdjustmentDependentSerializer>**](ApiV1EmployeeYearendAdjustmentDependentSerializer.md)> | 家族情報(年末調整対象外の場合は取得できません。) | [optional]
**insurances** | Option<[**Vec<crate::models::ApiV1EmployeeYearendAdjustmentInsuranceSerializer>**](ApiV1EmployeeYearendAdjustmentInsuranceSerializer.md)> | 保険料情報(年末調整対象外の場合は取得できません。) | [optional]
**housing_loan_deduction** | Option<**i32**> | 住宅借入金等特別控除(年末調整対象外の場合は取得できません。) | [optional]
**housing_loans** | Option<[**Vec<crate::models::ApiV1EmployeeYearendAdjustmentHousingLoanSerializer>**](ApiV1EmployeeYearendAdjustmentHousingLoanSerializer.md)> | 住宅ローン(年末調整対象外の場合は取得できません。) | [optional]
**payroll_and_bonus** | Option<[**crate::models::ApiV1EmployeeYearendAdjustmentPayrollAndBonusSerializer**](ApiV1EmployeeYearendAdjustmentPayrollAndBonusSerializer.md)> |  | [optional]
**previous_job** | Option<[**crate::models::ApiV1EmployeeYearendAdjustmentPreviousJobSerializer**](ApiV1EmployeeYearendAdjustmentPreviousJobSerializer.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


