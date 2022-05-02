# \QuotationsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_quotation**](QuotationsApi.md#create_quotation) | **POST** /api/1/quotations | 見積書の作成
[**destroy_quotation**](QuotationsApi.md#destroy_quotation) | **DELETE** /api/1/quotations/{id} | 見積書の削除
[**get_quotation**](QuotationsApi.md#get_quotation) | **GET** /api/1/quotations/{id} | 見積書の取得
[**get_quotations**](QuotationsApi.md#get_quotations) | **GET** /api/1/quotations | 見積書一覧の取得
[**update_quotation**](QuotationsApi.md#update_quotation) | **PUT** /api/1/quotations/{id} | 見積書の更新



## create_quotation

> crate::models::QuotationResponse create_quotation(quotation_create_params)
見積書の作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所の見積書を作成する</p>  <h2 id=\"_1\">注意点</h2> <ul> <li> <p>partner_code, partner_idはどちらかの指定が必須です。ただし両方同時に指定することはできません。</p> </li> <li> <p>partner_codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。</p> </li> <li> <p>本APIでは見積内容(quotation_contents)は、最大100行までになります。</p> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quotation_create_params** | Option<[**QuotationCreateParams**](QuotationCreateParams.md)> | 見積書の作成 |  |

### Return type

[**crate::models::QuotationResponse**](quotationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_quotation

> destroy_quotation(id, company_id)
見積書の削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所の見積書を削除する</p>

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


## get_quotation

> crate::models::QuotationResponse get_quotation(company_id, id)
見積書の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の見積書詳細を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** | 見積書ID | [required] |

### Return type

[**crate::models::QuotationResponse**](quotationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quotations

> crate::models::QuotationIndexResponse get_quotations(company_id, partner_id, partner_code, start_issue_date, end_issue_date, quotation_number, description, quotation_status, offset, limit)
見積書一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の見積書一覧を取得する</p> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**partner_id** | Option<**i32**> | 取引先IDで絞込 |  |
**partner_code** | Option<**String**> | 取引先コードで絞込 |  |
**start_issue_date** | Option<**String**> | 見積日の開始日(yyyy-mm-dd) |  |
**end_issue_date** | Option<**String**> | 見積日の終了日(yyyy-mm-dd) |  |
**quotation_number** | Option<**String**> | 見積書番号 |  |
**description** | Option<**String**> | 概要 |  |
**quotation_status** | Option<**String**> | 見積書ステータス  (unsubmitted: 送付待ち, submitted: 送付済み, all: 全て) |  |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 20, 最大: 100)  |  |

### Return type

[**crate::models::QuotationIndexResponse**](quotationIndexResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_quotation

> crate::models::QuotationResponse update_quotation(id, quotation_update_params)
見積書の更新

 <h2 id=\"\">概要</h2>  <p>指定した事業所の見積書を更新する</p>  <h2 id=\"_1\">注意点</h2> <ul> <li> <p>partner_code, partner_idを両方同時に指定することはできません。</p> </li> <li> <p>partner_codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。</p> </li> <li> <p>本APIでは見積内容(quotation_contents)は、最大100行までになります。</p> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 見積書ID | [required] |
**quotation_update_params** | Option<[**QuotationUpdateParams**](QuotationUpdateParams.md)> | 見積書の更新 |  |

### Return type

[**crate::models::QuotationResponse**](quotationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

