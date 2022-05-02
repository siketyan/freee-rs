# \PaymentRequestsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment_request**](PaymentRequestsApi.md#create_payment_request) | **POST** /api/1/payment_requests | 支払依頼の作成
[**destroy_payment_request**](PaymentRequestsApi.md#destroy_payment_request) | **DELETE** /api/1/payment_requests/{id} | 支払依頼の削除
[**get_payment_request**](PaymentRequestsApi.md#get_payment_request) | **GET** /api/1/payment_requests/{id} | 支払依頼詳細の取得
[**get_payment_requests**](PaymentRequestsApi.md#get_payment_requests) | **GET** /api/1/payment_requests | 支払依頼一覧の取得
[**update_payment_request**](PaymentRequestsApi.md#update_payment_request) | **PUT** /api/1/payment_requests/{id} | 支払依頼の更新
[**update_payment_request_action**](PaymentRequestsApi.md#update_payment_request_action) | **POST** /api/1/payment_requests/{id}/actions | 支払依頼の承認操作



## create_payment_request

> crate::models::PaymentRequestResponse create_payment_request(payment_request_create_params)
支払依頼の作成

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の支払依頼を作成する</p>  <p>支払依頼APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-payment-requests\" target=\"_blank\">freee会計の支払依頼APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している支払依頼は本API経由で作成ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>本APIでは支払依頼の項目行一覧(payment_request_lines)は、最大100行までになります。</li>   <li>個人アカウントの場合は、ご利用になれません。</li>   <li>法人アカウントの場合は、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_request_create_params** | Option<[**PaymentRequestCreateParams**](PaymentRequestCreateParams.md)> | 支払依頼の作成 |  |

### Return type

[**crate::models::PaymentRequestResponse**](paymentRequestResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_payment_request

> destroy_payment_request(id, company_id)
支払依頼の削除

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の支払依頼を削除する</p>  <p>支払依頼APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-payment-requests\" target=\"_blank\">freee会計の支払依頼APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、支払依頼の承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）を行うことができます。</li>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li> 　<li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している支払依頼はAPI経由で承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>個人アカウントの場合は、ご利用になれません。</li>   <li>法人アカウントの場合は、プロフェッショナルプラン、エンタープライズプランでご利用いただけます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 支払依頼ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_request

> crate::models::PaymentRequestResponse get_payment_request(id, company_id)
支払依頼詳細の取得

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の支払依頼を取得する</p>  <p>支払依頼APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-payment-requests\" target=\"_blank\">freee会計の支払依頼APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している支払依頼と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>個人アカウントの場合は、ご利用になれません。</li>   <li>法人アカウントの場合は、プロフェッショナルプラン、エンタープライズプランでご利用いただけます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 支払依頼ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::PaymentRequestResponse**](paymentRequestResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_requests

> crate::models::PaymentRequestsIndexResponse get_payment_requests(company_id, status, start_application_date, end_application_date, start_issue_date, end_issue_date, application_number, title, applicant_id, approver_id, min_amount, max_amount, partner_id, partner_code, payment_method, start_payment_date, end_payment_date, document_code, offset, limit)
支払依頼一覧の取得

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の支払依頼一覧を取得する</p>  <p>支払依頼APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-payment-requests\" target=\"_blank\">freee会計の支払依頼APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、支払依頼の一覧を取得することができます。</li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している支払依頼と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>個人アカウントの場合は、ご利用になれません。</li>   <li>法人アカウントの場合は、プロフェッショナルプラン、エンタープライズプランでご利用いただけます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**status** | Option<**String**> | '申請ステータス(draft:下書き, in_progress:申請中, approved:承認済, rejected:却下, feedback:差戻し)、 取引ステータス(unsettled:支払待ち, settled:支払済み)'<br> approver_id を指定した場合は無効です。  |  |
**start_application_date** | Option<**String**> | 申請日で絞込：開始日(yyyy-mm-dd) |  |
**end_application_date** | Option<**String**> | 申請日で絞込：終了日(yyyy-mm-dd) |  |
**start_issue_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_issue_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**application_number** | Option<**i32**> | 申請No. |  |
**title** | Option<**String**> | 申請タイトル |  |
**applicant_id** | Option<**i32**> | 申請者のユーザーID |  |
**approver_id** | Option<**i32**> | 承認者のユーザーID<br> 'approver_id を指定した場合は、 in_progress: 申請中 の支払依頼のみを返します。'  |  |
**min_amount** | Option<**i32**> | 金額で絞込 (下限金額) |  |
**max_amount** | Option<**i32**> | 金額で絞込 (上限金額) |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込 |  |
**partner_code** | Option<**String**> | 取引先コードで絞込 |  |
**payment_method** | Option<**String**> | 支払方法で絞込 (none: 指定なし, domestic_bank_transfer: 国内振込, abroad_bank_transfer: 国外振込, account_transfer: 口座振替, credit_card: クレジットカード) |  |
**start_payment_date** | Option<**String**> | 支払期限で絞込：開始日(yyyy-mm-dd) |  |
**end_payment_date** | Option<**String**> | 支払期限で絞込：終了日(yyyy-mm-dd) |  |
**document_code** | Option<**String**> | 請求書番号で絞込 |  |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 500) |  |

### Return type

[**crate::models::PaymentRequestsIndexResponse**](paymentRequestsIndexResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_payment_request

> crate::models::PaymentRequestResponse update_payment_request(id, payment_request_update_params)
支払依頼の更新

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の支払依頼を更新する</p>  <p>支払依頼APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-payment-requests\" target=\"_blank\">freee会計の支払依頼APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、支払依頼を更新することができます。</li>   <li>本APIでは、status(申請ステータス): draft:下書き, in_progress:申請中, feedback:差戻しのみ更新可能です。</li>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している支払依頼は本API経由で更新ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>申請ステータスが申請中の場合には、申請タイトル(title)、申請日(application_date)、申請経路ID(approval_flow_route_id)、承認者のユーザーID(approver_id)は更新ができません。</li>   <li>本APIでは支払依頼の項目行一覧(payment_request_lines)は、最大100行までになります。</li>   <li>個人アカウントの場合は、ご利用になれません。</li>   <li>法人アカウントの場合は、プロフェッショナルプラン、エンタープライズプランでご利用いただけます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 支払依頼ID | [required] |
**payment_request_update_params** | Option<[**PaymentRequestUpdateParams**](PaymentRequestUpdateParams.md)> | 支払依頼の更新 |  |

### Return type

[**crate::models::PaymentRequestResponse**](paymentRequestResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_payment_request_action

> crate::models::PaymentRequestResponse update_payment_request_action(id, payment_request_action_create_params)
支払依頼の承認操作

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の支払依頼の承認操作を行う</p>  <p>支払依頼APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-payment-requests\" target=\"_blank\">freee会計の支払依頼APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、支払依頼の承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）を行うことができます。</li>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li> 　<li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している支払依頼はAPI経由で承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>個人アカウントの場合は、ご利用になれません。</li>   <li>法人アカウントの場合は、プロフェッショナルプラン、エンタープライズプランでご利用いただけます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 支払依頼ID | [required] |
**payment_request_action_create_params** | [**PaymentRequestActionCreateParams**](PaymentRequestActionCreateParams.md) | 支払依頼の承認操作 | [required] |

### Return type

[**crate::models::PaymentRequestResponse**](paymentRequestResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

