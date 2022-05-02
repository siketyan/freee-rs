# \TransfersApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_transfer**](TransfersApi.md#create_transfer) | **POST** /api/1/transfers | 取引（振替）の作成
[**destroy_transfer**](TransfersApi.md#destroy_transfer) | **DELETE** /api/1/transfers/{id} | 取引（振替）の削除する
[**get_transfer**](TransfersApi.md#get_transfer) | **GET** /api/1/transfers/{id} | 取引（振替）の取得
[**get_transfers**](TransfersApi.md#get_transfers) | **GET** /api/1/transfers | 取引（振替）一覧の取得
[**update_transfer**](TransfersApi.md#update_transfer) | **PUT** /api/1/transfers/{id} | 取引（振替）の更新



## create_transfer

> crate::models::TransferResponse create_transfer(transfer_params)
取引（振替）の作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所の取引（振替）を作成する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>amount : 振替金額</p> </li>  <li> <p>from_walletable_type, to_walletable_type</p>  <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_params** | Option<[**TransferParams**](TransferParams.md)> | 取引（振替）の作成 |  |

### Return type

[**crate::models::TransferResponse**](transferResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_transfer

> destroy_transfer(id, company_id)
取引（振替）の削除する

 <h2 id=\"\">概要</h2>  <p>指定した事業所の取引（振替）を削除する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 取引(振替)ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transfer

> crate::models::TransferResponse get_transfer(id, company_id)
取引（振替）の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の取引（振替）を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>amount : 振替金額</p> </li>  <li> <p>from_walletable_type, to_walletable_type</p>  <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 取引(振替)ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::TransferResponse**](transferResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transfers

> crate::models::InlineResponse20012 get_transfers(company_id, start_date, end_date, offset, limit)
取引（振替）一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の取引（振替）一覧を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>amount : 振替金額</p> </li>  <li> <p>from_walletable_type, to_walletable_type</p>  <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**start_date** | Option<**String**> | 振替日で絞込：開始日 (yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 振替日で絞込：終了日 (yyyy-mm-dd) |  |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 20, 最小: 1, 最大: 100)  |  |

### Return type

[**crate::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transfer

> crate::models::TransferResponse update_transfer(id, transfer_params)
取引（振替）の更新

 <h2 id=\"\">概要</h2>  <p>指定した事業所の取引（振替）を更新する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>amount : 振替金額</p> </li>  <li> <p>from_walletable_type, to_walletable_type</p>  <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 取引(振替)ID | [required] |
**transfer_params** | [**TransferParams**](TransferParams.md) | 取引（振替）の更新 | [required] |

### Return type

[**crate::models::TransferResponse**](transferResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

