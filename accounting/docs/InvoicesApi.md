# \InvoicesApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_invoice**](InvoicesApi.md#create_invoice) | **POST** /api/1/invoices | 請求書の作成
[**destroy_invoice**](InvoicesApi.md#destroy_invoice) | **DELETE** /api/1/invoices/{id} | 請求書の削除
[**get_invoice**](InvoicesApi.md#get_invoice) | **GET** /api/1/invoices/{id} | 請求書の取得
[**get_invoices**](InvoicesApi.md#get_invoices) | **GET** /api/1/invoices | 請求書一覧の取得
[**update_invoice**](InvoicesApi.md#update_invoice) | **PUT** /api/1/invoices/{id} | 請求書の更新



## create_invoice

> crate::models::InvoiceResponse create_invoice(invoice_create_params)
請求書の作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所の請求書を作成する</p>  <h2 id=\"_1\">注意点</h2> <ul> <li> <p>partner_code, partner_idはどちらかの指定が必須です。ただし両方同時に指定することはできません。</p> </li> <li> <p>請求書ステータス(invoice_status)を発行(issue)で利用した場合、請求内容の合計金額が0円以上になる必要があります。</p> </li> <li> <p>partner_codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。</p> </li> <li> <p>本APIでは請求内容(invoice_contents)は、最大100行までになります。</p> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_create_params** | Option<[**InvoiceCreateParams**](InvoiceCreateParams.md)> | 請求書の作成 |  |

### Return type

[**crate::models::InvoiceResponse**](invoiceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_invoice

> destroy_invoice(id, company_id)
請求書の削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所の請求書を削除する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice

> crate::models::InvoiceResponse get_invoice(company_id, id)
請求書の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の請求書詳細を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** | 請求書ID | [required] |

### Return type

[**crate::models::InvoiceResponse**](invoiceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoices

> crate::models::InvoiceIndexResponse get_invoices(company_id, partner_id, partner_code, start_issue_date, end_issue_date, start_due_date, end_due_date, invoice_number, description, invoice_status, payment_status, offset, limit)
請求書一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の請求書一覧を取得する</p> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**partner_id** | Option<**i32**> | 取引先IDで絞込 |  |
**partner_code** | Option<**String**> | 取引先コードで絞込 |  |
**start_issue_date** | Option<**String**> | 請求日の開始日(yyyy-mm-dd) |  |
**end_issue_date** | Option<**String**> | 請求日の終了日(yyyy-mm-dd) |  |
**start_due_date** | Option<**String**> | 期日の開始日(yyyy-mm-dd) |  |
**end_due_date** | Option<**String**> | 期日の終了日(yyyy-mm-dd) |  |
**invoice_number** | Option<**String**> | 請求書番号 |  |
**description** | Option<**String**> | 概要 |  |
**invoice_status** | Option<**String**> | 請求書ステータス  (draft: 下書き, applying: 申請中, remanded: 差し戻し, rejected: 却下, approved: 承認済み, unsubmitted: 送付待ち, submitted: 送付済み) |  |
**payment_status** | Option<**String**> | 入金ステータス  (unsettled: 入金待ち, settled: 入金済み) |  |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 20, 最大: 100)  |  |

### Return type

[**crate::models::InvoiceIndexResponse**](invoiceIndexResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_invoice

> crate::models::InvoiceResponse update_invoice(id, invoice_update_params)
請求書の更新

 <h2 id=\"\">概要</h2>  <p>指定した事業所の請求書を更新する</p>  <h2 id=\"_1\">注意点</h2> <ul> <li> <p>入金済みの請求書に対する金額関連の変更はできません。</p> </li> <li> <p>請求書ステータスは確定(issue)のみ指定可能です。請求書ステータスを確定する時のみ指定してください。</p> </li> <li> <p>請求書WFを利用している場合、承認済み請求書は承認権限を持たないユーザーでは更新できません。</p> </li> <li> <p>更新後の請求書ステータス(invoice_status)が下書き以外の場合、請求内容の合計金額が0円以上になる必要があります。</p> </li> <li> <p>partner_code, partner_idを両方同時に指定することはできません。</p> </li> <li> <p>partner_codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。</p> </li> <li> <p>本APIでは請求内容(invoice_contents)は、最大100行までになります。</p> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 請求書ID | [required] |
**invoice_update_params** | Option<[**InvoiceUpdateParams**](InvoiceUpdateParams.md)> | 請求書の更新 |  |

### Return type

[**crate::models::InvoiceResponse**](invoiceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

