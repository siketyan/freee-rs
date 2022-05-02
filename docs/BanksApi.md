# \BanksApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bank**](BanksApi.md#get_bank) | **GET** /api/1/banks/{id} | 連携サービスの取得
[**get_banks**](BanksApi.md#get_banks) | **GET** /api/1/banks | 連携サービス一覧の取得



## get_bank

> crate::models::BankResponse get_bank(id)
連携サービスの取得

 <h2 id=\"\">概要</h2>  <p>連携しているサービスを取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li>type <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 連携サービスID | [required] |

### Return type

[**crate::models::BankResponse**](bankResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_banks

> crate::models::InlineResponse20011 get_banks(offset, limit, _type)
連携サービス一覧の取得

 <h2 id=\"\">概要</h2>  <p>連携しているサービス一覧を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li>type <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 20, 最小: 1, 最大: 500) |  |
**_type** | Option<**String**> | サービス種別 |  |

### Return type

[**crate::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

