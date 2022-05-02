# \SectionsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_section**](SectionsApi.md#create_section) | **POST** /api/1/sections | 部門の作成
[**destroy_section**](SectionsApi.md#destroy_section) | **DELETE** /api/1/sections/{id} | 部門の削除
[**get_section**](SectionsApi.md#get_section) | **GET** /api/1/sections/{id} | 
[**get_sections**](SectionsApi.md#get_sections) | **GET** /api/1/sections | 部門一覧の取得
[**update_section**](SectionsApi.md#update_section) | **PUT** /api/1/sections/{id} | 部門の更新



## create_section

> crate::models::SectionResponse create_section(section_params)
部門の作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所の部門を作成する</p>  <h2 id=\"_2\">レスポンスの例</h2>  <pre><code>// プレミアムプラン（個人）、ベーシックプラン以上（法人） {   &quot;section&quot; : {     &quot;id&quot; : 102,     &quot;company_id&quot; : 1,     &quot;name&quot; : &quot;開発部門&quot;,     &quot;shortcut1&quot; : &quot;DEVELOPER&quot;,     &quot;shortcut2&quot; : &quot;123&quot;,     &quot;indent_count&quot;: 1,     &quot;parent_id&quot;: 101   } } // それ以外のプラン {   &quot;section&quot; : {     &quot;id&quot; : 102,     &quot;company_id&quot; : 1,     &quot;name&quot; : &quot;開発部門&quot;,     &quot;shortcut1&quot; : &quot;DEVELOPER&quot;,     &quot;shortcut2&quot; : &quot;123&quot;   } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_params** | Option<[**SectionParams**](SectionParams.md)> | 部門の作成 |  |

### Return type

[**crate::models::SectionResponse**](sectionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_section

> destroy_section(id, company_id)
部門の削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所の部門を削除する</p>

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


## get_section

> crate::models::SectionResponse get_section(id, company_id)


 <h2 id=\"\">概要</h2>  <p>指定した事業所の部門を参照する</p><h2 id=\"_2\">レスポンスの例</h2>  <pre><code>// プレミアムプラン（個人）、ベーシックプラン以上（法人） {   &quot;section&quot; : {     &quot;id&quot; : 102,     &quot;company_id&quot; : 1,     &quot;name&quot; : &quot;開発部門&quot;,     &quot;long_name&quot;: &quot;開発部門&quot;,     &quot;shortcut1&quot; : &quot;DEVELOPER&quot;,     &quot;shortcut2&quot; : &quot;123&quot;,     &quot;indent_count&quot;: 1,     &quot;parent_id&quot;: 101   } } // それ以外のプラン {   &quot;section&quot; : {     &quot;id&quot; : 102,     &quot;company_id&quot; : 1,     &quot;name&quot; : &quot;開発部門&quot;,     &quot;long_name&quot;: &quot;開発部門&quot;,     &quot;shortcut1&quot; : &quot;DEVELOPER&quot;,     &quot;shortcut2&quot; : &quot;123&quot;   } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 部門ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::SectionResponse**](sectionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sections

> crate::models::InlineResponse2001 get_sections(company_id)
部門一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の部門一覧を取得する</p>  <h2 id=\"_2\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/sections?company_id=1</p> </blockquote>  <pre><code>// プレミアムプラン（個人）、ベーシックプラン以上（法人） {   &quot;sections&quot; : [     {       &quot;id&quot; : 101,       &quot;company_id&quot; : 1,       &quot;name&quot; : &quot;開発部門&quot;,       &quot;long_name&quot;: &quot;開発部門&quot;,       &quot;shortcut1&quot; : &quot;DEVELOPER&quot;,       &quot;shortcut2&quot; : &quot;123&quot;,       &quot;indent_count&quot;: 1,       &quot;parent_id&quot;: 11     },     ...   ] } // それ以外のプラン {   &quot;sections&quot; : [     {       &quot;id&quot; : 101,       &quot;company_id&quot; : 1,       &quot;name&quot; : &quot;開発部門&quot;,       &quot;long_name&quot;: &quot;開発部門&quot;,       &quot;shortcut1&quot; : &quot;DEVELOPER&quot;,       &quot;shortcut2&quot; : &quot;123&quot;     },     ...   ] }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_section

> crate::models::SectionResponse update_section(id, section_params)
部門の更新

 <h2 id=\"\">概要</h2>  <p>指定した事業所の部門を更新する</p><h2 id=\"_2\">レスポンスの例</h2>  <pre><code>// プレミアムプラン（個人）、ベーシックプラン以上（法人） {   &quot;section&quot; : {     &quot;id&quot; : 102,     &quot;company_id&quot; : 1,     &quot;name&quot; : &quot;開発部門&quot;,     &quot;long_name&quot;: &quot;開発部門&quot;,     &quot;shortcut1&quot; : &quot;DEVELOPER&quot;,     &quot;shortcut2&quot; : &quot;123&quot;,     &quot;indent_count&quot;: 1,     &quot;parent_id&quot;: 101   } } // それ以外のプラン {   &quot;section&quot; : {     &quot;id&quot; : 102,     &quot;company_id&quot; : 1,     &quot;name&quot; : &quot;開発部門&quot;,     &quot;long_name&quot;: &quot;開発部門&quot;,     &quot;shortcut1&quot; : &quot;DEVELOPER&quot;,     &quot;shortcut2&quot; : &quot;123&quot;   } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**section_params** | Option<[**SectionParams**](SectionParams.md)> | 部門の更新 |  |

### Return type

[**crate::models::SectionResponse**](sectionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

