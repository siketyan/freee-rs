# ApiV1SalariesEmployeePayrollStatementSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 給与明細ID | [optional]
**company_id** | Option<**i32**> | 事業所ID | [optional]
**employee_id** | Option<**i32**> | 従業員ID | [optional]
**employee_name** | Option<**String**> | 従業員の姓名 | [optional]
**employee_display_name** | Option<**String**> | 従業員の表示名 | [optional]
**employee_num** | Option<**String**> | 従業員番号 | [optional]
**pay_date** | Option<[**String**](string.md)> | 支払日 | [optional]
**start_date** | Option<[**String**](string.md)> | 給与計算開始日（固定給） | [optional]
**closing_date** | Option<[**String**](string.md)> | 給与計算締日（固定給） | [optional]
**variable_pay_start_date** | Option<[**String**](string.md)> | 給与計算開始日（変動給） 残業手当、遅刻早退・欠勤などの計算に使われる期間 | [optional]
**variable_pay_closing_date** | Option<[**String**](string.md)> | 給与計算締日（変動給） | [optional]
**fixed** | Option<**bool**> | 給与明細が確定されているかどうか | [optional]
**calc_status** | Option<**String**> | 計算状況ステータス calculating: 計算中, calculated: 計算完了, overwritten: 直接編集, imported: インポート, error: エラー | [optional]
**calculated_at** | Option<**String**> | 計算状況ステータスの更新日 | [optional]
**pay_calc_type** | Option<**String**> | 給与形態 monthly: 月給, daily: 日給, hourly: 時給, (空文字列): 計算中 | [optional]
**board_member_remuneration_amount** | Option<**String**> | 役員報酬 | [optional]
**basic_pay_amount** | Option<**String**> | 基本給 | [optional]
**work_days** | Option<**String**> | 労働日数 | [optional]
**normal_work_time** | Option<**String**> | 労働時間のうち、所定労働時間に該当するもの | [optional]
**normal_work_days** | Option<**String**> | 所定労働出勤日数 | [optional]
**work_mins_by_paid_holiday** | Option<**String**> | 有給休暇時間数 | [optional]
**num_paid_holidays** | Option<**String**> | 有給日数 | [optional]
**is_board_member** | Option<**bool**> | 役員かどうか | [optional]
**total_attendance_deduction_amount** | Option<**String**> | 勤怠控除額合計 | [optional]
**total_allowance_amount** | Option<**String**> | 支給手当額合計 | [optional]
**total_deduction_amount** | Option<**String**> | 控除額合計 | [optional]
**net_payment_amount** | Option<**String**> | 差引支給額(手取り額) | [optional]
**gross_payment_amount** | Option<**String**> | 総支給額(額面) | [optional]
**total_worked_days_count** | Option<**String**> | 平日と休日の合計労働日数（日給用） | [optional]
**total_taxable_payment_amount** | Option<**String**> | 課税対象支給額 | [optional]
**total_expense_amount** | Option<**String**> | 総経費精算額 | [optional]
**total_transfer_amount** | Option<**String**> | 総振込額 | [optional]
**total_annual_payment_amount** | Option<**String**> | 課税支給累計額 | [optional]
**payments** | Option<[**Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer>**](ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer.md)> | 支給項目（基本給、残業代、通勤手当等） | [optional]
**deductions** | Option<[**Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer>**](ApiV1EmployeePayrollStatementsEmployeePayrollStatementItemSerializer.md)> | 控除項目（所得税、住民税、社会保険料等） | [optional]
**attendances** | Option<[**Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeeAttendanceItemSerializer>**](ApiV1EmployeePayrollStatementsEmployeeAttendanceItemSerializer.md)> | 勤怠控除項目（遅刻早退控除、欠勤控除等） | [optional]
**overtime_pays** | Option<[**Vec<crate::models::ApiV1EmployeePayrollStatementsEmployeeOvertimePayItemSerializer>**](ApiV1EmployeePayrollStatementsEmployeeOvertimePayItemSerializer.md)> | 時間外労働項目(法定内残業、時間外労働、休日労働、深夜労働等) | [optional]
**remark** | Option<**String**> | 備考 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


