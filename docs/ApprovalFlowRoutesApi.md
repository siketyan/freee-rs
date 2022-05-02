# \ApprovalFlowRoutesApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_approval_flow_route**](ApprovalFlowRoutesApi.md#get_approval_flow_route) | **GET** /api/1/approval_flow_routes/{id} | 申請経路の取得
[**get_approval_flow_routes**](ApprovalFlowRoutesApi.md#get_approval_flow_routes) | **GET** /api/1/approval_flow_routes | 申請経路一覧の取得



## get_approval_flow_route

> crate::models::ApprovalFlowRouteResponse get_approval_flow_route(id, company_id)
申請経路の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の申請経路を取得する</p>  <p>各種申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-approval-requests\" target=\"_blank\">freee会計の各種申請APIの使い方</a>をご参照ください</p> <p>経費精算APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計の経費精算APIの使い方</a>をご参照ください</p>  <h2 id=\"\">注意点</h2>  <ul>   <li>     <p>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</p>     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 経路申請ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::ApprovalFlowRouteResponse**](approvalFlowRouteResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_flow_routes

> crate::models::ApprovalFlowRoutesIndexResponse get_approval_flow_routes(company_id, included_user_id, usage, request_form_id)
申請経路一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の申請経路一覧を取得する</p>  <p>各種申請APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-approval-requests\" target=\"_blank\">freee会計の各種申請APIの使い方</a>をご参照ください</p> <p>経費精算APIの使い方については、<a href=\"https://developer.freee.co.jp/tips/accounting-expense-applications\" target=\"_blank\">freee会計の経費精算APIの使い方</a>をご参照ください</p>  <h2 id=\"\">注意点</h2>  <ul>   <li>     <p>申請経路、承認者の指定として部門役職データ連携を活用し、以下のいずれかを利用している申請と申請経路はAPI経由で参照は可能ですが、作成と更新、承認ステータスの変更ができません。</p>     <ul>       <li>役職指定（申請者の所属部門）</li>       <li>役職指定（申請時に部門指定）</li>       <li>部門および役職指定</li>     </ul>   </li>   <li>個人アカウントの場合は、プレミアムプランでご利用できます。</li>   <li>法人アカウントの場合は、ベーシックプラン、プロフェッショナルプラン、エンタープライズプランでご利用できます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**included_user_id** | Option<**i32**> | 経路に含まれるユーザーのユーザーID |  |
**usage** | Option<**String**> | 申請種別（各申請種別が使用できる申請経路に絞り込めます。例えば、ApprovalRequest を指定すると、各種申請が使用できる申請経路に絞り込めます。） * `TxnApproval` - 仕訳承認 * `ExpenseApplication` - 経費精算 * `PaymentRequest` - 支払依頼 * `ApprovalRequest` - 各種申請 * `DocApproval` - 請求書等 (見積書・納品書・請求書・発注書) |  |
**request_form_id** | Option<**i32**> | 申請フォームID request_form_id指定時はusage条件をApprovalRequestに指定してください。指定しない場合無効になります。 |  |

### Return type

[**crate::models::ApprovalFlowRoutesIndexResponse**](approvalFlowRoutesIndexResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

