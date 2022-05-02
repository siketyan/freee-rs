# \ExpenseApplicationLineTemplatesApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_expense_application_line_template**](ExpenseApplicationLineTemplatesApi.md#create_expense_application_line_template) | **POST** /api/1/expense_application_line_templates | 経費科目の作成
[**destroy_expense_application_line_template**](ExpenseApplicationLineTemplatesApi.md#destroy_expense_application_line_template) | **DELETE** /api/1/expense_application_line_templates/{id} | 経費科目の削除
[**get_expense_application_line_template**](ExpenseApplicationLineTemplatesApi.md#get_expense_application_line_template) | **GET** /api/1/expense_application_line_templates/{id} | 経費科目の取得
[**get_expense_application_line_templates**](ExpenseApplicationLineTemplatesApi.md#get_expense_application_line_templates) | **GET** /api/1/expense_application_line_templates | 経費科目一覧の取得
[**update_expense_application_line_template**](ExpenseApplicationLineTemplatesApi.md#update_expense_application_line_template) | **PUT** /api/1/expense_application_line_templates/{id} | 経費科目の更新



## create_expense_application_line_template

> crate::models::ExpenseApplicationLineTemplateResponse create_expense_application_line_template(expense_application_line_template_params)
経費科目の作成

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expense_application_line_template_params** | [**ExpenseApplicationLineTemplateParams**](ExpenseApplicationLineTemplateParams.md) | 経費科目の作成 | [required] |

### Return type

[**crate::models::ExpenseApplicationLineTemplateResponse**](expenseApplicationLineTemplateResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_expense_application_line_template

> destroy_expense_application_line_template(id, company_id)
経費科目の削除

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 経費科目ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_expense_application_line_template

> crate::models::ExpenseApplicationLineTemplateResponse get_expense_application_line_template(id, company_id)
経費科目の取得

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 経費科目ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::ExpenseApplicationLineTemplateResponse**](expenseApplicationLineTemplateResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_expense_application_line_templates

> crate::models::InlineResponse20015 get_expense_application_line_templates(company_id, offset, limit)
経費科目一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の経費科目一覧を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 20, 最小: 1, 最大: 100) |  |

### Return type

[**crate::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_expense_application_line_template

> crate::models::ExpenseApplicationLineTemplateResponse update_expense_application_line_template(id, expense_application_line_template_params)
経費科目の更新

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 経費科目ID | [required] |
**expense_application_line_template_params** | [**ExpenseApplicationLineTemplateParams**](ExpenseApplicationLineTemplateParams.md) | 経費科目の更新 | [required] |

### Return type

[**crate::models::ExpenseApplicationLineTemplateResponse**](expenseApplicationLineTemplateResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

