# \TagsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tag**](TagsApi.md#create_tag) | **POST** /api/1/tags | メモタグの作成
[**destroy_tag**](TagsApi.md#destroy_tag) | **DELETE** /api/1/tags/{id} | メモタグの削除
[**get_tag**](TagsApi.md#get_tag) | **GET** /api/1/tags/{id} | メモタグの詳細情報の取得
[**get_tags**](TagsApi.md#get_tags) | **GET** /api/1/tags | メモタグ一覧の取得
[**update_tag**](TagsApi.md#update_tag) | **PUT** /api/1/tags/{id} | メモタグの更新



## create_tag

> crate::models::TagResponse create_tag(tag_params)
メモタグの作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所のメモタグを作成する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_params** | [**TagParams**](TagParams.md) | メモタグの作成 | [required] |

### Return type

[**crate::models::TagResponse**](tagResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_tag

> destroy_tag(id, company_id)
メモタグの削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所のメモタグを削除する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | タグID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag

> crate::models::TagResponse get_tag(id, company_id)
メモタグの詳細情報の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所のメモタグを取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | タグID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::TagResponse**](tagResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> crate::models::InlineResponse200 get_tags(company_id, start_update_date, end_update_date, offset, limit)
メモタグ一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所のメモタグ一覧を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**start_update_date** | Option<**String**> | 更新日で絞り込み：開始日(yyyy-mm-dd) |  |
**end_update_date** | Option<**String**> | 更新日で絞り込み：終了日(yyyy-mm-dd) |  |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 3000) |  |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tag

> crate::models::TagResponse update_tag(id, tag_params)
メモタグの更新

 <h2 id=\"\">概要</h2>  <p>指定した事業所のメモタグを更新する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | メモタグID | [required] |
**tag_params** | Option<[**TagParams**](TagParams.md)> | メモタグの更新 |  |

### Return type

[**crate::models::TagResponse**](tagResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

