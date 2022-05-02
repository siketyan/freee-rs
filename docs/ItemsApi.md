# \ItemsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_item**](ItemsApi.md#create_item) | **POST** /api/1/items | 品目の作成
[**destroy_item**](ItemsApi.md#destroy_item) | **DELETE** /api/1/items/{id} | 品目の削除
[**get_item**](ItemsApi.md#get_item) | **GET** /api/1/items/{id} | 品目の取得
[**get_items**](ItemsApi.md#get_items) | **GET** /api/1/items | 品目一覧の取得
[**update_item**](ItemsApi.md#update_item) | **PUT** /api/1/items/{id} | 品目の更新



## create_item

> crate::models::ItemResponse create_item(item_params)
品目の作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所の品目を作成する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_params** | Option<[**ItemParams**](ItemParams.md)> | 品目の作成 |  |

### Return type

[**crate::models::ItemResponse**](itemResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_item

> destroy_item(id, company_id)
品目の削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所の品目を削除する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 品目ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item

> crate::models::ItemResponse get_item(company_id, id)
品目の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の品目を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** | 品目ID | [required] |

### Return type

[**crate::models::ItemResponse**](itemResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_items

> crate::models::InlineResponse2006 get_items(company_id, start_update_date, end_update_date, offset, limit)
品目一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の品目一覧を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**start_update_date** | Option<**String**> | 更新日で絞り込み：開始日(yyyy-mm-dd) |  |
**end_update_date** | Option<**String**> | 更新日で絞り込み：終了日(yyyy-mm-dd) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 3000) |  |

### Return type

[**crate::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_item

> crate::models::ItemResponse update_item(id, item_params)
品目の更新

 <h2 id=\"\">概要</h2>  <p>指定した事業所の品目を更新する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 品目ID | [required] |
**item_params** | Option<[**ItemParams**](ItemParams.md)> | 品目の更新 |  |

### Return type

[**crate::models::ItemResponse**](itemResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

