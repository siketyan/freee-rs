# ApiV1EmployeesWorkRecordSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**break_records** | Option<[**Vec<crate::models::ApiV1EmployeesWorkRecordTimeRangeResponseSerializer>**](ApiV1EmployeesWorkRecordTimeRangeResponseSerializer.md)> | 休憩時間のリスト | [optional]
**clock_in_at** | Option<**String**> | 出勤時間 | [optional]
**clock_out_at** | Option<**String**> | 退勤時間 | [optional]
**date** | Option<**String**> | 対象日付 | [optional]
**day_pattern** | Option<**String**> | 勤務パターン - normal_day: 所定労働日 - prescribed_holiday: 所定休日 - legal_holiday: 法定休日 | [optional]
**schedule_pattern** | Option<**String**> | スケジュールパターン - substitute_holiday_work: 振替出勤 - substitute_holiday: 振替休日 - compensatory_holiday_work: 代休出勤 - compensatory_holiday: 代休 - special_holiday: 特別休暇 | [optional]
**early_leaving_mins** | Option<**i32**> | 早退分の時間（分単位） | [optional]
**hourly_paid_holiday_mins** | Option<**i32**> | 時間休を利用した時間（分単位） | [optional]
**is_absence** | Option<**bool**> | 欠勤かどうか | [optional]
**is_editable** | Option<**bool**> | 勤怠データが編集可能かどうか | [optional]
**lateness_mins** | Option<**i32**> | 遅刻分の時間（分単位） | [optional]
**normal_work_clock_in_at** | Option<**String**> | 所定労働開始時刻 | [optional]
**normal_work_clock_out_at** | Option<**String**> | 所定労働終了時刻 | [optional]
**normal_work_mins** | Option<**i32**> | 所定労働時間 | [optional]
**normal_work_mins_by_paid_holiday** | Option<**i32**> | 有給によって計上される所定労働時間（分） | [optional]
**note** | Option<**String**> | 勤怠メモ | [optional]
**paid_holiday** | Option<**f32**> | この日に対する有給取得日数。半休の場合は0.5が入ります。時間休の場合はhourly_paid_holiday_minsを所定労働時間で割った値が入るため、実際の時間を確認するにはhourly_paid_holiday_minsを参照してください。 | [optional]
**use_attendance_deduction** | Option<**bool**> | 欠勤・遅刻・早退を控除対象時間に算入するかどうか | [optional]
**use_default_work_pattern** | Option<**bool**> | デフォルトの勤務時間設定を使っているかどうか | [optional]
**total_overtime_work_mins** | Option<**i32**> | 時間外労働時間（分）（Webの勤怠登録画面にて詳細項目の「勤務時間の長さを自動で計算しない」にチェックを入れた場合0が返却されます。時間外労働時間はtotal_overtime_except_normal_work_minsを参照してください。） | [optional]
**total_holiday_work_mins** | Option<**i32**> | 休日労働時間（分） | [optional]
**total_latenight_work_mins** | Option<**i32**> | 深夜労働時間（分） | [optional]
**not_auto_calc_work_time** | Option<**bool**> | 勤怠登録時に勤務時間の長さを自動で計算しないかどうか | [optional]
**total_excess_statutory_work_mins** | Option<**i32**> | 法定内残業時間（分） | [optional]
**total_latenight_excess_statutory_work_mins** | Option<**i32**> | 深夜の法定内残業時間（分） | [optional]
**total_overtime_except_normal_work_mins** | Option<**i32**> | 所定外法定外労働時間（分） | [optional]
**total_latenight_overtime_except_normal_work_min** | Option<**i32**> | 深夜の所定外法定外労働時間（分） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


