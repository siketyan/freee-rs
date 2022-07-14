# ApiV1EmployeesWorkRecordSummaryControllerUpdateBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID（必須） | 
**work_days** | Option<**f32**> | 総勤務日数 | [optional]
**work_days_on_weekdays** | Option<**f32**> | 所定労働日の勤務日数 | [optional]
**work_days_on_prescribed_holidays** | Option<**f32**> | 所定休日の勤務日数 | [optional]
**work_days_on_legal_holidays** | Option<**f32**> | 法定休日の勤務日数 | [optional]
**total_work_mins** | Option<**i32**> | 労働時間（分） | [optional]
**total_normal_work_mins** | Option<**i32**> | 所定労働時間（分） | [optional]
**total_excess_statutory_work_mins** | Option<**i32**> | 給与計算に用いられる法定内残業時間（分） | [optional]
**total_holiday_work_mins** | Option<**i32**> | 法定休日労働時間（分） | [optional]
**total_latenight_work_mins** | Option<**i32**> | 深夜労働時間（分） | [optional]
**total_actual_excess_statutory_work_mins** | Option<**i32**> | 実労働時間ベースの法定内残業時間（分） | [optional]
**total_overtime_work_mins** | Option<**i32**> | 時間外労働時間（分） | [optional]
**num_absences** | Option<**f32**> | 欠勤日数 | [optional]
**num_absences_for_deduction** | Option<**f32**> | 控除対象の欠勤日数 | [optional]
**total_lateness_mins** | Option<**i32**> | 遅刻時間（分） | [optional]
**total_lateness_mins_for_deduction** | Option<**i32**> | 控除対象の遅刻時間（分） | [optional]
**total_early_leaving_mins** | Option<**i32**> | 早退時間（分） | [optional]
**total_early_leaving_mins_for_deduction** | Option<**i32**> | 控除対象の早退時間（分） | [optional]
**num_paid_holidays** | Option<**f32**> | 有給取得日数 | [optional]
**total_shortage_work_mins** | Option<**i32**> | 不足時間（分）（フレックスタイム制でのみ使用） | [optional]
**total_deemed_paid_excess_statutory_work_mins** | Option<**i32**> | 支給対象の法定内残業時間（分）（裁量労働制でのみ使用） | [optional]
**total_deemed_paid_overtime_except_normal_work_mins** | Option<**i32**> | 支給対象の法定内残業時間（分）（裁量労働制でのみ使用） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


