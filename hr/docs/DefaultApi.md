# \DefaultApi

All URIs are relative to *https://api.freee.co.jp/hr*

Method | HTTP request | Description
------------- | ------------- | -------------
[**action_approval_requests_monthly_attendance**](DefaultApi.md#action_approval_requests_monthly_attendance) | **POST** /api/v1/approval_requests/monthly_attendances/{id}/actions | 月次勤怠締め申請の承認操作
[**action_approval_requests_overtime_work**](DefaultApi.md#action_approval_requests_overtime_work) | **POST** /api/v1/approval_requests/overtime_works/{id}/actions | 残業申請の承認操作
[**action_approval_requests_paid_holiday**](DefaultApi.md#action_approval_requests_paid_holiday) | **POST** /api/v1/approval_requests/paid_holidays/{id}/actions | 有給申請の承認操作
[**action_approval_requests_special_holiday**](DefaultApi.md#action_approval_requests_special_holiday) | **POST** /api/v1/approval_requests/special_holidays/{id}/actions | 特別休暇申請の承認操作
[**action_approval_requests_work_time**](DefaultApi.md#action_approval_requests_work_time) | **POST** /api/v1/approval_requests/work_times/{id}/actions | 勤務時間修正申請の承認操作
[**bulk_update_employee_dependent_rules**](DefaultApi.md#bulk_update_employee_dependent_rules) | **PUT** /api/v1/employees/{employee_id}/dependent_rules/bulk_update | 従業員の家族情報の更新
[**create_approval_requests_monthly_attendance**](DefaultApi.md#create_approval_requests_monthly_attendance) | **POST** /api/v1/approval_requests/monthly_attendances | 月次勤怠締め申請の作成
[**create_approval_requests_overtime_work**](DefaultApi.md#create_approval_requests_overtime_work) | **POST** /api/v1/approval_requests/overtime_works | 残業申請の作成
[**create_approval_requests_paid_holiday**](DefaultApi.md#create_approval_requests_paid_holiday) | **POST** /api/v1/approval_requests/paid_holidays | 有給申請の作成
[**create_approval_requests_special_holiday**](DefaultApi.md#create_approval_requests_special_holiday) | **POST** /api/v1/approval_requests/special_holidays | 特別休暇申請の作成
[**create_approval_requests_work_time**](DefaultApi.md#create_approval_requests_work_time) | **POST** /api/v1/approval_requests/work_times | 勤務時間修正申請の作成
[**create_employee**](DefaultApi.md#create_employee) | **POST** /api/v1/employees | 従業員の作成
[**create_employee_time_clock**](DefaultApi.md#create_employee_time_clock) | **POST** /api/v1/employees/{employee_id}/time_clocks | 打刻の登録
[**create_group**](DefaultApi.md#create_group) | **POST** /api/v1/groups | 部門の作成
[**create_position**](DefaultApi.md#create_position) | **POST** /api/v1/positions | 役職の作成
[**destroy_approval_requests_monthly_attendance**](DefaultApi.md#destroy_approval_requests_monthly_attendance) | **DELETE** /api/v1/approval_requests/monthly_attendances/{id} | 月次勤怠締め申請の削除
[**destroy_approval_requests_overtime_work**](DefaultApi.md#destroy_approval_requests_overtime_work) | **DELETE** /api/v1/approval_requests/overtime_works/{id} | 残業申請の削除
[**destroy_approval_requests_paid_holiday**](DefaultApi.md#destroy_approval_requests_paid_holiday) | **DELETE** /api/v1/approval_requests/paid_holidays/{id} | 有給申請の削除
[**destroy_approval_requests_special_holiday**](DefaultApi.md#destroy_approval_requests_special_holiday) | **DELETE** /api/v1/approval_requests/special_holidays/{id} | 特別休暇申請の削除
[**destroy_approval_requests_work_time**](DefaultApi.md#destroy_approval_requests_work_time) | **DELETE** /api/v1/approval_requests/work_times/{id} | 勤務時間修正申請の削除
[**destroy_employee**](DefaultApi.md#destroy_employee) | **DELETE** /api/v1/employees/{id} | 従業員の削除
[**destroy_employee_work_record**](DefaultApi.md#destroy_employee_work_record) | **DELETE** /api/v1/employees/{employee_id}/work_records/{date} | 勤怠の削除
[**destroy_group**](DefaultApi.md#destroy_group) | **DELETE** /api/v1/groups/{id} | 部門の削除
[**destroy_position**](DefaultApi.md#destroy_position) | **DELETE** /api/v1/positions/{id} | 役職の削除
[**destroy_yearend_adjustment_housing_loan**](DefaultApi.md#destroy_yearend_adjustment_housing_loan) | **DELETE** /api/v1/yearend_adjustments/{year}/housing_loans/{employee_id}/{id} | 年末調整従業員住宅ローンの削除
[**destroy_yearend_adjustment_insurances**](DefaultApi.md#destroy_yearend_adjustment_insurances) | **DELETE** /api/v1/yearend_adjustments/{year}/insurances/{employee_id}/{id} | 年末調整従業員保険料情報の削除
[**destroy_yearend_adjustment_previous_job**](DefaultApi.md#destroy_yearend_adjustment_previous_job) | **DELETE** /api/v1/yearend_adjustments/{year}/previous_jobs/{employee_id} | 年末調整従業員前職情報の削除
[**get_approval_flow_route**](DefaultApi.md#get_approval_flow_route) | **GET** /api/v1/approval_flow_routes/{id} | 申請経路の取得
[**get_approval_flow_routes**](DefaultApi.md#get_approval_flow_routes) | **GET** /api/v1/approval_flow_routes | 申請経路一覧の取得
[**get_approval_requests_monthly_attendance**](DefaultApi.md#get_approval_requests_monthly_attendance) | **GET** /api/v1/approval_requests/monthly_attendances/{id} | 月次勤怠締め申請の取得
[**get_approval_requests_monthly_attendances**](DefaultApi.md#get_approval_requests_monthly_attendances) | **GET** /api/v1/approval_requests/monthly_attendances | 月次勤怠締め申請一覧の取得
[**get_approval_requests_overtime_work**](DefaultApi.md#get_approval_requests_overtime_work) | **GET** /api/v1/approval_requests/overtime_works/{id} | 残業申請の取得
[**get_approval_requests_overtime_works**](DefaultApi.md#get_approval_requests_overtime_works) | **GET** /api/v1/approval_requests/overtime_works | 残業申請一覧の取得
[**get_approval_requests_paid_holiday**](DefaultApi.md#get_approval_requests_paid_holiday) | **GET** /api/v1/approval_requests/paid_holidays/{id} | 有給申請の取得
[**get_approval_requests_paid_holidays**](DefaultApi.md#get_approval_requests_paid_holidays) | **GET** /api/v1/approval_requests/paid_holidays | 有給申請一覧の取得
[**get_approval_requests_special_holiday**](DefaultApi.md#get_approval_requests_special_holiday) | **GET** /api/v1/approval_requests/special_holidays/{id} | 特別休暇申請の取得
[**get_approval_requests_special_holidays**](DefaultApi.md#get_approval_requests_special_holidays) | **GET** /api/v1/approval_requests/special_holidays | 特別休暇申請一覧の取得
[**get_approval_requests_work_time**](DefaultApi.md#get_approval_requests_work_time) | **GET** /api/v1/approval_requests/work_times/{id} | 勤務時間修正申請の取得
[**get_approval_requests_work_times**](DefaultApi.md#get_approval_requests_work_times) | **GET** /api/v1/approval_requests/work_times | 勤務時間修正申請一覧の取得
[**get_bonuses_employee_payroll_statement**](DefaultApi.md#get_bonuses_employee_payroll_statement) | **GET** /api/v1/bonuses/employee_payroll_statements/{employee_id} | 賞与明細の取得
[**get_bonuses_employee_payroll_statements**](DefaultApi.md#get_bonuses_employee_payroll_statements) | **GET** /api/v1/bonuses/employee_payroll_statements | 賞与明細一覧の取得
[**get_company_employees**](DefaultApi.md#get_company_employees) | **GET** /api/v1/companies/{company_id}/employees | 全期間の従業員一覧の取得
[**get_employee**](DefaultApi.md#get_employee) | **GET** /api/v1/employees/{id} | 従業員の取得
[**get_employee_bank_account_rule**](DefaultApi.md#get_employee_bank_account_rule) | **GET** /api/v1/employees/{employee_id}/bank_account_rule | 従業員の銀行口座の取得
[**get_employee_basic_pay_rule**](DefaultApi.md#get_employee_basic_pay_rule) | **GET** /api/v1/employees/{employee_id}/basic_pay_rule | 従業員の基本給の取得
[**get_employee_dependent_rules**](DefaultApi.md#get_employee_dependent_rules) | **GET** /api/v1/employees/{employee_id}/dependent_rules | 従業員の家族情報の取得
[**get_employee_group_memberships**](DefaultApi.md#get_employee_group_memberships) | **GET** /api/v1/employee_group_memberships | 所属一覧の取得
[**get_employee_health_insurance_rule**](DefaultApi.md#get_employee_health_insurance_rule) | **GET** /api/v1/employees/{employee_id}/health_insurance_rule | 従業員の健康保険の取得
[**get_employee_profile_custom_fields_rule**](DefaultApi.md#get_employee_profile_custom_fields_rule) | **GET** /api/v1/employees/{employee_id}/profile_custom_fields | 従業員のカスタム項目の取得
[**get_employee_profile_rule**](DefaultApi.md#get_employee_profile_rule) | **GET** /api/v1/employees/{employee_id}/profile_rule | 従業員の姓名・住所などの取得
[**get_employee_time_clock**](DefaultApi.md#get_employee_time_clock) | **GET** /api/v1/employees/{employee_id}/time_clocks/{id} | 打刻の取得
[**get_employee_time_clocks**](DefaultApi.md#get_employee_time_clocks) | **GET** /api/v1/employees/{employee_id}/time_clocks | 打刻一覧の取得
[**get_employee_time_clocks_available_types**](DefaultApi.md#get_employee_time_clocks_available_types) | **GET** /api/v1/employees/{employee_id}/time_clocks/available_types | 打刻可能種別の取得
[**get_employee_welfare_pension_insurance_rule**](DefaultApi.md#get_employee_welfare_pension_insurance_rule) | **GET** /api/v1/employees/{employee_id}/welfare_pension_insurance_rule | 従業員の厚生年金保険の取得
[**get_employee_work_record**](DefaultApi.md#get_employee_work_record) | **GET** /api/v1/employees/{employee_id}/work_records/{date} | 勤怠の取得
[**get_employee_work_record_summary**](DefaultApi.md#get_employee_work_record_summary) | **GET** /api/v1/employees/{employee_id}/work_record_summaries/{year}/{month} | 勤怠情報月次サマリの取得
[**get_employees**](DefaultApi.md#get_employees) | **GET** /api/v1/employees | 従業員一覧の取得
[**get_employees_special_holidays**](DefaultApi.md#get_employees_special_holidays) | **GET** /api/v1/employees/{employee_id}/special_holidays | 従業員の特別休暇一覧の取得
[**get_groups**](DefaultApi.md#get_groups) | **GET** /api/v1/groups | 部門一覧の取得
[**get_positions**](DefaultApi.md#get_positions) | **GET** /api/v1/positions | 役職一覧の取得
[**get_salaries_employee_payroll_statement**](DefaultApi.md#get_salaries_employee_payroll_statement) | **GET** /api/v1/salaries/employee_payroll_statements/{employee_id} | 給与明細の取得
[**get_salaries_employee_payroll_statements**](DefaultApi.md#get_salaries_employee_payroll_statements) | **GET** /api/v1/salaries/employee_payroll_statements | 給与明細一覧の取得
[**get_users_me**](DefaultApi.md#get_users_me) | **GET** /api/v1/users/me | ログインユーザーの取得
[**get_yearend_adjustment_employee**](DefaultApi.md#get_yearend_adjustment_employee) | **GET** /api/v1/yearend_adjustments/{year}/employees/{employee_id} | 年末調整の取得
[**get_yearend_adjustment_employees**](DefaultApi.md#get_yearend_adjustment_employees) | **GET** /api/v1/yearend_adjustments/{year}/employees | 年末調整対象一覧の取得
[**post_yearend_adjustment_housing_loan**](DefaultApi.md#post_yearend_adjustment_housing_loan) | **POST** /api/v1/yearend_adjustments/{year}/housing_loans/{employee_id} | 年末調整従業員住宅ローンの作成
[**post_yearend_adjustment_insurances**](DefaultApi.md#post_yearend_adjustment_insurances) | **POST** /api/v1/yearend_adjustments/{year}/insurances/{employee_id} | 年末調整従業員保険料情報の作成
[**put_yearend_adjustment_dependents**](DefaultApi.md#put_yearend_adjustment_dependents) | **PUT** /api/v1/yearend_adjustments/{year}/dependents/{employee_id} | 年末調整家族情報の更新
[**put_yearend_adjustment_employee**](DefaultApi.md#put_yearend_adjustment_employee) | **PUT** /api/v1/yearend_adjustments/{year}/employees/{employee_id} | 年末調整従業員情報の更新
[**put_yearend_adjustment_housing_loan**](DefaultApi.md#put_yearend_adjustment_housing_loan) | **PUT** /api/v1/yearend_adjustments/{year}/housing_loans/{employee_id}/{id} | 年末調整従業員住宅ローンの更新
[**put_yearend_adjustment_housing_loan_deduction**](DefaultApi.md#put_yearend_adjustment_housing_loan_deduction) | **PUT** /api/v1/yearend_adjustments/{year}/housing_loan_deductions/{employee_id} | 年末調整従業員住宅ローン控除額の更新
[**put_yearend_adjustment_insurances**](DefaultApi.md#put_yearend_adjustment_insurances) | **PUT** /api/v1/yearend_adjustments/{year}/insurances/{employee_id}/{id} | 年末調整従業員保険料情報の更新
[**put_yearend_adjustment_payroll_and_bonus**](DefaultApi.md#put_yearend_adjustment_payroll_and_bonus) | **PUT** /api/v1/yearend_adjustments/{year}/payroll_and_bonus/{employee_id} | 年末調整従業員給与・賞与の更新
[**put_yearend_adjustment_previous_job**](DefaultApi.md#put_yearend_adjustment_previous_job) | **PUT** /api/v1/yearend_adjustments/{year}/previous_jobs/{employee_id} | 年末調整従業員前職情報の更新
[**update_approval_requests_monthly_attendance**](DefaultApi.md#update_approval_requests_monthly_attendance) | **PUT** /api/v1/approval_requests/monthly_attendances/{id} | 月次勤怠締め申請の更新
[**update_approval_requests_overtime_work**](DefaultApi.md#update_approval_requests_overtime_work) | **PUT** /api/v1/approval_requests/overtime_works/{id} | 残業申請の更新
[**update_approval_requests_paid_holiday**](DefaultApi.md#update_approval_requests_paid_holiday) | **PUT** /api/v1/approval_requests/paid_holidays/{id} | 有給申請の更新
[**update_approval_requests_special_holiday**](DefaultApi.md#update_approval_requests_special_holiday) | **PUT** /api/v1/approval_requests/special_holidays/{id} | 特別休暇申請の更新
[**update_approval_requests_work_time**](DefaultApi.md#update_approval_requests_work_time) | **PUT** /api/v1/approval_requests/work_times/{id} | 勤務時間修正申請の更新
[**update_employee**](DefaultApi.md#update_employee) | **PUT** /api/v1/employees/{id} | 従業員の更新
[**update_employee_bank_account_rule**](DefaultApi.md#update_employee_bank_account_rule) | **PUT** /api/v1/employees/{employee_id}/bank_account_rule | 従業員の銀行口座の更新
[**update_employee_basic_pay_rule**](DefaultApi.md#update_employee_basic_pay_rule) | **PUT** /api/v1/employees/{employee_id}/basic_pay_rule | 従業員の基本給の更新
[**update_employee_health_insurance_rule**](DefaultApi.md#update_employee_health_insurance_rule) | **PUT** /api/v1/employees/{employee_id}/health_insurance_rule | 従業員の健康保険の更新
[**update_employee_profile_rule**](DefaultApi.md#update_employee_profile_rule) | **PUT** /api/v1/employees/{employee_id}/profile_rule | 従業員の姓名・住所などの更新
[**update_employee_welfare_pension_insurance_rule**](DefaultApi.md#update_employee_welfare_pension_insurance_rule) | **PUT** /api/v1/employees/{employee_id}/welfare_pension_insurance_rule | 従業員の厚生年金保険の更新
[**update_employee_work_record**](DefaultApi.md#update_employee_work_record) | **PUT** /api/v1/employees/{employee_id}/work_records/{date} | 勤怠の更新
[**update_employee_work_record_summary**](DefaultApi.md#update_employee_work_record_summary) | **PUT** /api/v1/employees/{employee_id}/work_record_summaries/{year}/{month} | 勤怠情報月次サマリの更新
[**update_group**](DefaultApi.md#update_group) | **PUT** /api/v1/groups/{id} | 部門の更新
[**update_position**](DefaultApi.md#update_position) | **PUT** /api/v1/positions/{id} | 役職の更新



## action_approval_requests_monthly_attendance

> crate::models::ApiV1MonthlyAttendanceResponse action_approval_requests_monthly_attendance(id, api_v1_approval_action_request)
月次勤怠締め申請の承認操作

<h2 id=\"\">概要</h2> <p>指定した事業所の月次勤怠締め申請情報を承認操作します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 月次勤怠締め申請ID | [required] |
**api_v1_approval_action_request** | Option<[**ApiV1ApprovalActionRequest**](ApiV1ApprovalActionRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1MonthlyAttendanceResponse**](ApiV1MonthlyAttendanceResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## action_approval_requests_overtime_work

> crate::models::ApiV1OvertimeWorkResponse action_approval_requests_overtime_work(id, api_v1_approval_action_request)
残業申請の承認操作

<h2 id=\"\">概要</h2> <p>指定した事業所の残業申請情報を承認操作します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 残業申請ID | [required] |
**api_v1_approval_action_request** | Option<[**ApiV1ApprovalActionRequest**](ApiV1ApprovalActionRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1OvertimeWorkResponse**](ApiV1OvertimeWorkResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## action_approval_requests_paid_holiday

> crate::models::ApiV1PaidHolidayResponse action_approval_requests_paid_holiday(id, api_v1_approval_action_request)
有給申請の承認操作

<h2 id=\"\">概要</h2> <p>指定した事業所の有給申請情報を承認操作します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>全休の有給申請は承認されると申請者の有給の残数が減ります。</li>   <li>半休と時間休の有給申請は承認されても申請者の有給の残数が減らない場合があります。以下の条件を満たす場合、申請者の有給の残数が減ります。</li>   <ul>     <li>申請承認後、申請者が申請の対象日に出勤打刻と退勤打刻をする。</li>     <li>申請承認前に、申請者が申請の対象日に勤怠を登録している。</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 有給申請ID | [required] |
**api_v1_approval_action_request** | Option<[**ApiV1ApprovalActionRequest**](ApiV1ApprovalActionRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1PaidHolidayResponse**](ApiV1PaidHolidayResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## action_approval_requests_special_holiday

> crate::models::ApiV1specialHolidayResponse action_approval_requests_special_holiday(id, api_v1_approval_action_request)
特別休暇申請の承認操作

<h2 id=\"\">概要</h2> <p>指定した事業所の特別休暇申請情報を承認操作します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>全休の特別休暇申請は承認されると申請者の特別休暇の残数が減ります。</li>   <li>半休と時間休の特別休暇申請は承認されても申請者の特別休暇の残数が減らない場合があります。以下の条件を満たす場合、申請者の特別休暇の残数が減ります。</li>   <ul>     <li>申請承認後、申請者が申請の対象日に出勤打刻と退勤打刻をする。</li>     <li>申請承認前に、申請者が申請の対象日に勤怠を登録している。</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 特別休暇申請ID | [required] |
**api_v1_approval_action_request** | Option<[**ApiV1ApprovalActionRequest**](ApiV1ApprovalActionRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1specialHolidayResponse**](ApiV1specialHolidayResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## action_approval_requests_work_time

> serde_json::Value action_approval_requests_work_time(id, api_v1_approval_action_request)
勤務時間修正申請の承認操作

<h2 id=\"\">概要</h2> <p>指定した事業所の勤務時間修正申請情報を承認操作します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 勤務時間修正申請ID | [required] |
**api_v1_approval_action_request** | Option<[**ApiV1ApprovalActionRequest**](ApiV1ApprovalActionRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_employee_dependent_rules

> crate::models::ApiV1EmployeesDependentRulesControllerPeriodBulkUpdateResponse bulk_update_employee_dependent_rules(employee_id, body)
従業員の家族情報の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の家族情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>idがない場合は新規作成、destroyがtrueの場合は削除になります。</li>   <li>residence_type=live_in: 同居の場合、以下のパラメータに指定した値はWebに反映されません。</li>   <ul>     <li>zipcode1</li>     <li>zipcode2</li>     <li>prefecture_code</li>     <li>address</li>     <li>address_kana</li>     <li>annual_remittance_amount</li>   </ul>   <li>residence_type=non_resident: 別居(国外)の場合、以下のパラメータに指定した値はWebに反映されません。</li>   <ul>     <li>prefecture_code</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesDependentRulesControllerPeriodBulkUpdateBody**](ApiV1EmployeesDependentRulesControllerPeriodBulkUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesDependentRulesControllerPeriodBulkUpdateResponse**](ApiV1EmployeesDependentRulesController.bulk_update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_approval_requests_monthly_attendance

> crate::models::ApiV1MonthlyAttendanceResponse create_approval_requests_monthly_attendance(api_v1_monthly_attendance_create_request)
月次勤怠締め申請の作成

<h2 id=\"\">概要</h2> <p>指定した事業所の月次勤怠締め申請を新規作成します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v1_monthly_attendance_create_request** | Option<[**ApiV1MonthlyAttendanceCreateRequest**](ApiV1MonthlyAttendanceCreateRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1MonthlyAttendanceResponse**](ApiV1MonthlyAttendanceResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_approval_requests_overtime_work

> crate::models::ApiV1OvertimeWorkResponse create_approval_requests_overtime_work(api_v1_overtime_work_request)
残業申請の作成

<h2 id=\"\">概要</h2> <p>指定した事業所の残業申請を新規作成します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v1_overtime_work_request** | Option<[**ApiV1OvertimeWorkRequest**](ApiV1OvertimeWorkRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1OvertimeWorkResponse**](ApiV1OvertimeWorkResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_approval_requests_paid_holiday

> crate::models::ApiV1PaidHolidayResponse create_approval_requests_paid_holiday(api_v1_paid_holiday_request)
有給申請の作成

<h2 id=\"\">概要</h2> <p>指定した事業所の有給申請を新規作成します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v1_paid_holiday_request** | Option<[**ApiV1PaidHolidayRequest**](ApiV1PaidHolidayRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1PaidHolidayResponse**](ApiV1PaidHolidayResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_approval_requests_special_holiday

> crate::models::ApiV1specialHolidayResponse create_approval_requests_special_holiday(api_v1special_holiday_request)
特別休暇申請の作成

<h2 id=\"\">概要</h2> <p>指定した事業所の特別休暇申請を新規作成します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v1special_holiday_request** | Option<[**ApiV1specialHolidayRequest**](ApiV1specialHolidayRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1specialHolidayResponse**](ApiV1specialHolidayResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_approval_requests_work_time

> crate::models::CreateApprovalRequestsWorkTime201Response create_approval_requests_work_time(create_approval_requests_work_time_request)
勤務時間修正申請の作成

<h2 id=\"\">概要</h2> <p>指定した事業所の勤務時間修正を新規作成します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_approval_requests_work_time_request** | Option<[**CreateApprovalRequestsWorkTimeRequest**](CreateApprovalRequestsWorkTimeRequest.md)> |  |  |

### Return type

[**crate::models::CreateApprovalRequestsWorkTime201Response**](create_approval_requests_work_time_201_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_employee

> crate::models::ApiV1EmployeesControllerPeriodCreateResponse create_employee(body)
従業員の作成

<h2 id=\"\">概要</h2> <p>従業員を新規作成します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV1EmployeesControllerPeriodCreateBody**](ApiV1EmployeesControllerPeriodCreateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesControllerPeriodCreateResponse**](ApiV1EmployeesController.create_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_employee_time_clock

> crate::models::ApiV1EmployeesTimeClocksControllerPeriodCreateResponse create_employee_time_clock(employee_id, body)
打刻の登録

<h2 id=\"\">概要</h2> <p>指定した従業員の打刻情報を登録します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>休憩開始の連続や退勤のみなど、整合性の取れていない打刻は登録できません。 打刻可能種別の取得APIを呼ぶことで、その従業員がその時点で登録可能な打刻種別が取得できます。</li>    <li>出勤の打刻は</li>   <ul>     <li>前日の出勤時刻から24時間以内の場合、前日の退勤打刻が必須です。</li>     <li> 前日の出勤時刻から24時間経過している場合は、前日の退勤打刻がなくとも出勤打刻を登録することができます。</li>   </ul>    <li>退勤の打刻は</li>   <ul>     <li><a href=\\\"https://support.freee.co.jp/hc/ja/articles/900004490226-%E5%8B%A4%E6%80%A0%E5%9F%BA%E6%9C%AC%E8%A8%AD%E5%AE%9A%E3%82%92%E8%A1%8C%E3%81%86#h_01EYPYTR9HZ7YB8V5F18VMD1BT\"target=\\\"_blank\\\">『退勤を自動打刻する』</a>の設定を使用している場合は、出勤打刻から24時間経過しても退勤打刻がない場合に、退勤打刻が自動で登録されます。</li>     <li>すでに登録されている退勤打刻よりも後の時刻であれば上書き登録することができます。</li>   </ul>    <li>打刻が日をまたぐ場合は、base_date(打刻日)に前日の日付を指定してください。</li>    <li>datetime(打刻日時)を指定できるのは管理者か事務担当者の権限を持ったユーザーのみです。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesTimeClocksControllerPeriodCreateBody**](ApiV1EmployeesTimeClocksControllerPeriodCreateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesTimeClocksControllerPeriodCreateResponse**](ApiV1EmployeesTimeClocksController.create_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> crate::models::ApiV1GroupResponse create_group(api_v1_group_create_request)
部門の作成

<h2 id=\"\">概要</h2> <p>指定した事業所の部門を新規作成します。</p> <p>部門APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/groups-api-hierarchy\" target=\"_blank\">部門APIを利用した組織図の取得について</a> をご参照ください。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v1_group_create_request** | Option<[**ApiV1GroupCreateRequest**](ApiV1GroupCreateRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1GroupResponse**](ApiV1GroupResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_position

> crate::models::ApiV1PositionResponse create_position(api_v1_position_request)
役職の作成

<h2 id=\"\">概要</h2> <p>指定した事業所の役職を新規作成します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v1_position_request** | Option<[**ApiV1PositionRequest**](ApiV1PositionRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1PositionResponse**](ApiV1PositionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_approval_requests_monthly_attendance

> destroy_approval_requests_monthly_attendance(id, company_id)
月次勤怠締め申請の削除

<h2 id=\"\">概要</h2> <p>指定した事業所の月次勤怠締め申請情報を削除します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 月次勤怠締め申請ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_approval_requests_overtime_work

> destroy_approval_requests_overtime_work(id, company_id)
残業申請の削除

<h2 id=\"\">概要</h2> <p>指定した事業所の残業申請情報を削除します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 残業申請ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_approval_requests_paid_holiday

> destroy_approval_requests_paid_holiday(id, company_id)
有給申請の削除

<h2 id=\"\">概要</h2> <p>指定した事業所の有給申請情報を削除します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 有給申請ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_approval_requests_special_holiday

> destroy_approval_requests_special_holiday(id, company_id)
特別休暇申請の削除

<h2 id=\"\">概要</h2> <p>指定した事業所の特別休暇申請情報を削除します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 特別休暇申請ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_approval_requests_work_time

> destroy_approval_requests_work_time(id, company_id)
勤務時間修正申請の削除

<h2 id=\"\">概要</h2> <p>指定した事業所の勤務時間修正申請情報を削除します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 勤務時間修正申請ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_employee

> destroy_employee(id, company_id)
従業員の削除

<h2 id=\"\">概要</h2> <p>指定したIDの従業員を削除します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 従業員ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_employee_work_record

> destroy_employee_work_record(employee_id, date, company_id)
勤怠の削除

<h2 id=\"\">概要</h2> <p>指定した従業員の勤怠情報を削除します。</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**date** | **String** | 削除対象年月日(YYYY-MM-DD)(例:2018-08-01) | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_group

> destroy_group(id, company_id)
部門の削除

<h2 id=\"\">概要</h2> <p>指定した事業所の部門の情報を削除します。</p> <p>部門APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/groups-api-hierarchy\" target=\"_blank\">部門APIを利用した組織図の取得について</a> をご参照ください。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 部門ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_position

> destroy_position(id, company_id)
役職の削除

<h2 id=\"\">概要</h2> <p>指定した事業所の役職の情報を削除します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>従業員に役職が適用されている場合、従業員の役職情報も削除されます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 役職ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_yearend_adjustment_housing_loan

> destroy_yearend_adjustment_housing_loan(company_id, year, employee_id, id)
年末調整従業員住宅ローンの削除

<h2 id=\"\">概要</h2> <p>指定した従業員の住宅ローンを削除します。</p> <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 更新対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**id** | **i32** | 住宅ローンID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_yearend_adjustment_insurances

> destroy_yearend_adjustment_insurances(company_id, year, employee_id, id)
年末調整従業員保険料情報の削除

<h2 id=\"\">概要</h2> <p>指定した従業員の保険料情報を削除します。</p> <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 更新対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**id** | **i32** | 保険料ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_yearend_adjustment_previous_job

> destroy_yearend_adjustment_previous_job(company_id, year, employee_id)
年末調整従業員前職情報の削除

<h2 id=\"\">概要</h2> <p>指定した従業員の前職情報を削除します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 更新対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_flow_route

> crate::models::ApiV1ApprovalFlowRouteResponse get_approval_flow_route(id, company_id)
申請経路の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の申請経路を取得する。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>指定した事業所の従業員に紐づくユーザーのみ実行可能です。</li>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 申請経路ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::ApiV1ApprovalFlowRouteResponse**](ApiV1ApprovalFlowRouteResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_flow_routes

> crate::models::ApiV1ApprovalFlowRoutesIndexResponse get_approval_flow_routes(company_id, included_user_id, usage)
申請経路一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の申請経路一覧を取得する。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>指定した事業所の従業員に紐づくユーザーのみ実行可能です。</li>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**included_user_id** | Option<**i32**> | 経路に含まれるユーザーのユーザーID |  |
**usage** | Option<**String**> | 申請種別（申請経路を使用できる申請種別を示します。例えば、AttendanceWorkflow の場合は、勤怠申請で使用できる申請経路です。） - `AttendanceWorkflow` - 勤怠申請 - `PersonalDataWorkflow` - 身上変更申請 |  |

### Return type

[**crate::models::ApiV1ApprovalFlowRoutesIndexResponse**](ApiV1ApprovalFlowRoutesIndexResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests_monthly_attendance

> crate::models::ApiV1MonthlyAttendanceResponse get_approval_requests_monthly_attendance(company_id, id)
月次勤怠締め申請の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の月次勤怠締め申請情報を取得します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** | 月次勤怠締め申請ID | [required] |

### Return type

[**crate::models::ApiV1MonthlyAttendanceResponse**](ApiV1MonthlyAttendanceResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests_monthly_attendances

> crate::models::ApiV1MonthlyAttendanceIndexResponse get_approval_requests_monthly_attendances(company_id, status, application_number, start_issue_date, end_issue_date, approver_id, applicant_id, start_target_date, end_target_date, passed_auto_check, limit, offset)
月次勤怠締め申請一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の指定日付時点における月次勤怠締め申請情報をリストで返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**status** | Option<**String**> | 申請ステータス - `draft` - 下書き - `in_progress` - 申請中 - `approved` - 承認済 - `feedback` - 差戻し |  |
**application_number** | Option<**i32**> | 申請No |  |
**start_issue_date** | Option<**String**> | 申請開始日 |  |
**end_issue_date** | Option<**String**> | 申請終了日 |  |
**approver_id** | Option<**i32**> | 現在承認ステップの承認者のユーザーID  approver_idに値を指定する場合、指定なしの申請経路を利用した申請は返却されません  |  |
**applicant_id** | Option<**i32**> | 申請者のユーザーID |  |
**start_target_date** | Option<**String**> | 対象開始日 |  |
**end_target_date** | Option<**String**> | 対象終了日 |  |
**passed_auto_check** | Option<**bool**> | 自動チェック結果 - trueを指定した場合、自動チェック結果がtrueの申請のみ返却します。 - falseを指定した場合、自動チェック結果がfalseの申請のみ返却します。 - キーごと指定しない場合、すべての申請を返却します。 |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::ApiV1MonthlyAttendanceIndexResponse**](ApiV1MonthlyAttendanceIndexResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests_overtime_work

> crate::models::ApiV1OvertimeWorkResponse get_approval_requests_overtime_work(company_id, id)
残業申請の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の残業申請情報を取得します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** | 残業申請ID | [required] |

### Return type

[**crate::models::ApiV1OvertimeWorkResponse**](ApiV1OvertimeWorkResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests_overtime_works

> crate::models::ApiV1OvertimeWorkIndexResponse get_approval_requests_overtime_works(company_id, status, application_number, start_issue_date, end_issue_date, approver_id, applicant_id, start_target_date, end_target_date, passed_auto_check, limit, offset)
残業申請一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の指定日付時点における残業申請情報をリストで返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**status** | Option<**String**> | 申請ステータス - `draft` - 下書き - `in_progress` - 申請中 - `approved` - 承認済 - `feedback` - 差戻し |  |
**application_number** | Option<**i32**> | 申請No |  |
**start_issue_date** | Option<**String**> | 申請開始日 |  |
**end_issue_date** | Option<**String**> | 申請終了日 |  |
**approver_id** | Option<**i32**> | 現在承認ステップの承認者のユーザーID  approver_idに値を指定する場合、指定なしの申請経路を利用した申請は返却されません  |  |
**applicant_id** | Option<**i32**> | 申請者のユーザーID |  |
**start_target_date** | Option<**String**> | 対象開始日 |  |
**end_target_date** | Option<**String**> | 対象終了日 |  |
**passed_auto_check** | Option<**bool**> | 自動チェック結果 - trueを指定した場合、自動チェック結果がtrueの申請のみ返却します。 - falseを指定した場合、自動チェック結果がfalseの申請のみ返却します。 - キーごと指定しない場合、すべての申請を返却します。 |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::ApiV1OvertimeWorkIndexResponse**](ApiV1OvertimeWorkIndexResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests_paid_holiday

> crate::models::ApiV1PaidHolidayResponse get_approval_requests_paid_holiday(company_id, id)
有給申請の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の有給申請情報を取得します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** | 有給申請ID | [required] |

### Return type

[**crate::models::ApiV1PaidHolidayResponse**](ApiV1PaidHolidayResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests_paid_holidays

> crate::models::ApiV1PaidHolidayIndexResponse get_approval_requests_paid_holidays(company_id, status, application_number, start_issue_date, end_issue_date, approver_id, applicant_id, start_target_date, end_target_date, passed_auto_check, limit, offset)
有給申請一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の指定日付時点における有給申請情報をリストで返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**status** | Option<**String**> | 申請ステータス - `draft` - 下書き - `in_progress` - 申請中 - `approved` - 承認済 - `feedback` - 差戻し |  |
**application_number** | Option<**i32**> | 申請No |  |
**start_issue_date** | Option<**String**> | 申請開始日 |  |
**end_issue_date** | Option<**String**> | 申請終了日 |  |
**approver_id** | Option<**i32**> | 現在承認ステップの承認者のユーザーID  approver_idに値を指定する場合、指定なしの申請経路を利用した申請は返却されません  |  |
**applicant_id** | Option<**i32**> | 申請者のユーザーID |  |
**start_target_date** | Option<**String**> | 対象開始日 |  |
**end_target_date** | Option<**String**> | 対象終了日 |  |
**passed_auto_check** | Option<**bool**> | 自動チェック結果 - trueを指定した場合、自動チェック結果がtrueの申請のみ返却します。 - falseを指定した場合、自動チェック結果がfalseの申請のみ返却します。 - キーごと指定しない場合、すべての申請を返却します。 |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::ApiV1PaidHolidayIndexResponse**](ApiV1PaidHolidayIndexResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests_special_holiday

> crate::models::ApiV1specialHolidayResponse get_approval_requests_special_holiday(company_id, id)
特別休暇申請の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の特別休暇申請情報を取得します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** | 特別休暇申請ID | [required] |

### Return type

[**crate::models::ApiV1specialHolidayResponse**](ApiV1specialHolidayResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests_special_holidays

> crate::models::ApiV1specialHolidayIndexResponse get_approval_requests_special_holidays(company_id, status, application_number, start_issue_date, end_issue_date, approver_id, applicant_id, start_target_date, end_target_date, passed_auto_check, limit, offset)
特別休暇申請一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の指定日付時点における特別休暇申請情報をリストで返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**status** | Option<**String**> | 申請ステータス - `draft` - 下書き - `in_progress` - 申請中 - `approved` - 承認済 - `feedback` - 差戻し |  |
**application_number** | Option<**i32**> | 申請No |  |
**start_issue_date** | Option<**String**> | 申請開始日 |  |
**end_issue_date** | Option<**String**> | 申請終了日 |  |
**approver_id** | Option<**i32**> | 現在承認ステップの承認者のユーザーID  approver_idに値を指定する場合、指定なしの申請経路を利用した申請は返却されません  |  |
**applicant_id** | Option<**i32**> | 申請者のユーザーID |  |
**start_target_date** | Option<**String**> | 対象開始日 |  |
**end_target_date** | Option<**String**> | 対象終了日 |  |
**passed_auto_check** | Option<**bool**> | 自動チェック結果 - trueを指定した場合、自動チェック結果がtrueの申請のみ返却します。 - falseを指定した場合、自動チェック結果がfalseの申請のみ返却します。 - キーごと指定しない場合、すべての申請を返却します。 |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::ApiV1specialHolidayIndexResponse**](ApiV1specialHolidayIndexResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests_work_time

> serde_json::Value get_approval_requests_work_time(company_id, id)
勤務時間修正申請の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の勤務時間修正申請情報を取得します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** | 勤務時間修正申請ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests_work_times

> crate::models::GetApprovalRequestsWorkTimes200Response get_approval_requests_work_times(company_id, status, application_number, start_issue_date, end_issue_date, approver_id, applicant_id, start_target_date, end_target_date, passed_auto_check, limit, offset)
勤務時間修正申請一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の指定日付時点における勤務時間修正申請情報をリストで返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**status** | Option<**String**> | 申請ステータス - `draft` - 下書き - `in_progress` - 申請中 - `approved` - 承認済 - `feedback` - 差戻し |  |
**application_number** | Option<**i32**> | 申請No |  |
**start_issue_date** | Option<**String**> | 申請開始日 |  |
**end_issue_date** | Option<**String**> | 申請終了日 |  |
**approver_id** | Option<**i32**> | 現在承認ステップの承認者のユーザーID approver_idに値を指定する場合、指定なしの申請経路を利用した申請は返却されません  |  |
**applicant_id** | Option<**i32**> | 申請者のユーザーID |  |
**start_target_date** | Option<**String**> | 対象開始日 |  |
**end_target_date** | Option<**String**> | 対象終了日 |  |
**passed_auto_check** | Option<**bool**> | 自動チェック結果 - trueを指定した場合、自動チェック結果がtrueの申請のみ返却します。 - falseを指定した場合、自動チェック結果がfalseの申請のみ返却します。 - キーごと指定しない場合、すべての申請を返却します。 |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::GetApprovalRequestsWorkTimes200Response**](get_approval_requests_work_times_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bonuses_employee_payroll_statement

> crate::models::ApiV1BonusesEmployeePayrollStatementsControllerPeriodShowResponse get_bonuses_employee_payroll_statement(company_id, year, month, employee_id)
賞与明細の取得

<h2 id=\"\">概要</h2> <p>指定した従業員ID、年月の賞与明細を返します。</p> <p>指定した年月に支払いのある賞与明細が返されます。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>  <h2 id=\"_2\">examples</h2> <pre>   <code>     {     \"employee_payroll_statement\": {       \"id\": 1,       \"company_id\": 1,       \"employee_id\": 1,       \"employee_name\": \"給与 太郎\",       \"employee_display_name\": \"給与 太郎\",       \"employee_num\": \"001\",       \"closing_date\": \"2018-03-31\",       \"pay_date\": \"2018-03-31\",       \"fixed\": true,       \"calc_status\": \"calculated\",       \"calculated_at\": \"2018-09-27T05:06:45.315Z\",       \"bonus_amount\": \"300000.0\",       \"total_allowance_amount\": \"0.0\",       \"total_deduction_amount\": \"23830.0\",       \"net_payment_amount\": \"268000.0\",       \"gross_payment_amount\": \"330000.0\",       \"total_taxable_payment_amount\": \"330000.0\",       \"allowances\": [{\"name\": \"地域手当\", \"amount\": \"30000.0\"}],       \"deductions\": [{\"name\": \"所得税\", \"amount\": \"46000.0\"}, {\"name\": \"健康保険料\", \"amount\": \"16000.0\"}],       \"remark\": \"備考\"     }     }   </code> </pre>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1BonusesEmployeePayrollStatementsControllerPeriodShowResponse**](ApiV1BonusesEmployeePayrollStatementsController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bonuses_employee_payroll_statements

> crate::models::ApiV1BonusesEmployeePayrollStatementsIndexSerializer get_bonuses_employee_payroll_statements(company_id, year, month, limit, offset)
賞与明細一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所に所属する従業員の賞与明細をリストで返します。</p> <p>指定した年月に支払いのある賞与明細が返されます。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月 | [required] |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::ApiV1BonusesEmployeePayrollStatementsIndexSerializer**](ApiV1BonusesEmployeePayrollStatementsIndexSerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company_employees

> Vec<crate::models::ApiV1CompaniesEmployeeSerializer> get_company_employees(company_id, limit, offset, with_no_payroll_calculation)
全期間の従業員一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所に所属する従業員をリストで返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>退職ユーザーも含めて取得可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |
**with_no_payroll_calculation** | Option<**bool**> | trueを指定すると給与計算対象外の従業員情報をレスポンスに含めます。 |  |[default to false]

### Return type

[**Vec<crate::models::ApiV1CompaniesEmployeeSerializer>**](ApiV1CompaniesEmployeeSerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee

> crate::models::ApiV1EmployeesControllerPeriodShowResponse get_employee(company_id, year, month, id)
従業員の取得

<h2 id=\"\">概要</h2> <p>指定したIDの従業員を返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>指定した年月に退職済みユーザーは取得できません。</li>   <li>保険料計算方法が自動計算の場合、対応する保険料の直接指定金額は無視されnullが返されます。(例: 給与計算時の健康保険料の計算方法が自動計算の場合、給与計算時の健康保険料の直接指定金額はnullが返されます)</li>   <li>事業所が定額制の健康保険組合に加入している場合、保険料の直接指定金額は無視されnullが返されます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が検索結果として返します。<br> 翌月払いの従業員の2022/01の従業員情報を取得する場合は、year=2021,month=12を指定してください。<br> | [required] |
**id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesControllerPeriodShowResponse**](ApiV1EmployeesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_bank_account_rule

> crate::models::ApiV1EmployeesBankAccountRulesControllerPeriodShowResponse get_employee_bank_account_rule(company_id, year, month, employee_id)
従業員の銀行口座の取得

<h2 id=\"\">概要</h2> <p>指定した従業員・日付の銀行口座情報を返します。</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が検索結果として返します。<br> 翌月払いの従業員の2022/01の従業員情報を取得する場合は、year=2021,month=12を指定してください。<br> | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesBankAccountRulesControllerPeriodShowResponse**](ApiV1EmployeesBankAccountRulesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_basic_pay_rule

> crate::models::ApiV1EmployeesBasicPayRulesControllerPeriodShowResponse get_employee_basic_pay_rule(company_id, year, month, employee_id)
従業員の基本給の取得

<h2 id=\"\">概要</h2> <p>指定した従業員・日付の基本給情報を返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が検索結果として返します。<br> 翌月払いの従業員の2022/01の従業員情報を取得する場合は、year=2021,month=12を指定してください。<br> | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesBasicPayRulesControllerPeriodShowResponse**](ApiV1EmployeesBasicPayRulesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_dependent_rules

> crate::models::ApiV1EmployeesDependentRulesControllerPeriodIndexResponse get_employee_dependent_rules(company_id, year, month, employee_id)
従業員の家族情報の取得

<h2 id=\"\">概要</h2> <p>指定した従業員・日付の家族情報を返します。</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が検索結果として返します。<br> 翌月払いの従業員の2022/01の従業員情報を取得する場合は、year=2021,month=12を指定してください。<br> | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesDependentRulesControllerPeriodIndexResponse**](ApiV1EmployeesDependentRulesController.index_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_group_memberships

> crate::models::ApiV1EmployeeGroupMembershipsIndexSerializer get_employee_group_memberships(company_id, base_date, with_no_payroll_calculation, limit, offset)
所属一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の指定日付時点における所属情報をリストで返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**base_date** | **String** | 指定日。指定日付時点における所属情報をリストで返します。(YYYY-MM-DD)(例:2018-07-31) | [required] |
**with_no_payroll_calculation** | Option<**bool**> | trueを指定すると給与計算対象外の従業員情報をレスポンスに含めます。 |  |[default to false]
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::ApiV1EmployeeGroupMembershipsIndexSerializer**](ApiV1EmployeeGroupMembershipsIndexSerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_health_insurance_rule

> crate::models::ApiV1EmployeesHealthInsuranceRulesControllerPeriodShowResponse get_employee_health_insurance_rule(company_id, year, month, employee_id)
従業員の健康保険の取得

<h2 id=\"\">概要</h2> <p>指定した従業員・日付の健康保険情報を返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>保険料計算方法が自動計算の場合、対応する保険料の直接指定金額は無視されnullが返されます。(例: 給与計算時の健康保険料の計算方法が自動計算の場合、給与計算時の健康保険料の直接指定金額はnullが返されます)</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が検索結果として返します。<br> 翌月払いの従業員の2022/01の従業員情報を取得する場合は、year=2021,month=12を指定してください。<br> | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesHealthInsuranceRulesControllerPeriodShowResponse**](ApiV1EmployeesHealthInsuranceRulesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_profile_custom_fields_rule

> crate::models::ApiV1EmployeesProfileCustomFieldRulesControllerPeriodIndexResponse get_employee_profile_custom_fields_rule(company_id, year, month, employee_id)
従業員のカスタム項目の取得

<h2 id=\"\">概要</h2> <p>指定した従業員・日付のカスタム項目情報を返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>指定年月に在籍していない従業員および給与計算対象外の従業員ではデータが存在しないため、空の配列が返ります。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が検索結果として返します。<br> 翌月払いの従業員の2022/01の従業員情報を取得する場合は、year=2021,month=12を指定してください。<br> | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesProfileCustomFieldRulesControllerPeriodIndexResponse**](ApiV1EmployeesProfileCustomFieldRulesController.index_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_profile_rule

> crate::models::ApiV1EmployeesProfileRulesControllerPeriodShowResponse get_employee_profile_rule(company_id, year, month, employee_id)
従業員の姓名・住所などの取得

<h2 id=\"\">概要</h2> <p>指定した従業員・日付の姓名などの情報を返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、給与計算対象外の従業員には非対応です。employee_idに給与計算対象外の従業員IDを指定した場合、本APIを利用できません。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月<br> 締日支払日設定が翌月払いの従業員情報の場合は、指定したmonth + 1の値が検索結果として返します。<br> 翌月払いの従業員の2022/01の従業員情報を取得する場合は、year=2021,month=12を指定してください。<br> | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesProfileRulesControllerPeriodShowResponse**](ApiV1EmployeesProfileRulesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_time_clock

> crate::models::ApiV1EmployeesTimeClocksControllerPeriodShowResponse get_employee_time_clock(company_id, employee_id, id)
打刻の取得

<h2 id=\"\">概要</h2> <p>指定した従業員・指定した打刻の詳細情報を返します。</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**id** | **i32** | 打刻ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesTimeClocksControllerPeriodShowResponse**](ApiV1EmployeesTimeClocksController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_time_clocks

> Vec<crate::models::ApiV1EmployeesTimeClockSerializer> get_employee_time_clocks(company_id, employee_id, from_date, to_date, limit, offset)
打刻一覧の取得

<h2 id=\"\">概要</h2> <p>指定した従業員・期間の打刻情報を返します。</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**from_date** | Option<**String**> | 取得する打刻期間の開始日(YYYY-MM-DD)(例:2018-08-01)(デフォルト: 当月の打刻開始日) |  |
**to_date** | Option<**String**> | 取得する打刻期間の終了日(YYYY-MM-DD)(例:2018-08-31)(デフォルト: 当日) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**Vec<crate::models::ApiV1EmployeesTimeClockSerializer>**](ApiV1EmployeesTimeClockSerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_time_clocks_available_types

> crate::models::ApiV1EmployeesTimeClocksControllerPeriodAvailableTypesResponse get_employee_time_clocks_available_types(company_id, employee_id, date)
打刻可能種別の取得

<h2 id=\"\">概要</h2> <p>指定した従業員・日付の打刻可能種別と打刻基準日を返します。</p> <p>例: すでに出勤した状態だと、休憩開始、退勤が配列で返ります。</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**date** | Option<**String**> | 従業員情報を取得したい年月日(YYYY-MM-DD)(例:2018-08-01)(デフォルト：当日) |  |

### Return type

[**crate::models::ApiV1EmployeesTimeClocksControllerPeriodAvailableTypesResponse**](ApiV1EmployeesTimeClocksController.available_types_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_welfare_pension_insurance_rule

> crate::models::ApiV1EmployeesWelfarePensionInsuranceRulesControllerPeriodShowResponse get_employee_welfare_pension_insurance_rule(company_id, year, month, employee_id)
従業員の厚生年金保険の取得

<h2 id=\"\">概要</h2> <p>指定した従業員・日付の厚生年金保険情報を返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>保険料計算方法が自動計算の場合、対応する保険料の直接指定金額は無視されnullが返されます。(例: 給与計算時の厚生年金保険料の計算方法が自動計算の場合、給与計算時の厚生年金保険料の直接指定金額はnullが返されます)</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が検索結果として返します。<br> 翌月払いの従業員の2022/01の従業員情報を取得する場合は、year=2021,month=12を指定してください。<br> | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesWelfarePensionInsuranceRulesControllerPeriodShowResponse**](ApiV1EmployeesWelfarePensionInsuranceRulesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_work_record

> crate::models::ApiV1EmployeesWorkRecordSerializer get_employee_work_record(company_id, employee_id, date)
勤怠の取得

<h2 id=\"\">概要</h2> <p>指定した従業員・日付の勤怠情報を返します。</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**date** | **String** | 従業員情報を取得したい年月日(YYYY-MM-DD)(例:2018-08-01) | [required] |

### Return type

[**crate::models::ApiV1EmployeesWorkRecordSerializer**](ApiV1EmployeesWorkRecordSerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_work_record_summary

> crate::models::ApiV1EmployeesWorkRecordSummarySerializer get_employee_work_record_summary(company_id, employee_id, year, month, work_records)
勤怠情報月次サマリの取得

<h2 id=\"\">概要</h2> <p>指定した従業員、月の勤怠情報のサマリを返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>work_recordsオプションにtrueを指定することで、明細となる日次の勤怠情報もあわせて返却します。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月 | [required] |
**work_records** | Option<**bool**> | サマリ情報に日次の勤怠情報を含める(true/false)(デフォルト: false) |  |

### Return type

[**crate::models::ApiV1EmployeesWorkRecordSummarySerializer**](ApiV1EmployeesWorkRecordSummarySerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employees

> crate::models::ApiV1EmployeesIndexSerializer get_employees(company_id, year, month, limit, offset)
従業員一覧の取得

<h2 id=\"\">概要</h2> <p>指定した対象年月に事業所に所属する従業員をリストで返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>指定した年月に退職済みユーザーは取得できません。</li>   <li>保険料計算方法が自動計算の場合、対応する保険料の直接指定金額は無視されnullが返されます。(例: 給与計算時の健康保険料の計算方法が自動計算の場合、給与計算時の健康保険料の直接指定金額はnullが返されます)</li>   <li>事業所が定額制の健康保険組合に加入している場合、保険料の直接指定金額は無視されnullが返されます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が検索結果として返します。<br> 翌月払いの従業員の2022/01の従業員情報を取得する場合は、year=2021,month=12を指定してください。<br> | [required] |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::ApiV1EmployeesIndexSerializer**](ApiV1EmployeesIndexSerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employees_special_holidays

> crate::models::GetEmployeesSpecialHolidays200Response get_employees_special_holidays(company_id, employee_id, date, start_date, end_date)
従業員の特別休暇一覧の取得

<h2 id=\"\">概要</h2> <p>指定した従業員に付与された特別休暇情報をリストで返します。</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**date** | Option<**String**> | 対象日 |  |
**start_date** | Option<**String**> | 対象開始日 |  |
**end_date** | Option<**String**> | 対象終了日 |  |

### Return type

[**crate::models::GetEmployeesSpecialHolidays200Response**](get_employees_special_holidays_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups

> crate::models::ApiV1GroupsIndexResponse get_groups(company_id)
部門一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の指定日付時点における部門情報をリストで返します。</p> <p>部門APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/groups-api-hierarchy\" target=\"_blank\">部門APIを利用した組織図の取得について</a> をご参照ください。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::ApiV1GroupsIndexResponse**](ApiV1GroupsIndexResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_positions

> crate::models::ApiV1PositionIndexResponse get_positions(company_id)
役職一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の指定日付時点における役職情報をリストで返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::ApiV1PositionIndexResponse**](ApiV1PositionIndexResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_salaries_employee_payroll_statement

> crate::models::ApiV1SalariesEmployeePayrollStatementsControllerPeriodShowResponse get_salaries_employee_payroll_statement(company_id, year, month, employee_id)
給与明細の取得

<h2 id=\"\">概要</h2> <p>指定した従業員ID、年月の給与明細を返します。</p> <p>指定した年月に支払いのある給与明細が返されます。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>複数時給を設定している場合はpaymentsに内訳が返されます。</li>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>給与計算中の場合は、各パラメータはnullおよび空配列が返ります。</li> </ul>  <h2 id=\"_2\">examples</h2> <pre>   <code>   {   \"employee_payroll_statement\": {     \"id\": 1,     \"company_id\": 1,     \"employee_id\": 1,     \"employee_name\": \"給与 太郎\",     \"employee_display_name\": \"給与 太郎\",     \"employee_num\": \"001\",     \"pay_date\": \"2018-02-25\",     \"start_date\": \"2018-02-01\",     \"closing_date\": \"2018-02-28\",     \"variable_pay_start_date\": \"2018-01-01\",     \"variable_pay_closing_date\": \"2018-01-31\",     \"fixed\": true,     \"calc_status\": \"calculated\",     \"calculated_at\": \"2018-09-27T05:06:45.315Z\",     \"pay_calc_type\": \"monthly\",     \"board_member_remuneration_amount\": \"400000.0\",     \"basic_pay_amount\": \"300000.0\",     \"work_days\": \"21.0\",     \"normal_work_time\": \"8.0\",     \"normal_work_days\": \"21.0\",     \"work_mins_by_paid_holiday\": \"480.0\",     \"num_paid_holidays\": \"1.0\",     \"is_board_member\": true,     \"total_attendance_deduction_amount\": \"0.0\",     \"total_allowance_amount\": \"0.0\",     \"total_deduction_amount\": \"23830.0\",     \"net_payment_amount\": \"277170.0\",     \"gross_payment_amount\": \"301000.0\",     \"total_worked_days_count\": \"21.0\",     \"total_taxable_payment_amount\": \"301000.0\",     \"total_expense_amount\": \"0.0\",     \"total_transfer_amount\": \"277170.0\",     \"total_annual_payment_amount\": \"600000.0\",     \"payments\": [{ \"name\": \"基本給\", \"amount\": \"300000.0\"},{ \"name\": \"残業代\", \"amount\": \"1000.0\"}],     \"deductions\": [{\"name\": \"所得税\", \"amount\": \"7000.0\"}, {\"name\": \"健康保険料\", \"amount\": \"16830.0\"}],     \"attendances\": [{\"name\": \"遅刻・早退\", \"time\": \"0.0\", \"amount\": \"0.0\"}],     \"overtime_pays\": [{ \"name\": \"時間外労働\", \"time\": \"60.0\", \"amount\": \"1000.0\", \"code\": null }, { \"name\": \"カスタム固定残業代\", \"time\": null, \"amount\": \"10000.0\", \"code\": \"a001\" }],     \"remark\": \"備考\"     }   }   </code> </pre>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1SalariesEmployeePayrollStatementsControllerPeriodShowResponse**](ApiV1SalariesEmployeePayrollStatementsController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_salaries_employee_payroll_statements

> crate::models::ApiV1SalariesEmployeePayrollStatementsControllerPeriodIndexResponse get_salaries_employee_payroll_statements(company_id, year, month, limit, offset)
給与明細一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所に所属する従業員の給与明細をリストで返します。</p> <p>指定した年月に支払いのある給与明細が返されます。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>複数時給を設定している場合はpaymentsに内訳が返されます。</li>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>給与計算中の場合は、各パラメータはnullおよび空配列が返ります。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 従業員情報を取得したい年 | [required] |
**month** | **i32** | 従業員情報を取得したい月 | [required] |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::ApiV1SalariesEmployeePayrollStatementsControllerPeriodIndexResponse**](ApiV1SalariesEmployeePayrollStatementsController.index_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_me

> crate::models::ApiV1UsersMeSerializer get_users_me()
ログインユーザーの取得

<h2 id=\"\">概要</h2> <p>このリクエストの認可セッションにおけるログインユーザーの情報を返します。</p> <p>freee人事労務では一人のログインユーザーを複数の事業所に関連付けられるため、このユーザーと関連のあるすべての事業所の情報をリストで返します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>他のAPIのパラメータとしてcompany_idが求められる場合は、このAPIで取得したcompany_idを使用します。</li>   <li>給与計算対象外の従業員のemployee_idとdisplay_nameは取得できません。</li> </ul>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiV1UsersMeSerializer**](ApiV1UsersMeSerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_yearend_adjustment_employee

> crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodShowResponse get_yearend_adjustment_employee(company_id, year, employee_id)
年末調整の取得

指定した年、従業員IDの年末調整の詳細内容を返します。<br> 年末調整対象外の従業員は、本人情報、給与・賞与、前職情報のみが取得できます。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 年末調整を取得したい年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodShowResponse**](ApiV1EmployeeYearendAdjustmentController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_yearend_adjustment_employees

> crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodIndexResponse get_yearend_adjustment_employees(company_id, year, limit, offset)
年末調整対象一覧の取得

 指定した年の年末調整対象のリスト返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 年末調整対象を取得したい年 | [required] |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodIndexResponse**](ApiV1EmployeeYearendAdjustmentController.index_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_yearend_adjustment_housing_loan

> crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateHousingLoanResponse post_yearend_adjustment_housing_loan(year, employee_id, body)
年末調整従業員住宅ローンの作成

<h2 id=\"\">概要</h2> <p>指定した従業員の住宅ローンを作成します。</p> <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | 作成対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateHousingLoanBody**](ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateHousingLoanBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateHousingLoanResponse**](ApiV1EmployeeYearendAdjustmentController.update_housing_loan_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_yearend_adjustment_insurances

> crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateInsuranceResponse post_yearend_adjustment_insurances(year, employee_id, body)
年末調整従業員保険料情報の作成

<h2 id=\"\">概要</h2> <p>指定した従業員の保険料情報を作成します。</p> <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | 作成対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateInsuranceBody**](ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateInsuranceBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateInsuranceResponse**](ApiV1EmployeeYearendAdjustmentController.update_insurance_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_yearend_adjustment_dependents

> crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateDependentsResponse put_yearend_adjustment_dependents(year, employee_id, api_v1_employee_yearend_adjustment_controller_period_update_dependents_body)
年末調整家族情報の更新

<h2 id=\"\">概要</h2> <p>指定した年末調整の家族情報を更新します。</p> <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li>   <li>idがない場合は新規作成、destroyがtrueの場合は削除になります。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | 更新対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**api_v1_employee_yearend_adjustment_controller_period_update_dependents_body** | Option<[**ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateDependentsBody**](ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateDependentsBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateDependentsResponse**](ApiV1EmployeeYearendAdjustmentController.update_dependents_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_yearend_adjustment_employee

> crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateEmployeeResponse put_yearend_adjustment_employee(year, employee_id, api_v1_employee_yearend_adjustment_controller_period_update_employee_body)
年末調整従業員情報の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の姓名・住所などを更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | 更新対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**api_v1_employee_yearend_adjustment_controller_period_update_employee_body** | Option<[**ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateEmployeeBody**](ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateEmployeeBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateEmployeeResponse**](ApiV1EmployeeYearendAdjustmentController.update_employee_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_yearend_adjustment_housing_loan

> crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateHousingLoanResponse put_yearend_adjustment_housing_loan(year, employee_id, id, api_v1_employee_yearend_adjustment_controller_period_update_housing_loan_body)
年末調整従業員住宅ローンの更新

<h2 id=\"\">概要</h2> <p>指定した従業員の住宅ローンを更新します。</p> <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | 更新対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**id** | **i32** | 住宅ローンID | [required] |
**api_v1_employee_yearend_adjustment_controller_period_update_housing_loan_body** | Option<[**ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateHousingLoanBody**](ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateHousingLoanBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateHousingLoanResponse**](ApiV1EmployeeYearendAdjustmentController.update_housing_loan_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_yearend_adjustment_housing_loan_deduction

> crate::models::PutYearendAdjustmentHousingLoanDeduction200Response put_yearend_adjustment_housing_loan_deduction(year, employee_id, put_yearend_adjustment_housing_loan_deduction_request)
年末調整従業員住宅ローン控除額の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の住宅ローン控除額を更新します。</p> <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | 更新対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**put_yearend_adjustment_housing_loan_deduction_request** | Option<[**PutYearendAdjustmentHousingLoanDeductionRequest**](PutYearendAdjustmentHousingLoanDeductionRequest.md)> |  |  |

### Return type

[**crate::models::PutYearendAdjustmentHousingLoanDeduction200Response**](put_yearend_adjustment_housing_loan_deduction_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_yearend_adjustment_insurances

> crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateInsuranceResponse put_yearend_adjustment_insurances(year, employee_id, id, api_v1_employee_yearend_adjustment_controller_period_update_insurance_body)
年末調整従業員保険料情報の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の保険料情報を更新します。</p> <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | 更新対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**id** | **i32** | 保険料ID | [required] |
**api_v1_employee_yearend_adjustment_controller_period_update_insurance_body** | Option<[**ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateInsuranceBody**](ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateInsuranceBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdateInsuranceResponse**](ApiV1EmployeeYearendAdjustmentController.update_insurance_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_yearend_adjustment_payroll_and_bonus

> crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdatePayrollAndBonusResponse put_yearend_adjustment_payroll_and_bonus(year, employee_id, api_v1_employee_yearend_adjustment_controller_period_update_payroll_and_bonus_body)
年末調整従業員給与・賞与の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の給与・賞与を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | 更新対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**api_v1_employee_yearend_adjustment_controller_period_update_payroll_and_bonus_body** | Option<[**ApiV1EmployeeYearendAdjustmentControllerPeriodUpdatePayrollAndBonusBody**](ApiV1EmployeeYearendAdjustmentControllerPeriodUpdatePayrollAndBonusBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdatePayrollAndBonusResponse**](ApiV1EmployeeYearendAdjustmentController.update_payroll_and_bonus_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_yearend_adjustment_previous_job

> crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdatePreviousJobResponse put_yearend_adjustment_previous_job(year, employee_id, api_v1_employee_yearend_adjustment_controller_period_update_previous_job_body)
年末調整従業員前職情報の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の前職情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、年末調整が確定済みの従業員には非対応です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | 更新対象年 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**api_v1_employee_yearend_adjustment_controller_period_update_previous_job_body** | Option<[**ApiV1EmployeeYearendAdjustmentControllerPeriodUpdatePreviousJobBody**](ApiV1EmployeeYearendAdjustmentControllerPeriodUpdatePreviousJobBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeeYearendAdjustmentControllerPeriodUpdatePreviousJobResponse**](ApiV1EmployeeYearendAdjustmentController.update_previous_job_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_approval_requests_monthly_attendance

> crate::models::ApiV1MonthlyAttendanceResponse update_approval_requests_monthly_attendance(id, api_v1_monthly_attendance_update_request)
月次勤怠締め申請の更新

<h2 id=\"\">概要</h2> <p>指定した事業所の月次勤怠締め申請情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 月次勤怠締め申請ID | [required] |
**api_v1_monthly_attendance_update_request** | Option<[**ApiV1MonthlyAttendanceUpdateRequest**](ApiV1MonthlyAttendanceUpdateRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1MonthlyAttendanceResponse**](ApiV1MonthlyAttendanceResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_approval_requests_overtime_work

> crate::models::ApiV1OvertimeWorkResponse update_approval_requests_overtime_work(id, api_v1_overtime_work_request)
残業申請の更新

<h2 id=\"\">概要</h2> <p>指定した事業所の残業申請情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 残業申請ID | [required] |
**api_v1_overtime_work_request** | Option<[**ApiV1OvertimeWorkRequest**](ApiV1OvertimeWorkRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1OvertimeWorkResponse**](ApiV1OvertimeWorkResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_approval_requests_paid_holiday

> crate::models::ApiV1PaidHolidayResponse update_approval_requests_paid_holiday(id, api_v1_paid_holiday_request)
有給申請の更新

<h2 id=\"\">概要</h2> <p>指定した事業所の有給申請情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 有給申請ID | [required] |
**api_v1_paid_holiday_request** | Option<[**ApiV1PaidHolidayRequest**](ApiV1PaidHolidayRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1PaidHolidayResponse**](ApiV1PaidHolidayResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_approval_requests_special_holiday

> crate::models::ApiV1specialHolidayResponse update_approval_requests_special_holiday(id, api_v1special_holiday_request)
特別休暇申請の更新

<h2 id=\"\">概要</h2> <p>指定した事業所の特別休暇申請情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 特別休暇申請ID | [required] |
**api_v1special_holiday_request** | Option<[**ApiV1specialHolidayRequest**](ApiV1specialHolidayRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1specialHolidayResponse**](ApiV1specialHolidayResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_approval_requests_work_time

> serde_json::Value update_approval_requests_work_time(id, body)
勤務時間修正申請の更新

<h2 id=\"\">概要</h2> <p>指定した事業所の勤務時間修正申請情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</li>   <ul>     <li>役職指定（申請者の所属部門）</li>     <li>役職指定（申請時に部門指定）</li>     <li>部門および役職指定</li>   </ul>   <li>申請者と承認者が同一ユーザーの場合、feedback(差戻し)をするとレスポンスは以下のようになります。</li>   <ul>     <li>status: draft</li>     <li>approval_flow_logs.action: cancel</li>   </ul> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 勤務時間修正申請ID | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee

> crate::models::ApiV1EmployeesControllerPeriodUpdateResponse update_employee(id, body)
従業員の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesControllerPeriodUpdateBody**](ApiV1EmployeesControllerPeriodUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesControllerPeriodUpdateResponse**](ApiV1EmployeesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_bank_account_rule

> crate::models::ApiV1EmployeesBankAccountRulesControllerPeriodUpdateResponse update_employee_bank_account_rule(employee_id, body)
従業員の銀行口座の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の銀行口座1の情報を更新します。</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesBankAccountRulesControllerPeriodUpdateBody**](ApiV1EmployeesBankAccountRulesControllerPeriodUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesBankAccountRulesControllerPeriodUpdateResponse**](ApiV1EmployeesBankAccountRulesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_basic_pay_rule

> crate::models::ApiV1EmployeesBasicPayRulesControllerPeriodUpdateResponse update_employee_basic_pay_rule(employee_id, body)
従業員の基本給の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の基本給情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesBasicPayRulesControllerPeriodUpdateBody**](ApiV1EmployeesBasicPayRulesControllerPeriodUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesBasicPayRulesControllerPeriodUpdateResponse**](ApiV1EmployeesBasicPayRulesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_health_insurance_rule

> crate::models::ApiV1EmployeesHealthInsuranceRulesControllerPeriodUpdateResponse update_employee_health_insurance_rule(employee_id, body)
従業員の健康保険の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の健康保険情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>保険料計算方法が自動計算の場合、対応する保険料の直接指定金額は無視されnullが返されます。(例: 給与計算時の健康保険料の計算方法が自動計算の場合、給与計算時の健康保険料の直接指定金額はnullが返されます)</li>   <li>事業所が定額制の健康保険組合に加入している場合、保険料の直接指定金額は無視されnullが返されます。</li>   <li>事業所が定額制の健康保険組合に加入している場合、保険料の計算方法と保険料の更新はできません。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesHealthInsuranceRulesControllerPeriodUpdateBody**](ApiV1EmployeesHealthInsuranceRulesControllerPeriodUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesHealthInsuranceRulesControllerPeriodUpdateResponse**](ApiV1EmployeesHealthInsuranceRulesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_profile_rule

> crate::models::ApiV1EmployeesProfileRulesControllerPeriodUpdateResponse update_employee_profile_rule(employee_id, body)
従業員の姓名・住所などの更新

<h2 id=\"\">概要</h2> <p>指定した従業員の姓名・住所などを更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>本APIは、給与計算対象外の従業員には非対応です。employee_idに給与計算対象外の従業員IDを指定した場合、本APIを利用できません。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesProfileRulesControllerPeriodUpdateBody**](ApiV1EmployeesProfileRulesControllerPeriodUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesProfileRulesControllerPeriodUpdateResponse**](ApiV1EmployeesProfileRulesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_welfare_pension_insurance_rule

> crate::models::ApiV1EmployeesWelfarePensionInsuranceRulesControllerPeriodUpdateResponse update_employee_welfare_pension_insurance_rule(employee_id, body)
従業員の厚生年金保険の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の厚生年金保険情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>保険料計算方法が自動計算の場合、対応する保険料の直接指定金額は無視されnullが返されます。(例: 給与計算時の厚生年金保険料の計算方法が自動計算の場合、給与計算時の厚生年金保険料の直接指定金額はnullが返されます)</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesWelfarePensionInsuranceRulesControllerPeriodUpdateBody**](ApiV1EmployeesWelfarePensionInsuranceRulesControllerPeriodUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesWelfarePensionInsuranceRulesControllerPeriodUpdateResponse**](ApiV1EmployeesWelfarePensionInsuranceRulesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_work_record

> crate::models::ApiV1EmployeesWorkRecordSerializer update_employee_work_record(employee_id, date, body)
勤怠の更新

<h2 id=\"\">概要</h2> <p>指定した従業員の勤怠情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>振替出勤・振替休日・代休出勤・代休・特別休暇の登録はAPIでは行うことができません。</li> </ul>  <h2 id=\"_2\">examples</h2> <ul>   <li>出勤日について出退勤時刻および休憩時間を更新する場合は以下のようなパラメータをリクエストします。     <pre>       <code>       {         \"company_id\": 1,         \"break_records\": [           {             \"clock_in_at\": \"2017-05-25 12:00:00\",             \"clock_out_at\": \"2017-05-25 13:00:00\"           }         ],         \"clock_in_at\": \"2017-05-25 09:10:00\",         \"clock_out_at\": \"2017-05-25 18:20:00\"       }       </code>     </pre>   </li>    <li>勤務パターンや既定の所定労働時間を変更する場合は use_default_work_pattern に false を指定するとともに、各設定を上書きするパラメータをリクエストします。     <pre>       <code>       {         \"company_id\": 1,         \"break_records\": [           {             \"clock_in_at\": \"2017-05-25 12:00:00\",             \"clock_out_at\": \"2017-05-25 13:00:00\"           }         ],         \"clock_in_at\": \"2017-05-25 09:10:00\",         \"clock_out_at\": \"2017-05-25 18:20:00\",         \"day_pattern\": \"normal_day\",         \"normal_work_clock_in_at\": \"2017-05-25 11:00:00\",         \"normal_work_clock_out_at\": \"2017-12-20 20:00:00\",         \"normal_work_mins\": 0,         \"use_default_work_pattern\": false       }       </code>     </pre>   </li>    <li>有給取得時の連携について半休の場合は通常勤務のように勤務開始・終了時間を指定しつつ、加えて以下の２つの要素を指定することで API での勤怠をつけることができます。     <ul>       <li>paid_holiday (半休の場合は 0.5 とします)</li>       <li>normal_work_mins_by_paid_holiday (半休により計上される所定労働時間を分で指定します)</li>     </ul>     <pre>       <code>       {         \"company_id\": 1,         \"break_records\": [           {             \"clock_in_at\": \"2017-05-25 12:00:00\",             \"clock_out_at\": \"2017-05-25 13:00:00\"           }         ],         \"clock_in_at\": \"2017-05-25 09:10:00\",         \"clock_out_at\": \"2017-05-25 18:20:00\",         \"paid_holiday\": 0.5,         \"normal_work_mins_by_paid_holiday\": 240       }       </code>     </pre>   </li>    <li>欠勤を付ける場合は company_idとis_absence 以外のパラメータは必要ありません。     <pre>       <code>       {         \"company_id\": 1,         \"is_absence\": true       }       </code>     </pre>   </li>  </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**date** | **String** | 更新対象年月日(YYYY-MM-DD)(例:2018-08-01) | [required] |
**body** | Option<[**ApiV1EmployeesWorkRecordsControllerPeriodUpdateBody**](ApiV1EmployeesWorkRecordsControllerPeriodUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesWorkRecordSerializer**](ApiV1EmployeesWorkRecordSerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_work_record_summary

> crate::models::ApiV1EmployeesWorkRecordSummarySerializer update_employee_work_record_summary(employee_id, year, month, api_v1_employees_work_record_summary_controller_period_update_body)
勤怠情報月次サマリの更新

<h2 id=\"\">概要</h2> <p>指定した従業員、月の勤怠情報のサマリを更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li>   <li>日毎の勤怠の更新はこのAPIではできません。日毎の勤怠の操作には勤怠APIを使用して下さい。</li>   <li>勤怠データが存在しない場合は新規作成、既に存在する場合は上書き更新されます。</li>   <li>値が設定された項目のみ更新されます。値が設定されなかった場合は自動的に0が設定されます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**year** | **i32** | 更新対象年 | [required] |
**month** | **i32** | 更新対象月 | [required] |
**api_v1_employees_work_record_summary_controller_period_update_body** | Option<[**ApiV1EmployeesWorkRecordSummaryControllerPeriodUpdateBody**](ApiV1EmployeesWorkRecordSummaryControllerPeriodUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesWorkRecordSummarySerializer**](ApiV1EmployeesWorkRecordSummarySerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group

> crate::models::ApiV1GroupResponse update_group(id, api_v1_group_update_request)
部門の更新

<h2 id=\"\">概要</h2> <p>指定した事業所の部門の情報を更新します。</p> <p>部門APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/groups-api-hierarchy\" target=\"_blank\">部門APIを利用した組織図の取得について</a> をご参照ください。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 部門ID | [required] |
**api_v1_group_update_request** | Option<[**ApiV1GroupUpdateRequest**](ApiV1GroupUpdateRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1GroupResponse**](ApiV1GroupResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_position

> crate::models::ApiV1PositionResponse update_position(id, api_v1_position_request)
役職の更新

<h2 id=\"\">概要</h2> <p>指定した事業所の役職の情報を更新します。</p>  <h2 id=\"_1\">注意点</h2> <ul>   <li>管理者権限を持ったユーザーのみ実行可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 役職ID | [required] |
**api_v1_position_request** | Option<[**ApiV1PositionRequest**](ApiV1PositionRequest.md)> |  |  |

### Return type

[**crate::models::ApiV1PositionResponse**](ApiV1PositionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

