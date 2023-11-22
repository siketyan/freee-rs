# ApiV1EmployeesWorkRecordsControllerPeriodUpdateBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID（必須） | 
**break_records** | Option<[**Vec<crate::models::ApiV1EmployeesWorkRecordTimeRangeSerializer>**](ApiV1EmployeesWorkRecordTimeRangeSerializer.md)> | 休憩時間のリスト | [optional]
**clock_in_at** | Option<**String**> | 出勤時刻 | [optional]
**clock_out_at** | Option<**String**> | 退勤時刻 | [optional]
**day_pattern** | Option<**String**> | 勤務パターン（所定労働日: normal_day, 所定休日: prescribed_holiday, 法定休日: legal_holiday）  prescribed_holiday、legal_holidayを指定すると、以下のパラメータについて、指定した値が反映されず無視されます。 - early_leaving_mins - lateness_mins - paid_holiday | [optional]
**early_leaving_mins** | Option<**i32**> | 早退分の時間（分単位） | [optional]
**is_absence** | Option<**bool**> | 欠勤かどうか  is_absenceにtrueを指定すると、以下のパラーメータについて、指定した値が反映されず無視されます。 - break_records   - clock_in_at   - clock_out_at - clock_in_at - clock_out_at - early_leaving_mins - lateness_mins - normal_work_clock_in_at - normal_work_clock_out_at - normal_work_mins - normal_work_mins_by_paid_holiday - paid_holiday | [optional]
**lateness_mins** | Option<**i32**> | 遅刻分の時間（分単位） | [optional]
**normal_work_clock_in_at** | Option<**String**> | 所定労働開始時刻。指定しない場合はデフォルト設定が使用されます。（デフォルト設定は従業員に設定した勤務賃金設定の出退勤時刻と労働時間の設定を参照して値が決まります。） | [optional]
**normal_work_clock_out_at** | Option<**String**> | 所定労働終了時刻。指定しない場合はデフォルト設定が使用されます。（デフォルト設定は従業員に設定した勤務賃金設定の出退勤時刻と労働時間の設定を参照して値が決まります。） | [optional]
**normal_work_mins** | Option<**i32**> | 所定労働時間。指定しない場合はデフォルト設定が使用されます。（デフォルト設定は従業員に設定した勤務賃金設定の出退勤時刻と労働時間の設定を参照して値が決まります。） | [optional]
**normal_work_mins_by_paid_holiday** | Option<**i32**> | 有給によって計上される所定労働時間（分） | [optional]
**note** | Option<**String**> | 勤怠メモ | [optional]
**paid_holiday** | Option<**f32**> | この日の有休取得日数。0.5日単位で指定します。 | [optional]
**use_attendance_deduction** | Option<**bool**> | 欠勤・遅刻・早退を控除対象時間に算入するかどうか | [optional]
**use_default_work_pattern** | Option<**bool**> | デフォルトの勤務設定を使うかどうか。  trueを指定した場合、以下のパラメータについて、指定した値に関係なく、従業員に設定した勤務賃金設定の休日の設定を参照して値が決まります - day_pattern  trueを指定した場合、以下のパラメータについて、指定した値に関係なく、従業員に設定した勤務賃金設定の出退勤時刻と労働時間の設定を参照して値が決まります。 - normal_work_clock_in_at - normal_work_clock_out_at - normal_work_mins | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


