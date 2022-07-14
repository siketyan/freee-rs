# ApiV1BonusesEmployeePayrollStatementSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 賞与明細ID | [optional]
**company_id** | Option<**i32**> | 事業所ID | [optional]
**employee_id** | Option<**i32**> | 従業員ID | [optional]
**employee_name** | Option<**String**> | 従業員の姓名 | [optional]
**employee_display_name** | Option<**String**> | 従業員の表示名 | [optional]
**employee_num** | Option<**String**> | 従業員番号 | [optional]
**closing_date** | Option<[**String**](string.md)> | 確定日 | [optional]
**pay_date** | Option<[**String**](string.md)> | 支払日 | [optional]
**fixed** | Option<**bool**> | 賞与明細が確定されているかどうか | [optional]
**calc_status** | Option<**String**> | 計算状況ステータス calculating: 計算中, calculated: 計算完了, error: エラー | [optional]
**calculated_at** | Option<**String**> | 計算状況ステータスの更新日 | [optional]
**bonus_amount** | Option<**String**> | 賞与額 | [optional]
**total_allowance_amount** | Option<**String**> | 手当額合計 | [optional]
**total_deduction_amount** | Option<**String**> | 控除額合計 | [optional]
**net_payment_amount** | Option<**String**> | 差引支給額(手取り額) | [optional]
**gross_payment_amount** | Option<**String**> | 総支給額(額面) | [optional]
**total_taxable_payment_amount** | Option<**String**> | 課税対象支給額 | [optional]
**allowances** | Option<[**Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer>**](ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer.md)> | 手当 | [optional]
**deductions** | Option<[**Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer>**](ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer.md)> | 控除項目（所得税、社会保険料等） | [optional]
**remark** | Option<**String**> | 備考 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


