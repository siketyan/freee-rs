# \SelectablesApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_forms_selectables**](SelectablesApi.md#get_forms_selectables) | **GET** /api/1/forms/selectables | フォーム用選択項目情報の取得



## get_forms_selectables

> crate::models::SelectablesIndexResponse get_forms_selectables(company_id, includes)
フォーム用選択項目情報の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所のフォーム用選択項目情報を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**includes** | Option<**String**> | 取得する項目(項目: account_item) |  |

### Return type

[**crate::models::SelectablesIndexResponse**](selectablesIndexResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

