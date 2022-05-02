# \ApprovalRequestsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_approval_request**](ApprovalRequestsApi.md#create_approval_request) | **POST** /api/1/approval_requests | 各種申請の作成
[**destroy_approval_request**](ApprovalRequestsApi.md#destroy_approval_request) | **DELETE** /api/1/approval_requests/{id} | 各種申請の削除
[**get_approval_request**](ApprovalRequestsApi.md#get_approval_request) | **GET** /api/1/approval_requests/{id} | 各種申請の取得
[**get_approval_request_form**](ApprovalRequestsApi.md#get_approval_request_form) | **GET** /api/1/approval_requests/forms/{id} | 各種申請の申請フォームの取得
[**get_approval_request_forms**](ApprovalRequestsApi.md#get_approval_request_forms) | **GET** /api/1/approval_requests/forms | 各種申請の申請フォーム一覧の取得
[**get_approval_requests**](ApprovalRequestsApi.md#get_approval_requests) | **GET** /api/1/approval_requests | 各種申請の一覧
[**update_approval_request**](ApprovalRequestsApi.md#update_approval_request) | **PUT** /api/1/approval_requests/{id} | 各種申請の更新
[**update_approval_request_action**](ApprovalRequestsApi.md#update_approval_request_action) | **POST** /api/1/approval_requests/{id}/actions | 各種申請の承認操作



## create_approval_request

> crate::models::ApprovalRequestResponse create_approval_request(approval_request_create_params)
各種申請の作成

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の各種申請を作成する</p>  <p>各種申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-approval-requests\" target=\"_blank\">freee会計の各種申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、各種申請を作成することができます。</li>   <li>     申請項目(request_items)については、申請フォームで設定された項目のIDとそれに対応する値を入力してください。     <ul>       <li>タイトル(title)：文字列(改行なし) 例)予算申請</li>       <li>1行コメント(single_line)：文字列(改行なし) 例)予算に関する申請</li>       <li>複数行コメント(multi_line)：文字列(改行あり)       <br>       &nbsp;&nbsp;例)<br>       &nbsp;&nbsp;予算に関する申請<br>       &nbsp;&nbsp;申請日 2019-12-17<br>       </li>       <li>プルダウン(select)： プルダウンの選択肢の名前(改行なし) 例)開発部</li>       <li>日付(date)： 日付形式 例)2019-12-17</li>       <li>金額(amount)： 数値(申請フォームで設定した上限・下限金額内の値, 改行なし) 例)10000</li>       <li>添付ファイル(receipt)： ファイルボックスAPIのID 例)1</li>       <li>部門ID(section)： 部門APIのID 例)1</li>       <li>取引先ID(partner)： 取引先APIのID 例)1</li>     </ul>   </li>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している各種申請は本API経由で作成ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>     申請フォームの項目に契約書（freeeサイン連携）が利用されている各種申請については、API経由で参照は可能ですが、作成と更新ができません。   </li>   <li>Web画面やAPIを通じて申請フォームが変更されると、本APIで使用する項目ID（request_itemsで指定するid）も変更されます。本API利用前に各種申請の取得APIを利用し、最新の申請フォームを取得することを推奨します。</li>   <li>本APIはプロフェッショナルプラン、エンタープライズプランをご利用の事業所のみ利用可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**approval_request_create_params** | Option<[**ApprovalRequestCreateParams**](ApprovalRequestCreateParams.md)> | 各種申請の作成 |  |

### Return type

[**crate::models::ApprovalRequestResponse**](approvalRequestResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_approval_request

> destroy_approval_request(id, company_id)
各種申請の削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所の各種申請を削除する</p>  <p>各種申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-approval-requests\" target=\"_blank\">freee会計の各種申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li>   <li>本APIはプロフェッショナルプラン、エンタープライズプランをご利用の事業所のみ利用可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 各種申請ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_request

> crate::models::ApprovalRequestResponse get_approval_request(id, company_id)
各種申請の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の各種申請を取得する</p>  <p>各種申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-approval-requests\" target=\"_blank\">freee会計の各種申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している各種申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>     申請フォームの項目に契約書（freeeサイン連携）が利用されている各種申請については、API経由で参照は可能ですが、作成と更新ができません。   </li>   <li>本APIはプロフェッショナルプラン、エンタープライズプランをご利用の事業所のみ利用可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 各種申請ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::ApprovalRequestResponse**](approvalRequestResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_request_form

> crate::models::ApprovalRequestFormResponse get_approval_request_form(id, company_id)
各種申請の申請フォームの取得

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の各種申請の申請フォームを取得する</p>  <p>各種申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-approval-requests\" target=\"_blank\">freee会計の各種申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIはプロフェッショナルプラン、エンタープライズプランをご利用の事業所のみ利用可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 申請フォームID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::ApprovalRequestFormResponse**](approvalRequestFormResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_request_forms

> crate::models::ApprovalRequestFormIndexResponse get_approval_request_forms(company_id)
各種申請の申請フォーム一覧の取得

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の各種申請の申請フォーム一覧を取得する</p>  <p>各種申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-approval-requests\" target=\"_blank\">freee会計の各種申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIはプロフェッショナルプラン、エンタープライズプランをご利用の事業所のみ利用可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::ApprovalRequestFormIndexResponse**](approvalRequestFormIndexResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_requests

> crate::models::ApprovalRequestsIndexResponse get_approval_requests(company_id, status, application_number, title, form_id, start_application_date, end_application_date, applicant_id, min_amount, max_amount, approver_id, offset, limit)
各種申請の一覧

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の各種申請一覧を取得する</p>  <p>各種申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-approval-requests\" target=\"_blank\">freee会計の各種申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、各種申請の一覧を取得することができます。</li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している各種申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>     申請フォームの項目に契約書（freeeサイン連携）が利用されている各種申請については、API経由で参照は可能ですが、作成と更新ができません。   </li>   <li>本APIはプロフェッショナルプラン、エンタープライズプランをご利用の事業所のみ利用可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**status** | Option<**String**> | 申請ステータス(draft:下書き, in_progress:申請中, approved:承認済, rejected:却下, feedback:差戻し) 承認者指定時には無効です。 |  |
**application_number** | Option<**i32**> | 申請No. |  |
**title** | Option<**String**> | 申請タイトル |  |
**form_id** | Option<**i32**> | 申請フォームID |  |
**start_application_date** | Option<**String**> | 申請日で絞込：開始日(yyyy-mm-dd) |  |
**end_application_date** | Option<**String**> | 申請日で絞込：終了日(yyyy-mm-dd) |  |
**applicant_id** | Option<**i32**> | 申請者のユーザーID |  |
**min_amount** | Option<**i64**> | 金額で絞込：以上 |  |
**max_amount** | Option<**i64**> | 金額で絞込：以下 |  |
**approver_id** | Option<**i32**> | 承認者のユーザーID 承認者指定時には申請ステータスが申請中のものだけが取得可能です。 |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 500) |  |

### Return type

[**crate::models::ApprovalRequestsIndexResponse**](approvalRequestsIndexResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_approval_request

> crate::models::ApprovalRequestResponse update_approval_request(id, approval_request_update_params)
各種申請の更新

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の各種申請を更新する</p>  <p>各種申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-approval-requests\" target=\"_blank\">freee会計の各種申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、各種申請を更新することができます。</li>   <li>     申請項目(request_items)については、各種申請の取得APIで取得したrequest_items.idとそれに対応する値を入力してください。     <ul>       <li>タイトル(title)：文字列(改行なし) 例)予算申請</li>       <li>1行コメント(single_line)：文字列(改行なし) 例)予算に関する申請</li>       <li>複数行コメント(multi_line)：文字列(改行あり)       <br>       &nbsp;&nbsp;例)<br>       &nbsp;&nbsp;予算に関する申請<br>       &nbsp;&nbsp;申請日 2019-12-17<br>       </li>       <li>プルダウン(select)： プルダウンの選択肢の名前(改行なし) 例)開発部</li>       <li>日付(date)： 日付形式 例)2019-12-17</li>       <li>金額(amount)： 数値(申請フォームで設定した上限・下限金額内の値, 改行なし) 例)10000</li>       <li>添付ファイル(receipt)： ファイルボックスAPIのID 例)1</li>       <li>部門ID(section)： 部門APIのID 例)1</li>       <li>取引先ID(partner)： 取引先APIのID 例)1</li>     </ul>   </li>   <li>本APIでは、status(申請ステータス): draft:下書き, feedback:差戻しのみ更新可能です。</li>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li>   <li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している各種申請は本API経由で更新ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>     申請フォームの項目に契約書（freeeサイン連携）が利用されている各種申請については、API経由で参照は可能ですが、作成と更新ができません。   </li>   <li>Web画面やAPIを通じて申請フォームが変更されると、本APIで使用する項目ID（request_itemsで指定するid）も変更されます。本APIで使用する項目IDは申請作成時点の項目IDです。本API利用前に各種申請の取得APIを利用し、申請作成時点の項目IDを取得することを推奨します。</li>   <li>本APIはプロフェッショナルプラン、エンタープライズプランをご利用の事業所のみ利用可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 各種申請ID | [required] |
**approval_request_update_params** | [**ApprovalRequestUpdateParams**](ApprovalRequestUpdateParams.md) | 各種申請の更新 | [required] |

### Return type

[**crate::models::ApprovalRequestResponse**](approvalRequestResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_approval_request_action

> crate::models::ApprovalRequestResponse update_approval_request_action(id, approval_request_action_create_params)
各種申請の承認操作

 <h2 id=\"_1\">概要</h2>  <p>指定した事業所の各種申請の承認操作を行う</p>  <p>各種申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-approval-requests\" target=\"_blank\">freee会計の各種申請APIの使い方</a>をご参照ください</p>  <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、各種申請の承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）を行うことができます。</li>   <li>     申請ステータス(下書き、申請中)の指定と変更、及び承認操作（承認する、却下する、申請者へ差し戻す、代理承認する、承認済み・却下済みを取り消す）は以下を参考にして行ってください。     <ul>       <li>         承認操作は申請ステータスが申請中、承認済み、却下のものだけが対象です。         <ul>           <li>             初回申請の場合             <ul><li>申請の作成（POST）</li></ul>           </li>           <li>             作成済みの申請の申請ステータス変更・更新する場合             <ul><li>申請の更新（PUT）</li></ul>           </li>           <li>             申請中、承認済み、却下の申請の承認操作を行う場合             <ul><li>承認操作の実行（POST）</li></ul>           </li>         </ul>       </li>       <li>申請の削除（DELETE）が可能なのは申請ステータスが下書き、差戻しの場合のみです</li>     </ul>   </li> 　<li>     申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している各種申請はAPI経由で承認ステータスの変更ができません。     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>     申請フォームの項目に契約書（freeeサイン連携）が利用されている各種申請については、API経由で参照は可能ですが、作成と更新ができません。   </li>   <li>本APIはプロフェッショナルプラン、エンタープライズプランをご利用の事業所のみ利用可能です。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 各種申請ID | [required] |
**approval_request_action_create_params** | [**ApprovalRequestActionCreateParams**](ApprovalRequestActionCreateParams.md) | 各種申請の承認操作 | [required] |

### Return type

[**crate::models::ApprovalRequestResponse**](approvalRequestResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

