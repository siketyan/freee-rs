# \WalletTxnsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_wallet_txn**](WalletTxnsApi.md#create_wallet_txn) | **POST** /api/1/wallet_txns | 明細の作成
[**destroy_wallet_txn**](WalletTxnsApi.md#destroy_wallet_txn) | **DELETE** /api/1/wallet_txns/{id} | 明細の削除
[**get_wallet_txn**](WalletTxnsApi.md#get_wallet_txn) | **GET** /api/1/wallet_txns/{id} | 明細の取得
[**get_wallet_txns**](WalletTxnsApi.md#get_wallet_txns) | **GET** /api/1/wallet_txns | 明細一覧の取得



## create_wallet_txn

> crate::models::WalletTxnResponse create_wallet_txn(wallet_txn_params)
明細の作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所の明細を作成する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>amount : 明細金額</p> </li>  <li> <p>due_amount : 取引登録待ち金額</p> </li>  <li> <p>balance : 残高</p> </li>  <li> <p>entry_side</p>  <ul> <li>income : 入金</li>  <li>expense : 出金</li> </ul> </li>  <li> <p>walletable_type</p>  <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_txn_params** | Option<[**WalletTxnParams**](WalletTxnParams.md)> | 明細の作成 |  |

### Return type

[**crate::models::WalletTxnResponse**](walletTxnResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_wallet_txn

> destroy_wallet_txn(id, company_id)
明細の削除

 <h2 id=\"\">概要</h2> <p>指定した事業所の明細を削除する</p>  <h2 id=\"\">注意点</h2> <ul>   <li>同期をして取得したデータが「明細」の場合は、削除および再取得はできません。</li>   <li>詳細は<a target=\"_blank\" href=\"https://support.freee.co.jp/hc/ja/articles/360015892332\">freeeヘルプセンター</a>をご確認ください。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 明細ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallet_txn

> crate::models::WalletTxnResponse get_wallet_txn(id, company_id)
明細の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の明細を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>amount : 明細金額</p> </li>  <li> <p>due_amount : 取引登録待ち金額</p> </li>  <li> <p>balance : 残高</p> </li>  <li> <p>entry_side</p>  <ul> <li>income : 入金</li>  <li>expense : 出金</li> </ul> </li>  <li> <p>walletable_type</p>  <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 明細ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::WalletTxnResponse**](walletTxnResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallet_txns

> crate::models::InlineResponse20013 get_wallet_txns(company_id, walletable_type, walletable_id, start_date, end_date, entry_side, offset, limit)
明細一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の明細一覧を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>amount : 明細金額</p> </li>  <li> <p>due_amount : 取引登録待ち金額</p> </li>  <li> <p>balance : 残高</p> </li>  <li> <p>entry_side</p>  <ul> <li>income : 入金</li>  <li>expense : 出金</li> </ul> </li>  <li> <p>walletable_type</p>  <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**walletable_type** | Option<**String**> | 口座区分 (銀行口座: bank_account, クレジットカード: credit_card, 現金: wallet) walletable_type、walletable_idは同時に指定が必要です。 |  |
**walletable_id** | Option<**i32**> | 口座ID walletable_type、walletable_idは同時に指定が必要です。 |  |
**start_date** | Option<**String**> | 取引日で絞込：開始日 (yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 取引日で絞込：終了日 (yyyy-mm-dd) |  |
**entry_side** | Option<**String**> | 入金／出金 (入金: income, 出金: expense) |  |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 20, 最小: 1, 最大: 100)  |  |

### Return type

[**crate::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

