# ApiV1EmployeesWorkRecordSummarySerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**year** | Option<**i32**> | 給与支払い年 | [optional]
**month** | Option<**i32**> | 給与支払い月 | [optional]
**start_date** | Option<[**String**](string.md)> | 集計開始日 | [optional]
**end_date** | Option<[**String**](string.md)> | 集計終了日 | [optional]
**work_days** | Option<**f32**> | 労働日数 | [optional]
**total_work_mins** | Option<**i32**> | 総勤務時間（分） | [optional]
**total_normal_work_mins** | Option<**i32**> | 所定内労働時間（分） | [optional]
**total_excess_statutory_work_mins** | Option<**i32**> | 給与計算に用いられる法定内残業時間（分） | [optional]
**total_overtime_except_normal_work_mins** | Option<**i32**> | 所定外法定外労働時間 | [optional]
**total_overtime_within_normal_work_mins** | Option<**i32**> | 所定内法定外労働時間（裁量労働制の場合はみなしベース） | [optional]
**total_holiday_work_mins** | Option<**i32**> | 法定休日労働時間（分） | [optional]
**total_latenight_work_mins** | Option<**i32**> | 深夜労働allow(company)時間（分） | [optional]
**num_absences** | Option<**f32**> | 欠勤日数 | [optional]
**num_paid_holidays** | Option<**f32**> | 有給取得日数 | [optional]
**num_paid_holidays_and_hours** | Option<[**crate::models::ApiV1HolidaysAndHoursSerializer**](ApiV1HolidaysAndHoursSerializer.md)> |  | [optional]
**num_paid_holidays_left** | Option<**f32**> | 有給残日数 | [optional]
**num_paid_holidays_and_hours_left** | Option<[**crate::models::ApiV1HolidaysAndHoursSerializer**](ApiV1HolidaysAndHoursSerializer.md)> |  | [optional]
**num_substitute_holidays_used** | Option<**f32**> | 振替休日の使用日数 | [optional]
**num_compensatory_holidays_used** | Option<**f32**> | 代休の使用日数 | [optional]
**num_special_holidays_used** | Option<**f32**> | 特別休暇の使用日数 | [optional]
**num_special_holidays_and_hours_used** | Option<[**crate::models::ApiV1HolidaysAndHoursSerializer**](ApiV1HolidaysAndHoursSerializer.md)> |  | [optional]
**total_lateness_and_early_leaving_mins** | Option<**i32**> | 遅刻早退時間（分） | [optional]
**multi_hourly_wages** | Option<[**Vec<crate::models::ApiV1EmployeesEmployeeMultiHourlyWageWorkRecordSummarySerializer>**](ApiV1EmployeesEmployeeMultiHourlyWageWorkRecordSummarySerializer.md)> | 複数時給の労働時間の内訳（複数時給を設定している従業員のみ） | [optional]
**work_records** | Option<[**Vec<crate::models::ApiV1EmployeesWorkRecordSerializer>**](ApiV1EmployeesWorkRecordSerializer.md)> | 日々の勤怠情報 | [optional]
**total_shortage_work_mins** | Option<**i32**> | 不足時間（分） | [optional]
**total_deemed_paid_excess_statutory_work_mins** | Option<**i32**> | 支給対象の法定内残業時間（分） | [optional]
**total_deemed_paid_overtime_except_normal_work_mins** | Option<**i32**> | 支給対象の時間外労働時間（分） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


