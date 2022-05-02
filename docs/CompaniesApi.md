# \CompaniesApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_companies**](CompaniesApi.md#get_companies) | **GET** /api/1/companies | 事業所一覧の取得
[**get_company**](CompaniesApi.md#get_company) | **GET** /api/1/companies/{id} | 事業所の詳細情報の取得



## get_companies

> crate::models::CompanyIndexResponse get_companies()
事業所一覧の取得

 <h2 id=\"\">概要</h2>  <p>ユーザーが所属する事業所の一覧を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li>role <ul> <li>admin : 管理者</li> <li>simple_accounting : 一般</li> <li>self_only : 取引登録のみ</li> <li>read_only : 閲覧のみ</li> <li>workflow : 申請・承認</li> </ul> </li> </ul>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CompanyIndexResponse**](companyIndexResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company

> crate::models::CompanyResponse get_company(id, details, account_items, taxes, items, partners, sections, tags, walletables)
事業所の詳細情報の取得

 <h2 id=\"\">概要</h2>  <p>ユーザーが所属する事業所の詳細を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li>role <ul> <li>admin : 管理者</li> <li>simple_accounting : 一般</li> <li>self_only : 取引登録のみ</li> <li>read_only : 閲覧のみ</li> <li>workflow : 申請・承認</li> </ul> </li> </ul>  <h2 id=\"_3\">

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 事業所ID | [required] |
**details** | Option<**bool**> | 取得情報に勘定科目・税区分コード・税区分・品目・取引先・部門・メモタグ・口座の一覧を含める |  |
**account_items** | Option<**bool**> | 取得情報に勘定科目一覧を含める |  |
**taxes** | Option<**bool**> | 取得情報に税区分コード・税区分一覧を含める |  |
**items** | Option<**bool**> | 取得情報に品目一覧を含める |  |
**partners** | Option<**bool**> | 取得情報に取引先一覧を含める |  |
**sections** | Option<**bool**> | 取得情報に部門一覧を含める |  |
**tags** | Option<**bool**> | 取得情報にメモタグ一覧を含める |  |
**walletables** | Option<**bool**> | 取得情報に口座一覧を含める |  |

### Return type

[**crate::models::CompanyResponse**](companyResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

