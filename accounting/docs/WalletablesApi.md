# \WalletablesApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_walletable**](WalletablesApi.md#create_walletable) | **POST** /api/1/walletables | 口座の作成
[**destroy_walletable**](WalletablesApi.md#destroy_walletable) | **DELETE** /api/1/walletables/{type}/{id} | 口座の削除
[**get_walletable**](WalletablesApi.md#get_walletable) | **GET** /api/1/walletables/{type}/{id} | 口座の取得
[**get_walletables**](WalletablesApi.md#get_walletables) | **GET** /api/1/walletables | 口座一覧の取得
[**update_walletable**](WalletablesApi.md#update_walletable) | **PUT** /api/1/walletables/{type}/{id} | 口座の更新



## create_walletable

> crate::models::WalletableCreateResponse create_walletable(walletable_create_params)
口座の作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所の口座を作成する</p>  <h2 id=\"\">注意点</h2> <ul><li>同期に対応した口座はこのAPIでは作成できません</li></ul>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>type</p>  <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li>  <li> <p>name : 口座名</p> </li>  <li> <p>bank_id : 連携サービスID</p> </li>  <li> <p>is_asset : type:wallet指定時に口座を資産口座とするか負債口座とするか（true: 資産口座 (デフォルト), false: 負債口座）</p> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**walletable_create_params** | Option<[**WalletableCreateParams**](WalletableCreateParams.md)> | 口座の作成 |  |

### Return type

[**crate::models::WalletableCreateResponse**](walletableCreateResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_walletable

> destroy_walletable(id, r#type, company_id)
口座の削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所の口座を削除する</p>  <h2 id=\"\">注意点</h2> <ul> <li>削除を実行するには、当該口座に関連する仕訳データを事前に削除する必要があります。</li> <li>当該口座に仕訳が残っていないか確認するには、レポートの「仕訳帳」等を参照し、必要に応じて、「取引」や「口座振替」も削除します。</li>  </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 口座ID | [required] |
**r#type** | **String** | 口座種別（bank_account : 銀行口座, credit_card : クレジットカード, wallet : その他の決済口座） | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_walletable

> crate::models::GetWalletable200Response get_walletable(id, r#type, company_id)
口座の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の口座を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li>type <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li>  <li>walletable_balance : 登録残高</li>  <li>last_balance : 同期残高</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 口座ID | [required] |
**r#type** | **String** | 口座種別（bank_account : 銀行口座, credit_card : クレジットカード, wallet : その他の決済口座） | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::GetWalletable200Response**](get_walletable_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_walletables

> crate::models::GetWalletables200Response get_walletables(company_id, with_balance, r#type)
口座一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の口座一覧を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li>type <ul> <li>bank_account : 銀行口座</li>  <li>credit_card : クレジットカード</li>  <li>wallet : その他の決済口座</li> </ul> </li>  <li>walletable_balance : 登録残高</li>  <li>last_balance : 同期残高</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**with_balance** | Option<**bool**> | 残高情報を含める |  |
**r#type** | Option<**String**> | 口座種別（bank_account : 銀行口座, credit_card : クレジットカード, wallet : その他の決済口座） |  |

### Return type

[**crate::models::GetWalletables200Response**](get_walletables_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_walletable

> crate::models::GetWalletable200Response update_walletable(id, r#type, walletable_update_params)
口座の更新

 <h2 id=\"\">概要</h2>  <p>指定した事業所の口座を更新する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**r#type** | **String** | 口座種別（bank_account : 銀行口座, credit_card : クレジットカード, wallet : その他の決済口座） | [required] |
**walletable_update_params** | Option<[**WalletableUpdateParams**](WalletableUpdateParams.md)> | 口座の更新 |  |

### Return type

[**crate::models::GetWalletable200Response**](get_walletable_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

