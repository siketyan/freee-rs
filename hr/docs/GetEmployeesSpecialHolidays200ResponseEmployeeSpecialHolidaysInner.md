# GetEmployeesSpecialHolidays200ResponseEmployeeSpecialHolidaysInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 特別休暇付与ID | [optional]
**company_id** | Option<**i32**> | 事業所ID | [optional]
**employee_id** | Option<**i32**> | 従業員ID | [optional]
**special_holiday_setting_id** | Option<**i32**> | 特別休暇設定ID | [optional]
**name** | Option<**String**> | 特別休暇名称 | [optional]
**type_name** | Option<**String**> | 特別休暇・休業休職種別名 | [optional]
**paid_type** | Option<**String**> | 有給・無給区分（paid: 有給、unpaid: 無休） | [optional]
**attendance_rate_calc_type** | Option<**String**> | 出勤率計算方法（in_workdays: 出勤日数に含める、not_in_workdays: 出勤日数に含めない、not_in_total_workdays: 全労働日に含めない） | [optional]
**usage_day** | Option<**String**> | 最小消化単位（full: 全休、half: 半休、hour: 時間休） | [optional]
**valid_date_from** | Option<[**String**](string.md)> | 有効期間開始日(YYYY-MM-DD)(例:2023-01-01) | [optional]
**valid_date_to** | Option<[**String**](string.md)> | 有効期間終了日(YYYY-MM-DD)(例:2023-01-31) | [optional]
**days** | Option<**i32**> | 付与日数 | [optional]
**used** | Option<**f32**> | 使用数 | [optional]
**num_days_and_hours_used** | Option<[**crate::models::GetEmployeesSpecialHolidays200ResponseEmployeeSpecialHolidaysInnerNumDaysAndHoursUsed**](get_employees_special_holidays_200_response_employee_special_holidays_inner_num_days_and_hours_used.md)> |  | [optional]
**left** | Option<**f32**> | 残数 | [optional]
**num_days_and_hours_left** | Option<[**crate::models::GetEmployeesSpecialHolidays200ResponseEmployeeSpecialHolidaysInnerNumDaysAndHoursLeft**](get_employees_special_holidays_200_response_employee_special_holidays_inner_num_days_and_hours_left.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


