# \DefaultApi

All URIs are relative to *https://api.freee.co.jp/hr*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_update_employee_dependent_rules**](DefaultApi.md#bulk_update_employee_dependent_rules) | **PUT** /api/v1/employees/{employee_id}/dependent_rules/bulk_update | 更新 
[**create_employee**](DefaultApi.md#create_employee) | **POST** /api/v1/employees | 作成 
[**create_employee_time_clock**](DefaultApi.md#create_employee_time_clock) | **POST** /api/v1/employees/{employee_id}/time_clocks | 打刻情報の登録 
[**create_group**](DefaultApi.md#create_group) | **POST** /api/v1/groups | 部門情報の作成 
[**create_position**](DefaultApi.md#create_position) | **POST** /api/v1/positions | 役職情報の作成 
[**destroy_employee**](DefaultApi.md#destroy_employee) | **DELETE** /api/v1/employees/{id} | 削除 
[**destroy_employee_work_record**](DefaultApi.md#destroy_employee_work_record) | **DELETE** /api/v1/employees/{employee_id}/work_records/{date} | 削除 
[**destroy_group**](DefaultApi.md#destroy_group) | **DELETE** /api/v1/groups/{id} | 部門情報の削除 
[**destroy_position**](DefaultApi.md#destroy_position) | **DELETE** /api/v1/positions/{id} | 役職情報の削除 
[**get_bonuses_employee_payroll_statement**](DefaultApi.md#get_bonuses_employee_payroll_statement) | **GET** /api/v1/bonuses/employee_payroll_statements/{employee_id} | 取得 
[**get_bonuses_employee_payroll_statements**](DefaultApi.md#get_bonuses_employee_payroll_statements) | **GET** /api/v1/bonuses/employee_payroll_statements | 一覧の取得 
[**get_company_employees**](DefaultApi.md#get_company_employees) | **GET** /api/v1/companies/{company_id}/employees | 一覧の取得 
[**get_employee**](DefaultApi.md#get_employee) | **GET** /api/v1/employees/{id} | 取得 
[**get_employee_bank_account_rule**](DefaultApi.md#get_employee_bank_account_rule) | **GET** /api/v1/employees/{employee_id}/bank_account_rule | 取得 
[**get_employee_basic_pay_rule**](DefaultApi.md#get_employee_basic_pay_rule) | **GET** /api/v1/employees/{employee_id}/basic_pay_rule | 取得 
[**get_employee_dependent_rules**](DefaultApi.md#get_employee_dependent_rules) | **GET** /api/v1/employees/{employee_id}/dependent_rules | 取得 
[**get_employee_group_memberships**](DefaultApi.md#get_employee_group_memberships) | **GET** /api/v1/employee_group_memberships | 一覧の取得 
[**get_employee_health_insurance_rule**](DefaultApi.md#get_employee_health_insurance_rule) | **GET** /api/v1/employees/{employee_id}/health_insurance_rule | 取得 
[**get_employee_profile_rule**](DefaultApi.md#get_employee_profile_rule) | **GET** /api/v1/employees/{employee_id}/profile_rule | 取得 
[**get_employee_time_clock**](DefaultApi.md#get_employee_time_clock) | **GET** /api/v1/employees/{employee_id}/time_clocks/{id} | 打刻情報の詳細取得 
[**get_employee_time_clocks**](DefaultApi.md#get_employee_time_clocks) | **GET** /api/v1/employees/{employee_id}/time_clocks | 打刻情報の一覧取得 
[**get_employee_time_clocks_available_types**](DefaultApi.md#get_employee_time_clocks_available_types) | **GET** /api/v1/employees/{employee_id}/time_clocks/available_types | 打刻可能種別の取得 
[**get_employee_welfare_pension_insurance_rule**](DefaultApi.md#get_employee_welfare_pension_insurance_rule) | **GET** /api/v1/employees/{employee_id}/welfare_pension_insurance_rule | 取得 
[**get_employee_work_record**](DefaultApi.md#get_employee_work_record) | **GET** /api/v1/employees/{employee_id}/work_records/{date} | 取得 
[**get_employee_work_record_summary**](DefaultApi.md#get_employee_work_record_summary) | **GET** /api/v1/employees/{employee_id}/work_record_summaries/{year}/{month} | 勤怠情報月次サマリの取得 
[**get_employees**](DefaultApi.md#get_employees) | **GET** /api/v1/employees | 一覧の取得 
[**get_groups**](DefaultApi.md#get_groups) | **GET** /api/v1/groups | 部門情報の一覧取得 
[**get_positions**](DefaultApi.md#get_positions) | **GET** /api/v1/positions | 役職情報の一覧取得 
[**get_salaries_employee_payroll_statement**](DefaultApi.md#get_salaries_employee_payroll_statement) | **GET** /api/v1/salaries/employee_payroll_statements/{employee_id} | 取得 
[**get_salaries_employee_payroll_statements**](DefaultApi.md#get_salaries_employee_payroll_statements) | **GET** /api/v1/salaries/employee_payroll_statements | 一覧の取得 
[**get_users_me**](DefaultApi.md#get_users_me) | **GET** /api/v1/users/me | 取得 
[**update_employee**](DefaultApi.md#update_employee) | **PUT** /api/v1/employees/{id} | 更新 
[**update_employee_bank_account_rule**](DefaultApi.md#update_employee_bank_account_rule) | **PUT** /api/v1/employees/{employee_id}/bank_account_rule | 更新 
[**update_employee_basic_pay_rule**](DefaultApi.md#update_employee_basic_pay_rule) | **PUT** /api/v1/employees/{employee_id}/basic_pay_rule | 更新 
[**update_employee_health_insurance_rule**](DefaultApi.md#update_employee_health_insurance_rule) | **PUT** /api/v1/employees/{employee_id}/health_insurance_rule | 更新 
[**update_employee_profile_rule**](DefaultApi.md#update_employee_profile_rule) | **PUT** /api/v1/employees/{employee_id}/profile_rule | 更新 
[**update_employee_welfare_pension_insurance_rule**](DefaultApi.md#update_employee_welfare_pension_insurance_rule) | **PUT** /api/v1/employees/{employee_id}/welfare_pension_insurance_rule | 更新 
[**update_employee_work_record**](DefaultApi.md#update_employee_work_record) | **PUT** /api/v1/employees/{employee_id}/work_records/{date} | 更新 
[**update_employee_work_record_summary**](DefaultApi.md#update_employee_work_record_summary) | **PUT** /api/v1/employees/{employee_id}/work_record_summaries/{year}/{month} | 勤怠情報月次サマリの更新 
[**update_group**](DefaultApi.md#update_group) | **PUT** /api/v1/groups/{id} | 部門情報の更新 
[**update_position**](DefaultApi.md#update_position) | **PUT** /api/v1/positions/{id} | 役職情報の更新 



## bulk_update_employee_dependent_rules

> crate::models::ApiV1EmployeesDependentRulesControllerBulkUpdateResponse bulk_update_employee_dependent_rules(employee_id, body)
更新 

指定した従業員の扶養親族情報を更新します。 idがない場合は新規作成、destroyがtrueの場合は削除になります。  residence_type=live_in: 同居の場合、以下のパラメータに指定した値はWebに反映されません。 - zipcode1 - zipcode2 - prefecture_code - address - address_kana - annual_remittance_amount  residence_type=non_resident: 別居(国外)の場合、以下のパラメータに指定した値はWebに反映されません。 - prefecture_code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesDependentRulesControllerBulkUpdateBody**](ApiV1EmployeesDependentRulesControllerBulkUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesDependentRulesControllerBulkUpdateResponse**](ApiV1EmployeesDependentRulesController.bulk_update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_employee

> crate::models::ApiV1EmployeesControllerCreateResponse create_employee(body)
作成 

 従業員を新規作成します。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV1EmployeesControllerCreateBody**](ApiV1EmployeesControllerCreateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesControllerCreateResponse**](ApiV1EmployeesController.create_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_employee_time_clock

> crate::models::ApiV1EmployeesTimeClocksControllerCreateResponse create_employee_time_clock(employee_id, body)
打刻情報の登録 

 指定した従業員の打刻情報を登録します。 休憩開始の連続や退勤のみなど、整合性の取れていない打刻は登録できません。 退勤の打刻は、すでに登録されている退勤打刻よりも後の時刻であれば上書き登録することができます。 打刻可能種別の取得APIを呼ぶことで、その従業員がその時点で登録可能な打刻種別が取得できます。 打刻が日をまたぐ場合は、base_date(打刻日)に前日の日付を指定してください。 datetime(打刻日時)を指定できるのは管理者か事務担当者の権限を持ったユーザのみです。  本APIはベーシックプラン以上で利用可能なAPIです。対象外のプランでは403エラーを返却します。  ## 操作可能な打刻種別 - clock_in：出勤 - break_begin：休憩開始 - break_end：休憩終了 - clock_out：退勤

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesTimeClocksControllerCreateBody**](ApiV1EmployeesTimeClocksControllerCreateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesTimeClocksControllerCreateResponse**](ApiV1EmployeesTimeClocksController.create_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> crate::models::ApiV1GroupResponse create_group(api_v1_group_create_request)
部門情報の作成 

 指定した事業所の部門を新規作成します。 部門APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/groups-api-hierarchy\" target=\"_blank\">部門APIを利用した組織図の取得について</a> をご参照ください。 - 管理者権限を持ったユーザのみ実行可能です。

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
役職情報の作成 

 指定した事業所の役職を新規作成します。 - 管理者権限を持ったユーザのみ実行可能です。

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


## destroy_employee

> destroy_employee(id, company_id)
削除 

 指定したIDの従業員を削除します。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Scope response to id | [required] |
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
削除 

 指定した従業員の勤怠情報を削除します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**date** | **String** | 対象日(YYYY-MM-DD)(例:2018-08-01) | [required] |
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
部門情報の削除 

 指定した事業所の部門の情報を削除します。 部門APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/groups-api-hierarchy\" target=\"_blank\">部門APIを利用した組織図の取得について</a> をご参照ください。 - 管理者権限を持ったユーザのみ実行可能です。

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
役職情報の削除 

 指定した事業所の役職の情報を削除します。 - 管理者権限を持ったユーザのみ実行可能です。 - 従業員に役職が適用されている場合、従業員の役職情報も削除されます。

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


## get_bonuses_employee_payroll_statement

> crate::models::ApiV1BonusesEmployeePayrollStatementsControllerShowResponse get_bonuses_employee_payroll_statement(company_id, year, month, employee_id)
取得 

 指定した従業員ID、年月の賞与明細を返します。 指定した年月に支払いのある賞与明細が返されます。 - 管理者権限を持ったユーザのみ実行可能です。  # examples ``` {  \"employee_payroll_statement\": {    \"id\": 1,    \"company_id\": 1,    \"employee_id\": 1,    \"employee_name\": \"給与 太郎\",    \"employee_display_name\": \"給与 太郎\",    \"employee_num\": \"001\",    \"closing_date\": \"2018-03-31\",    \"pay_date\": \"2018-03-31\",    \"fixed\": true,    \"calc_status\": \"calculated\",    \"calculated_at\": \"2018-09-27T05:06:45.315Z\",    \"bonus_amount\": \"300000.0\",    \"total_allowance_amount\": \"0.0\",    \"total_deduction_amount\": \"23830.0\",    \"net_payment_amount\": \"268000.0\",    \"gross_payment_amount\": \"330000.0\",    \"total_taxable_payment_amount\": \"330000.0\",    \"allowances\": [{\"name\": \"地域手当\", \"amount\": \"30000.0\"}],    \"deductions\": [{\"name\": \"所得税\", \"amount\": \"46000.0\"}, {\"name\": \"健康保険料\", \"amount\": \"16000.0\"}],    \"remark\": \"備考\"  } } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** |  | [required] |
**year** | **i32** |  | [required] |
**month** | **i32** |  | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1BonusesEmployeePayrollStatementsControllerShowResponse**](ApiV1BonusesEmployeePayrollStatementsController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bonuses_employee_payroll_statements

> crate::models::ApiV1BonusesEmployeePayrollStatementsIndexSerializer get_bonuses_employee_payroll_statements(company_id, year, month, limit, offset)
一覧の取得 

 指定した事業所に所属する従業員の賞与明細をリストで返します。 指定した年月に支払いのある賞与明細が返されます。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** |  | [required] |
**year** | **i32** |  | [required] |
**month** | **i32** |  | [required] |
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
一覧の取得 

 指定した事業所に所属する従業員をリストで返します。 - 管理者権限を持ったユーザのみ実行可能です。 - 退職ユーザも含めて取得可能です。

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

> crate::models::ApiV1EmployeesControllerShowResponse get_employee(company_id, year, month, id)
取得 

 指定したIDの従業員を返します。 - 管理者権限を持ったユーザのみ実行可能です。 - 指定した対象年月に退職済みユーザは取得できません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 対象年月（年） | [required] |
**month** | **i32** | 対象年月（月） | [required] |
**id** | **i32** | Scope response to id | [required] |

### Return type

[**crate::models::ApiV1EmployeesControllerShowResponse**](ApiV1EmployeesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_bank_account_rule

> crate::models::ApiV1EmployeesBankAccountRulesControllerShowResponse get_employee_bank_account_rule(company_id, year, month, employee_id)
取得 

 指定した従業員・日付の銀行口座情報を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 対象年 | [required] |
**month** | **i32** | 対象月 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesBankAccountRulesControllerShowResponse**](ApiV1EmployeesBankAccountRulesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_basic_pay_rule

> crate::models::ApiV1EmployeesBasicPayRulesControllerShowResponse get_employee_basic_pay_rule(company_id, year, month, employee_id)
取得 

 指定した従業員・日付の基本給情報を返します。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 対象年 | [required] |
**month** | **i32** | 対象月 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesBasicPayRulesControllerShowResponse**](ApiV1EmployeesBasicPayRulesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_dependent_rules

> crate::models::ApiV1EmployeesDependentRulesControllerIndexResponse get_employee_dependent_rules(company_id, year, month, employee_id)
取得 

 指定した従業員・日付の扶養親族情報を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 対象年 | [required] |
**month** | **i32** | 対象月 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesDependentRulesControllerIndexResponse**](ApiV1EmployeesDependentRulesController.index_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_group_memberships

> crate::models::ApiV1EmployeeGroupMembershipsIndexSerializer get_employee_group_memberships(company_id, base_date, with_no_payroll_calculation, limit, offset)
一覧の取得 

 指定した事業所の指定日付時点における所属情報をリストで返します。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** |  | [required] |
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

> crate::models::ApiV1EmployeesHealthInsuranceRulesControllerShowResponse get_employee_health_insurance_rule(company_id, year, month, employee_id)
取得 

 指定した従業員・日付の健康保険情報を返します。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 対象年 | [required] |
**month** | **i32** | 対象月 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesHealthInsuranceRulesControllerShowResponse**](ApiV1EmployeesHealthInsuranceRulesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_profile_rule

> crate::models::ApiV1EmployeesProfileRulesControllerShowResponse get_employee_profile_rule(company_id, year, month, employee_id)
取得 

 指定した従業員・日付の姓名などの情報を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 対象年 | [required] |
**month** | **i32** | 対象月<br> 締日支払日設定が翌月払いの従業員情報の場合は、 指定したmonth + 1の値が検索結果として戻ります。 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesProfileRulesControllerShowResponse**](ApiV1EmployeesProfileRulesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_time_clock

> crate::models::ApiV1EmployeesTimeClocksControllerShowResponse get_employee_time_clock(company_id, employee_id, id)
打刻情報の詳細取得 

 指定した従業員・指定した打刻の詳細情報を返します。 打刻情報の一覧取得APIにて取得した打刻IDを利用することができます。  本APIはベーシックプラン以上で利用可能なAPIです。対象外のプランでは403エラーを返却します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**id** | **i32** | 打刻ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesTimeClocksControllerShowResponse**](ApiV1EmployeesTimeClocksController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_time_clocks

> Vec<crate::models::ApiV1EmployeesTimeClockSerializer> get_employee_time_clocks(company_id, employee_id, from_date, to_date, limit, offset)
打刻情報の一覧取得 

 指定した従業員・期間の打刻情報を返します。 デフォルトでは従業員の当月の打刻開始日から当日までの値が返ります。  本APIはベーシックプラン以上で利用可能なAPIです。対象外のプランでは403エラーを返却します。

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

> crate::models::ApiV1EmployeesTimeClocksControllerAvailableTypesResponse get_employee_time_clocks_available_types(company_id, employee_id, date)
打刻可能種別の取得 

 指定した従業員・日付の打刻可能種別と打刻基準日を返します。 例: すでに出勤した状態だと、休憩開始、退勤が配列で返ります。  本APIはベーシックプラン以上で利用可能なAPIです。対象外のプランでは403エラーを返却します。  ## 返却される打刻種別 - clock_in：出勤 - break_begin：休憩開始 - break_end：休憩終了 - clock_out：退勤 ## 返却される日付 - base_date：打刻基準日

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**date** | Option<**String**> | 対象日(YYYY-MM-DD)(例:2018-08-01)(デフォルト：当日) |  |

### Return type

[**crate::models::ApiV1EmployeesTimeClocksControllerAvailableTypesResponse**](ApiV1EmployeesTimeClocksController.available_types_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_welfare_pension_insurance_rule

> crate::models::ApiV1EmployeesWelfarePensionInsuranceRulesControllerShowResponse get_employee_welfare_pension_insurance_rule(company_id, year, month, employee_id)
取得 

 指定した従業員・日付の厚生年金保険情報を返します。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 対象年 | [required] |
**month** | **i32** | 対象月 | [required] |
**employee_id** | **i32** | 従業員ID | [required] |

### Return type

[**crate::models::ApiV1EmployeesWelfarePensionInsuranceRulesControllerShowResponse**](ApiV1EmployeesWelfarePensionInsuranceRulesController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_work_record

> crate::models::ApiV1EmployeesWorkRecordSerializer get_employee_work_record(company_id, employee_id, date)
取得 

 指定した従業員・日付の勤怠情報を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**date** | **String** | 対象日(YYYY-MM-DD)(例:2018-08-01) | [required] |

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

 指定した従業員、月の勤怠情報のサマリを返します。 work_recordsオプションにtrueを指定することで、明細となる日次の勤怠情報もあわせて返却します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**employee_id** | **i32** | 従業員ID | [required] |
**year** | **i32** | 対象年 | [required] |
**month** | **i32** | 対象月 | [required] |
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
一覧の取得 

 指定した対象年月に事業所に所属する従業員をリストで返します。 - 管理者権限を持ったユーザのみ実行可能です。 - 指定した年月に退職済みユーザは取得できません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**year** | **i32** | 対象年月（年） | [required] |
**month** | **i32** | 対象年月（月） | [required] |
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


## get_groups

> crate::models::ApiV1GroupsIndexResponse get_groups(company_id)
部門情報の一覧取得 

 指定した事業所の指定日付時点における部門情報をリストで返します。 部門APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/groups-api-hierarchy\" target=\"_blank\">部門APIを利用した組織図の取得について</a> をご参照ください。 - 管理者権限を持ったユーザのみ実行可能です。

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
役職情報の一覧取得 

 指定した事業所の指定日付時点における役職情報をリストで返します。 - 管理者権限を持ったユーザのみ実行可能です。

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

> crate::models::ApiV1SalariesEmployeePayrollStatementsControllerShowResponse get_salaries_employee_payroll_statement(company_id, year, month, employee_id)
取得 

 指定した従業員ID、年月の給与明細を返します。 指定した年月に支払いのある給与明細が返されます。 - 複数時給を設定している場合はpaymentsに内訳が返されます。 - 管理者権限を持ったユーザのみ実行可能です。  # examples ``` {  \"employee_payroll_statement\": {    \"id\": 1,    \"company_id\": 1,    \"employee_id\": 1,    \"employee_name\": \"給与 太郎\",    \"employee_display_name\": \"給与 太郎\",    \"employee_num\": \"001\",    \"pay_date\": \"2018-02-25\",    \"start_date\": \"2018-02-01\",    \"closing_date\": \"2018-02-28\",    \"variable_pay_start_date\": \"2018-01-01\",    \"variable_pay_closing_date\": \"2018-01-31\",    \"fixed\": true,    \"calc_status\": \"calculated\",    \"calculated_at\": \"2018-09-27T05:06:45.315Z\",    \"pay_calc_type\": \"monthly\",    \"basic_pay_amount\": \"300000.0\",    \"work_days\": \"21.0\",    \"normal_work_time\": \"8.0\",    \"normal_work_days\": \"21.0\",    \"work_mins_by_paid_holiday\": \"480.0\",    \"num_paid_holidays\": \"1.0\",    \"is_board_member\": true,    \"total_attendance_deduction_amount\": \"0.0\",    \"total_allowance_amount\": \"0.0\",    \"total_deduction_amount\": \"23830.0\",    \"net_payment_amount\": \"277170.0\",    \"gross_payment_amount\": \"301000.0\",    \"total_worked_days_count\": \"21.0\",    \"total_taxable_payment_amount\": \"301000.0\",    \"total_expense_amount\": \"0.0\",    \"total_transfer_amount\": \"277170.0\",    \"total_annual_payment_amount\": \"600000.0\",    \"payments\": [{ \"name\": \"基本給\", \"amount\": \"300000.0\"},{ \"name\": \"残業代\", \"amount\": \"1000.0\"}],    \"deductions\": [{\"name\": \"所得税\", \"amount\": \"7000.0\"}, {\"name\": \"健康保険料\", \"amount\": \"16830.0\"}],    \"attendances\": [{\"name\": \"遅刻・早退\", \"time\": \"0.0\", \"amount\": \"0.0\"}],    \"overtime_pays\": [{ \"name\": \"時間外労働\", \"time\": \"60.0\", \"amount\": \"1000.0\", \"code\": null }, { \"name\": \"カスタム固定残業代\", \"time\": null, \"amount\": \"10000.0\", \"code\": \"a001\" }],    \"remark\": \"備考\"  } } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** |  | [required] |
**year** | **i32** |  | [required] |
**month** | **i32** |  | [required] |
**employee_id** | **i32** | Scope response to employee_id | [required] |

### Return type

[**crate::models::ApiV1SalariesEmployeePayrollStatementsControllerShowResponse**](ApiV1SalariesEmployeePayrollStatementsController.show_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_salaries_employee_payroll_statements

> crate::models::ApiV1SalariesEmployeePayrollStatementsControllerIndexResponse get_salaries_employee_payroll_statements(company_id, year, month, limit, offset)
一覧の取得 

 指定した事業所に所属する従業員の給与明細をリストで返します。 指定した年月に支払いのある給与明細が返されます。 - 複数時給を設定している場合はpaymentsに内訳が返されます。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** |  | [required] |
**year** | **i32** |  | [required] |
**month** | **i32** |  | [required] |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 100) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |

### Return type

[**crate::models::ApiV1SalariesEmployeePayrollStatementsControllerIndexResponse**](ApiV1SalariesEmployeePayrollStatementsController.index_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_me

> crate::models::ApiV1UsersMeSerializer get_users_me()
取得 

 このリクエストの認可セッションにおけるログインユーザの情報を返します。 freee人事労務では一人のログインユーザを複数の事業所に関連付けられるため、このユーザと関連のあるすべての事業所の情報をリストで返します。 他のAPIのパラメータとして `company_id` が求められる場合は、このAPIで取得した `company_id` を使用します。 給与計算対象外の従業員の `employee_id` は取得できません。

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


## update_employee

> crate::models::ApiV1EmployeesControllerUpdateResponse update_employee(id, body)
更新 

 指定した従業員の情報を更新します。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Scope response to id | [required] |
**body** | Option<[**ApiV1EmployeesControllerUpdateBody**](ApiV1EmployeesControllerUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesControllerUpdateResponse**](ApiV1EmployeesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_bank_account_rule

> crate::models::ApiV1EmployeesBankAccountRulesControllerUpdateResponse update_employee_bank_account_rule(employee_id, body)
更新 

 指定した従業員の銀行口座1の情報を更新します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesBankAccountRulesControllerUpdateBody**](ApiV1EmployeesBankAccountRulesControllerUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesBankAccountRulesControllerUpdateResponse**](ApiV1EmployeesBankAccountRulesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_basic_pay_rule

> crate::models::ApiV1EmployeesBasicPayRulesControllerUpdateResponse update_employee_basic_pay_rule(employee_id, body)
更新 

 指定した従業員の基本給情報を更新します。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesBasicPayRulesControllerUpdateBody**](ApiV1EmployeesBasicPayRulesControllerUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesBasicPayRulesControllerUpdateResponse**](ApiV1EmployeesBasicPayRulesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_health_insurance_rule

> crate::models::ApiV1EmployeesHealthInsuranceRulesControllerUpdateResponse update_employee_health_insurance_rule(employee_id, body)
更新 

 指定した従業員の健康保険情報を更新します。 - 管理者権限を持ったユーザのみ実行可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesHealthInsuranceRulesControllerUpdateBody**](ApiV1EmployeesHealthInsuranceRulesControllerUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesHealthInsuranceRulesControllerUpdateResponse**](ApiV1EmployeesHealthInsuranceRulesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_profile_rule

> crate::models::ApiV1EmployeesProfileRulesControllerUpdateResponse update_employee_profile_rule(employee_id, body)
更新 

 指定した従業員の姓名・住所などを更新します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesProfileRulesControllerUpdateBody**](ApiV1EmployeesProfileRulesControllerUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesProfileRulesControllerUpdateResponse**](ApiV1EmployeesProfileRulesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_welfare_pension_insurance_rule

> crate::models::ApiV1EmployeesWelfarePensionInsuranceRulesControllerUpdateResponse update_employee_welfare_pension_insurance_rule(employee_id, body)
更新 

 指定した従業員の厚生年金保険情報を更新します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**body** | Option<[**ApiV1EmployeesWelfarePensionInsuranceRulesControllerUpdateBody**](ApiV1EmployeesWelfarePensionInsuranceRulesControllerUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesWelfarePensionInsuranceRulesControllerUpdateResponse**](ApiV1EmployeesWelfarePensionInsuranceRulesController.update_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_work_record

> crate::models::ApiV1EmployeesWorkRecordSerializer update_employee_work_record(employee_id, date, body)
更新 

 指定した従業員の勤怠情報を更新します。 ※振替出勤・振替休日の登録はAPIでは行うことができません。  # examples  出勤日について出退勤時刻および休憩時間を更新する場合は以下のようなパラメータをリクエストします。  ``` {   \"work_record\": {     \"company_id\": 1,     \"break_records\": [       {         \"clock_in_at\": \"2017-05-25 12:00:00\",         \"clock_out_at\": \"2017-05-25 13:00:00\"       }     ],     \"clock_in_at\": \"2017-05-25 09:10:00\",     \"clock_out_at\": \"2017-05-25 18:20:00\"   } } ```  勤務パターンや既定の所定労働時間を変更する場合は use_default_work_pattern に false を指定するとともに、各設定を上書きするパラメータをリクエストします。  ``` {   \"work_record\": {     \"company_id\": 1,     \"break_records\": [       {         \"clock_in_at\": \"2017-05-25 12:00:00\",         \"clock_out_at\": \"2017-05-25 13:00:00\"       }     ],     \"clock_in_at\": \"2017-05-25 09:10:00\",     \"clock_out_at\": \"2017-05-25 18:20:00\",     \"day_pattern\": \"normal_day\",     \"normal_work_clock_in_at\": \"2017-05-25 11:00:00\",     \"normal_work_clock_out_at\": \"2017-12-20 20:00:00\",     \"normal_work_mins\": 0,     \"use_default_work_pattern\": false   } } ```  有給取得時の連携について 半休の場合は通常勤務のように勤務開始・終了時間を指定しつつ、加えて以下の２つの要素を指定することで API での勤怠をつけることができます。     - paid_holiday (半休の場合は 0.5 とします)     - normal_work_mins_by_paid_holiday (半休により計上される所定労働時間を分で指定します)  ``` {   \"work_record\": {     \"company_id\": 1,     \"break_records\": [       {         \"clock_in_at\": \"2017-05-25 12:00:00\",         \"clock_out_at\": \"2017-05-25 13:00:00\"       }     ],     \"clock_in_at\": \"2017-05-25 09:10:00\",     \"clock_out_at\": \"2017-05-25 18:20:00\",     \"paid_holiday\": 0.5,     \"normal_work_mins_by_paid_holiday\": 240   } } ```  欠勤を付ける場合は company_idとis_absence 以外のパラメータは必要ありません。  ``` {   \"work_record\": {     \"company_id\": 1,     \"is_absence\": true   } } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**date** | **String** | 対象日(YYYY-MM-DD)(例:2018-08-01) | [required] |
**body** | Option<[**ApiV1EmployeesWorkRecordsControllerUpdateBody**](ApiV1EmployeesWorkRecordsControllerUpdateBody.md)> |  |  |

### Return type

[**crate::models::ApiV1EmployeesWorkRecordSerializer**](ApiV1EmployeesWorkRecordSerializer.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_employee_work_record_summary

> crate::models::ApiV1EmployeesWorkRecordSummarySerializer update_employee_work_record_summary(employee_id, year, month, api_v1_employees_work_record_summary_controller_update_body)
勤怠情報月次サマリの更新 

 指定した従業員、月の勤怠情報のサマリを更新します。勤怠データが存在しない場合は新規作成、既に存在する場合は上書き更新されます。 ※日毎の勤怠の更新はこのAPIではできません。日毎の勤怠の操作には勤怠APIを使用して下さい。 ※管理者権限を持ったユーザのみ実行可能です。  ## 更新可能な項目 ※値が設定された項目のみ更新されます。値が設定されなかった場合は自動的に0が設定されます。 - work_days：総勤務日数 - work_days_on_weekdays：所定労働日の勤務日数 - work_days_on_prescribed_holidays：所定休日の勤務日数 - work_days_on_legal_holidays：法定休日の勤務日数 - total_work_mins：労働時間（分） - total_normal_work_mins：所定労働時間（分） - total_excess_statutory_work_mins：給与計算に用いられる法定内残業時間（分） - total_holiday_work_mins：法定休日労働時間（分） - total_latenight_work_mins：深夜労働時間（分） - total_actual_excess_statutory_work_mins：実労働時間ベースの法定内残業時間（分） - total_overtime_work_mins：時間外労働時間（分） - num_absences：欠勤日数 - num_absences_for_deduction：控除対象の欠勤日数 - total_lateness_mins：遅刻時間（分） - total_lateness_mins_for_deduction：控除対象の遅刻時間（分） - total_early_leaving_mins：早退時間（分） - total_early_leaving_mins_for_deduction：控除対象の早退時間（分） - num_paid_holidays：有給取得日数 - total_shortage_work_mins：不足時間（分）（フレックスタイム制でのみ使用） - total_deemed_paid_excess_statutory_work_mins：支給対象の法定内残業時間（分）（裁量労働制でのみ使用） - total_deemed_paid_overtime_except_normal_work_mins：支給対象の時間外労働時間（分）（裁量労働制でのみ使用）  # examples  勤怠情報を更新する場合は以下のようなパラメータをリクエストします。  ``` {   \"work_days\": 20,   \"work_days_on_weekdays\": 20,   \"work_days_on_prescribed_holidays\": 0,   \"work_days_on_legal_holidays\": 0,   \"total_work_mins\": 9600,   \"total_normal_work_mins\": 9000,   \"total_excess_statutory_work_mins\": 600,   \"total_holiday_work_mins\": 0,   \"total_latenight_work_mins\": 0,   \"total_actual_excess_statutory_work_mins\": 0,   \"total_overtime_work_mins\": 600,   \"num_absences\": 0,   \"num_absences_for_deduction\": 0,   \"total_lateness_mins\": 60,   \"total_lateness_mins_for_deduction\": 60,   \"total_early_leaving_mins\": 60,   \"total_early_leaving_mins_for_deduction\": 60,   \"num_paid_holidays\": 2 } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | 従業員ID | [required] |
**year** | **i32** | 対象年 | [required] |
**month** | **i32** | 対象月 | [required] |
**api_v1_employees_work_record_summary_controller_update_body** | Option<[**ApiV1EmployeesWorkRecordSummaryControllerUpdateBody**](ApiV1EmployeesWorkRecordSummaryControllerUpdateBody.md)> |  |  |

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
部門情報の更新 

 指定した事業所の部門の情報を更新します。 部門APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/groups-api-hierarchy\" target=\"_blank\">部門APIを利用した組織図の取得について</a> をご参照ください。 - 管理者権限を持ったユーザのみ実行可能です。

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
役職情報の更新 

 指定した事業所の役職の情報を更新します。 - 管理者権限を持ったユーザのみ実行可能です。

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

