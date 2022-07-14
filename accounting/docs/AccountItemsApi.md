# \AccountItemsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account_item**](AccountItemsApi.md#create_account_item) | **POST** /api/1/account_items | 勘定科目の作成
[**destroy_account_item**](AccountItemsApi.md#destroy_account_item) | **DELETE** /api/1/account_items/{id} | 勘定科目の削除
[**get_account_item**](AccountItemsApi.md#get_account_item) | **GET** /api/1/account_items/{id} | 勘定科目の詳細情報の取得
[**get_account_items**](AccountItemsApi.md#get_account_items) | **GET** /api/1/account_items | 勘定科目一覧の取得
[**update_account_item**](AccountItemsApi.md#update_account_item) | **PUT** /api/1/account_items/{id} | 勘定科目の更新



## create_account_item

> crate::models::AccountItemResponse create_account_item(account_item_params)
勘定科目の作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所の勘定科目を作成する</p>  <h2 id=\"_2\">注意点</h2> <p>tax_nameは、api/1/taxes/companies/{company_id} で該当事業所の税区分の一覧を取得して、availableの値がtrue、かつ”name_ja”に”税率%”を含んでいない税区分を確認して、そのnameを指定して勘定科目の作成をしてください</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_item_params** | [**AccountItemParams**](AccountItemParams.md) | 勘定科目の作成 | [required] |

### Return type

[**crate::models::AccountItemResponse**](accountItemResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_account_item

> destroy_account_item(id, company_id)
勘定科目の削除

 <h2 id=\"\">概要</h2>  <p>指定した勘定科目を削除する</p> <h2 id=\"\">注意点</h2> <ul> <li>削除できる勘定科目は、追加で作成したカスタム勘定項目のみです。</li> <li>デフォルトで存在する勘定科目や口座の勘定科目は削除できません。</li></ul>

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


## get_account_item

> crate::models::AccountItemResponse get_account_item(company_id, id)
勘定科目の詳細情報の取得

 <h2 id=\"\">概要</h2>  <p>指定した勘定科目の詳細を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** | 勘定科目ID | [required] |

### Return type

[**crate::models::AccountItemResponse**](accountItemResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_items

> crate::models::AccountItemsResponse get_account_items(company_id, base_date)
勘定科目一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の勘定科目一覧を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li>default_tax_id : デフォルト設定がされている税区分ID</li>  <li>default_tax_code : リクエストした日時を基準とした税区分コード</li> </ul>  <h2 id=\"_3\">注意点</h2> <p>default_tax_code は勘定科目作成・更新時に利用するものではありません</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**base_date** | Option<**String**> | 基準日:指定した場合、勘定科目に紐づく税区分(default_tax_code)が、基準日の税率に基づいて返ります。 |  |

### Return type

[**crate::models::AccountItemsResponse**](accountItemsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_account_item

> crate::models::AccountItemResponse update_account_item(id, account_item_params)
勘定科目の更新

 <h2 id=\"\">概要</h2>  <p>勘定科目を更新する</p>  <h2 id=\"_2\">注意点</h2> <p>tax_codeは、api/1/taxes/companies/{company_id} で該当事業所の税区分の一覧を取得して、availableの値がtrue、かつ”name_ja”に”税率%”を含んでいない税区分を確認して、そのcodeを指定して勘定科目の更新をしてください</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**account_item_params** | [**AccountItemParams**](AccountItemParams.md) | 勘定科目の更新 | [required] |

### Return type

[**crate::models::AccountItemResponse**](accountItemResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

