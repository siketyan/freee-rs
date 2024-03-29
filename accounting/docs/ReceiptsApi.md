# \ReceiptsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_receipt**](ReceiptsApi.md#create_receipt) | **POST** /api/1/receipts | ファイルボックス（証憑ファイル）のアップロード
[**destroy_receipt**](ReceiptsApi.md#destroy_receipt) | **DELETE** /api/1/receipts/{id} | ファイルボックス（証憑ファイル）の削除
[**download_receipt**](ReceiptsApi.md#download_receipt) | **GET** /api/1/receipts/{id}/download | ファイルボックス（証憑ファイル）のダウンロード
[**get_receipt**](ReceiptsApi.md#get_receipt) | **GET** /api/1/receipts/{id} | ファイルボックス（証憑ファイル）の取得
[**get_receipts**](ReceiptsApi.md#get_receipts) | **GET** /api/1/receipts | ファイルボックス（証憑ファイル）一覧の取得
[**update_receipt**](ReceiptsApi.md#update_receipt) | **PUT** /api/1/receipts/{id} | ファイルボックス（証憑ファイル）の更新



## create_receipt

> crate::models::ReceiptResponse create_receipt(company_id, receipt, description, issue_date, receipt_metadatum_partner_name, receipt_metadatum_issue_date, receipt_metadatum_amount, qualified_invoice, document_type)
ファイルボックス（証憑ファイル）のアップロード

 <h2 id=\"\">概要</h2>  <p>ファイルボックス（証憑ファイル）をアップロードする</p> <h2 id=\"_2\">注意点</h2> <ul>   <li>リクエストヘッダーの Content-Type は、multipart/form-dataにのみ対応しています。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**receipt** | **std::path::PathBuf** | 証憑ファイル | [required] |
**description** | Option<**String**> | メモ (255文字以内) |  |
**issue_date** | Option<**String**> | 取引日 (yyyy-mm-dd) |  |
**receipt_metadatum_partner_name** | Option<**String**> | 発行元 |  |
**receipt_metadatum_issue_date** | Option<**String**> | 発行日 (yyyy-mm-dd) |  |
**receipt_metadatum_amount** | Option<**i64**> | 金額 |  |
**qualified_invoice** | Option<**String**> | この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 適格請求書等（qualified: 該当する、not_qualified: 該当しない）  |  |
**document_type** | Option<**String**> | この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 書類の種類（receipt: 領収書、invoice: 請求書、other: その他）  |  |

### Return type

[**crate::models::ReceiptResponse**](receiptResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_receipt

> destroy_receipt(id, company_id)
ファイルボックス（証憑ファイル）の削除

 <h2 id=\"\">概要</h2>  <p>ファイルボックス（証憑ファイル）を削除する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ファイルボックス（証憑ファイル）ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_receipt

> String download_receipt(id, company_id)
ファイルボックス（証憑ファイル）のダウンロード

 <h2 id=\"\">概要</h2>  <p>指定した事業所のファイルボックス（証憑ファイル）をダウンロードする</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ファイルボックス（証憑ファイル）ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

**String**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/pdf, image/*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_receipt

> crate::models::ReceiptResponse get_receipt(id, company_id)
ファイルボックス（証憑ファイル）の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所のファイルボックス（証憑ファイル）を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ファイルボックス（証憑ファイル）ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::ReceiptResponse**](receiptResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_receipts

> crate::models::GetReceipts200Response get_receipts(company_id, start_date, end_date, user_name, number, comment_type, comment_important, category, offset, limit)
ファイルボックス（証憑ファイル）一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所のファイルボックス（証憑ファイル）一覧を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**start_date** | **String** | アップロード日 (yyyy-mm-dd) | [required] |
**end_date** | **String** | アップロード日 (yyyy-mm-dd) | [required] |
**user_name** | Option<**String**> | アップロードしたユーザー名、メールアドレス |  |
**number** | Option<**i32**> | アップロードファイルNo |  |
**comment_type** | Option<**String**> | posted:コメントあり, raised:未解決, resolved:解決済 |  |
**comment_important** | Option<**bool**> | trueの時、重要コメント付きが対象 |  |
**category** | Option<**String**> | all:すべて、without_deal:未登録、with_expense_application_line:経費申請中, with_deal:登録済み、ignored:無視 |  |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 3000) |  |

### Return type

[**crate::models::GetReceipts200Response**](get_receipts_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_receipt

> crate::models::ReceiptResponse update_receipt(id, receipt_update_params)
ファイルボックス（証憑ファイル）の更新

 <h2 id=\"\">概要</h2>  <p>ファイルボックス（証憑ファイル）を更新する</p> <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、証憑ファイルの再アップロードはできません。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ファイルボックス（証憑ファイル）ID | [required] |
**receipt_update_params** | [**ReceiptUpdateParams**](ReceiptUpdateParams.md) |  | [required] |

### Return type

[**crate::models::ReceiptResponse**](receiptResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

