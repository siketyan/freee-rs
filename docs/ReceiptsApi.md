# \ReceiptsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_receipt**](ReceiptsApi.md#create_receipt) | **POST** /api/1/receipts | ファイルボックス 証憑ファイルアップロード
[**destroy_receipt**](ReceiptsApi.md#destroy_receipt) | **DELETE** /api/1/receipts/{id} | ファイルボックス 証憑ファイルを削除する
[**download_receipt**](ReceiptsApi.md#download_receipt) | **GET** /api/1/receipts/{id}/download | ファイルボックス 証憑ファイルのダウンロード
[**get_receipt**](ReceiptsApi.md#get_receipt) | **GET** /api/1/receipts/{id} | ファイルボックス 証憑ファイルの取得
[**get_receipts**](ReceiptsApi.md#get_receipts) | **GET** /api/1/receipts | ファイルボックス 証憑ファイル一覧の取得
[**update_receipt**](ReceiptsApi.md#update_receipt) | **PUT** /api/1/receipts/{id} | ファイルボックス 証憑ファイル情報更新



## create_receipt

> crate::models::ReceiptResponse create_receipt(company_id, receipt, description, issue_date)
ファイルボックス 証憑ファイルアップロード

 <h2 id=\"\">概要</h2>  <p>ファイルボックスに証憑ファイルをアップロードする</p> <h2 id=\"_2\">注意点</h2> <ul>   <li>リクエストヘッダーの Content-Type は、multipart/form-dataにのみ対応しています。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**receipt** | **std::path::PathBuf** | 証憑ファイル | [required] |
**description** | Option<**String**> | メモ (255文字以内) |  |
**issue_date** | Option<**String**> | 取引日 (yyyy-mm-dd) |  |

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
ファイルボックス 証憑ファイルを削除する

 <h2 id=\"\">概要</h2>  <p>ファイルボックスの証憑ファイルを削除する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 証憑ファイルID | [required] |
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
ファイルボックス 証憑ファイルのダウンロード

 <h2 id=\"\">概要</h2>  <p>指定した事業所のファイルボックス 証憑ファイルのダウンロードをする</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 証憑ファイルID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

**String**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/pdf, image/_*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_receipt

> crate::models::ReceiptResponse get_receipt(id, company_id)
ファイルボックス 証憑ファイルの取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所のファイルボックス 証憑ファイルを取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 証憑ファイルID | [required] |
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

> crate::models::InlineResponse20014 get_receipts(company_id, start_date, end_date, user_name, number, comment_type, comment_important, category, offset, limit)
ファイルボックス 証憑ファイル一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所のファイルボックス 証憑ファイル一覧を取得する</p>

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

[**crate::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_receipt

> crate::models::ReceiptResponse update_receipt(id, receipt_update_params)
ファイルボックス 証憑ファイル情報更新

 <h2 id=\"\">概要</h2>  <p>ファイルボックスの証憑ファイル情報を更新する</p> <h2 id=\"_2\">注意点</h2> <ul>   <li>本APIでは、証憑ファイルの再アップロードはできません。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 証憑ファイルID | [required] |
**receipt_update_params** | [**ReceiptUpdateParams**](ReceiptUpdateParams.md) | 経費申請の更新 | [required] |

### Return type

[**crate::models::ReceiptResponse**](receiptResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

