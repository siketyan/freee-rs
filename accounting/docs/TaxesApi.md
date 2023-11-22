# \TaxesApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tax_code**](TaxesApi.md#get_tax_code) | **GET** /api/1/taxes/codes/{code} | 税区分の取得
[**get_tax_codes**](TaxesApi.md#get_tax_codes) | **GET** /api/1/taxes/codes | 税区分一覧の取得（廃止予定）
[**get_taxes_companies**](TaxesApi.md#get_taxes_companies) | **GET** /api/1/taxes/companies/{company_id} | 指定した事業所の税区分一覧の取得



## get_tax_code

> crate::models::TaxResponse get_tax_code(code)
税区分の取得

 <h2 id=\"\">概要</h2>  <p>税区分を取得する</p>

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

> crate::models::GetTaxCodes200Response get_tax_codes()
税区分一覧の取得（廃止予定）

 <h2 id=\"\">概要</h2>  <p>税区分一覧を取得する</p>  <h2 id=\"\">注意点</h2>  <p>このAPIは廃止予定のため非推奨です。api/1/taxes/companies/{company_id}（指定した事業所の税区分一覧の取得）をご利用ください。</p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetTaxCodes200Response**](get_tax_codes_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_taxes_companies

> crate::models::GetTaxesCompanies200Response get_taxes_companies(company_id, display_category, available)
指定した事業所の税区分一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の税区分一覧を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**display_category** | Option<**String**> | 税区分の表示カテゴリ（ tax_5: 5%表示の税区分、 tax_8: 8%表示の税区分、 tax_r8: 軽減税率8%表示の税区分、 tax_10: 10%表示の税区分、 tax_5_e80: インボイス経過措置5%表示80%控除の税区分、 tax_5_e50: インボイス経過措置5%表示50%控除の税区分、 tax_8_e80: インボイス経過措置8%表示80%控除の税区分、 tax_8_e50: インボイス経過措置8%表示50%控除の税区分、 tax_r8_e80: インボイス経過措置軽減税率8%表示80%控除の税区分、 tax_r8_e50: インボイス経過措置軽減税率8%表示50%控除の税区分、 tax_10_e80: インボイス経過措置10%表示80%控除の税区分、 tax_10_e50: インボイス経過措置10%表示50%控除の税区分） |  |
**available** | Option<**bool**> | 税区分の使用設定。true: 使用する、false: 使用しない |  |

### Return type

[**crate::models::GetTaxesCompanies200Response**](get_taxes_companies_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

