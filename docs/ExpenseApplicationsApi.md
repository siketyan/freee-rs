# \ExpenseApplicationsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_expense_application**](ExpenseApplicationsApi.md#create_expense_application) | **POST** /api/1/expense_applications | 経費申請の作成
[**destroy_expense_application**](ExpenseApplicationsApi.md#destroy_expense_application) | **DELETE** /api/1/expense_applications/{id} | 経費申請の削除
[**get_expense_application**](ExpenseApplicationsApi.md#get_expense_application) | **GET** /api/1/expense_applications/{id} | 経費申請詳細の取得
[**get_expense_applications**](ExpenseApplicationsApi.md#get_expense_applications) | **GET** /api/1/expense_applications | 経費申請一覧の取得
[**update_expense_application**](ExpenseApplicationsApi.md#update_expense_application) | **PUT** /api/1/expense_applications/{id} | 経費申請の更新
[**update_expense_application_action**](ExpenseApplicationsApi.md#update_expense_application_action) | **POST** /api/1/expense_applications/{id}/actions | 経費申請の承認操作



## create_expense_application

> crate::models::ExpenseApplicationResponse create_expense_application(expense_application_create_params)
経費申請の作成

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の経費申請を作成する</p>  <p>経費申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計の経費申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している経費申請は本API経由で作成ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>申請時には、申請タイトル(title)に加え、項目行については金額(amount)、日付(transaction_date)、内容(description)が必須項目となります。申請時の業務効率化のため、API入力をお勧めします。</li>   <li>本APIは駅すぱあと連携 (出発駅と到着駅から金額を自動入力する機能)には非対応です。駅すぱあと連携を使用した経費申請は作成できません。</li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expense_application_create_params** | Option<[**ExpenseApplicationCreateParams**](ExpenseApplicationCreateParams.md)> | 経費申請の作成 |  |

### Return type

[**crate::models::ExpenseApplicationResponse**](expenseApplicationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_expense_application

> destroy_expense_application(id, company_id)
経費申請の削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所の経費申請を削除する</p>  <p>経費申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計の経費申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>       <li>自分が申請者でない申請の削除が可能なのはユーザーの権限が管理者権限、且つ申請ステータスが差し戻しの場合のみです</li>     </ul>   </li>   <li>本APIは駅すぱあと連携 (出発駅と到着駅から金額を自動入力する機能)には非対応です。駅すぱあと連携を使用した経費申請は削除できません。</li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 経費申請ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_expense_application

> crate::models::ExpenseApplicationResponse get_expense_application(id, company_id)
経費申請詳細の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の経費申請を取得する</p>  <p>経費申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計の経費申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している経費申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>本APIは駅すぱあと連携 (出発駅と到着駅から金額を自動入力する機能)には非対応です。駅すぱあと連携を使用した経費申請は取得できません。</li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 経費申請ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::ExpenseApplicationResponse**](expenseApplicationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_expense_applications

> crate::models::ExpenseApplicationsIndexResponse get_expense_applications(company_id, status, payroll_attached, start_transaction_date, end_transaction_date, application_number, title, start_issue_date, end_issue_date, applicant_id, approver_id, min_amount, max_amount, offset, limit)
経費申請一覧の取得

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の経費申請一覧を取得する</p>  <p>経費申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計の経費申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、経費申請の一覧を取得することができます。</li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している経費申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**status** | Option<**String**> | 申請ステータス(draft:下書き, in_progress:申請中, approved:承認済, rejected:却下, feedback:差戻し)、 取引ステータス(unsettled:清算待ち, settled:精算済み) |  |
**payroll_attached** | Option<**bool**> | true:給与連携あり、false:給与連携なし、未指定時:絞り込みなし |  |
**start_transaction_date** | Option<**String**> | 発生日(経費申請項目の日付)で絞込：開始日(yyyy-mm-dd) |  |
**end_transaction_date** | Option<**String**> | 発生日(経費申請項目の日付)で絞込：終了日(yyyy-mm-dd) |  |
**application_number** | Option<**i32**> | 申請No. |  |
**title** | Option<**String**> | 申請タイトル |  |
**start_issue_date** | Option<**String**> | 申請日で絞込：開始日(yyyy-mm-dd) |  |
**end_issue_date** | Option<**String**> | 申請日で絞込：終了日(yyyy-mm-dd) |  |
**applicant_id** | Option<**i32**> | 申請者のユーザーID |  |
**approver_id** | Option<**i32**> | 承認者のユーザーID |  |
**min_amount** | Option<**i32**> | 金額で絞込 (下限金額) |  |
**max_amount** | Option<**i32**> | 金額で絞込 (上限金額) |  |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 500) |  |

### Return type

[**crate::models::ExpenseApplicationsIndexResponse**](expenseApplicationsIndexResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_expense_application

> crate::models::ExpenseApplicationResponse update_expense_application(id, expense_application_update_params)
経費申請の更新

 <h2 id=\"\">概要</h2>  <p>指定した事業所の経費申請を更新する</p>  <p>経費申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計の経費申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、経費申請を更新することができます。</li>   <li>本APIでは、status(申請ステータス): draft:下書き, feedback:差戻しのみ更新可能です。</li>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している経費申請は本API経由で更新ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>申請時には、申請タイトル(title)に加え、項目行については金額(amount)、日付(transaction_date)、内容(description)が必須項目となります。申請時の業務効率化のため、API入力をお勧めします。</li>   <li>本APIは駅すぱあと連携 (出発駅と到着駅から金額を自動入力する機能)には非対応です。駅すぱあと連携を使用した経費申請は更新できません。</li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 経費申請ID | [required] |
**expense_application_update_params** | Option<[**ExpenseApplicationUpdateParams**](ExpenseApplicationUpdateParams.md)> | 経費申請の更新 |  |

### Return type

[**crate::models::ExpenseApplicationResponse**](expenseApplicationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_expense_application_action

> crate::models::ExpenseApplicationResponse update_expense_application_action(id, expense_application_action_create_params)
経費申請の承認操作

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の経費申請の承認操作を行う</p>  <p>経費申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計の経費申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、経費申請の承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）を行うことができます。</li>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li> 　<li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している経費申請はAPI経由で承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>本APIは駅すぱあと連携 (出発駅と到着駅から金額を自動入力する機能)には非対応です。駅すぱあと連携を使用した経費申請は承認操作できません。</li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 経費申請ID | [required] |
**expense_application_action_create_params** | [**ExpenseApplicationActionCreateParams**](ExpenseApplicationActionCreateParams.md) | 経費申請の承認操作 | [required] |

### Return type

[**crate::models::ExpenseApplicationResponse**](expenseApplicationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

