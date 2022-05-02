# \TaxesApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tax_code**](TaxesApi.md#get_tax_code) | **GET** /api/1/taxes/codes/{code} | 税区分コードの取得
[**get_tax_codes**](TaxesApi.md#get_tax_codes) | **GET** /api/1/taxes/codes | 税区分コード一覧の取得
[**get_taxes_companies**](TaxesApi.md#get_taxes_companies) | **GET** /api/1/taxes/companies/{company_id} | 税区分コード詳細一覧の取得



## get_tax_code

> crate::models::TaxResponse get_tax_code(code)
税区分コードの取得

 <h2 id=\"\">概要</h2>  <p>税区分コードを取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **i32** | 税区分コード | [required] |

### Return type

[**crate::models::TaxResponse**](taxResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tax_codes

> crate::models::InlineResponse2007 get_tax_codes()
税区分コード一覧の取得

 <h2 id=\"\">概要</h2>  <p>税区分コード一覧を取得する</p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_taxes_companies

> crate::models::InlineResponse2008 get_taxes_companies(company_id)
税区分コード詳細一覧の取得

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

