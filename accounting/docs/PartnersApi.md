# \PartnersApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_partner**](PartnersApi.md#create_partner) | **POST** /api/1/partners | 取引先の作成
[**destroy_partner**](PartnersApi.md#destroy_partner) | **DELETE** /api/1/partners/{id} | 取引先の削除
[**get_partner**](PartnersApi.md#get_partner) | **GET** /api/1/partners/{id} | 取引先の取得
[**get_partners**](PartnersApi.md#get_partners) | **GET** /api/1/partners | 取引先一覧の取得
[**update_partner**](PartnersApi.md#update_partner) | **PUT** /api/1/partners/{id} | 取引先の更新
[**update_partner_by_code**](PartnersApi.md#update_partner_by_code) | **PUT** /api/1/partners/code/{code} | 取引先の更新



## create_partner

> crate::models::PartnerResponse create_partner(partner_create_params)
取引先の作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所の取引先を作成する</p> <ul> <li>codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。</li> <li>取引先コードの利用を有効にしている場合は、codeの指定は必須です。</li> <li>振込元口座ID（payer_walletable_id）, 振込手数料負担（transfer_fee_handling_side）, 支払期日設定（payment_term_attributes, 請求の入金期日設定（invoice_payment_term_attributes）は法人向けのプロフェッショナルプラン以上で利用可能です。</li></ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partner_create_params** | [**PartnerCreateParams**](PartnerCreateParams.md) | 取引先の作成 | [required] |

### Return type

[**crate::models::PartnerResponse**](partnerResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_partner

> destroy_partner(id, company_id)
取引先の削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所の取引先を削除する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 取引先ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_partner

> crate::models::PartnerResponse get_partner(id, company_id)
取引先の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の取引先を取得する</p> <ul> <li>振込元口座ID（payer_walletable_id）, 振込手数料負担（transfer_fee_handling_side）, 支払期日設定（payment_term_attributes, 請求の入金期日設定（invoice_payment_term_attributes）は法人向けのプロフェッショナルプラン以上で利用可能です。</li></ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 取引先ID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::PartnerResponse**](partnerResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_partners

> crate::models::PartnersResponse get_partners(company_id, start_update_date, end_update_date, offset, limit, keyword)
取引先一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の取引先一覧を取得する</p> <ul> <li>振込元口座ID（payer_walletable_id）, 振込手数料負担（transfer_fee_handling_side）は法人向けのプロフェッショナルプラン以上で利用可能です。</li></ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**start_update_date** | Option<**String**> | 更新日で絞り込み：開始日(yyyy-mm-dd) |  |
**end_update_date** | Option<**String**> | 更新日で絞り込み：終了日(yyyy-mm-dd) |  |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 3000) |  |
**keyword** | Option<**String**> | 検索キーワード：取引先名・正式名称・カナ名称に対するあいまい検索で一致、またはショートカットキー1・2のいずれかに完全一致 |  |

### Return type

[**crate::models::PartnersResponse**](partnersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_partner

> crate::models::PartnerResponse update_partner(id, partner_update_params)
取引先の更新

 <h2 id=\"\">概要</h2>  <p>指定した取引先の情報を更新する</p> <ul> <li>codeを指定、更新することはできません。</li> <li>振込元口座ID（payer_walletable_id）, 振込手数料負担（transfer_fee_handling_side）, 支払期日設定（payment_term_attributes, 請求の入金期日設定（invoice_payment_term_attributes）は法人向けのプロフェッショナルプラン以上で利用可能です。</li></ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 取引先ID | [required] |
**partner_update_params** | [**PartnerUpdateParams**](PartnerUpdateParams.md) | 取引先の更新 | [required] |

### Return type

[**crate::models::PartnerResponse**](partnerResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_partner_by_code

> crate::models::PartnerResponse update_partner_by_code(code, partner_update_params)
取引先の更新

 <h2 id=\"\">概要</h2>  <p>取引先コードをキーに、指定した取引先の情報を更新する</p> <ul> <li>このAPIを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。</li> <li>コードを日本語に設定している場合は、URLエンコードしてURLに含めるようにしてください。</li> <li>振込元口座ID（payer_walletable_id）, 振込手数料負担（transfer_fee_handling_side）, 支払期日設定（payment_term_attributes, 請求の入金期日設定（invoice_payment_term_attributes）は法人向けのプロフェッショナルプラン以上で利用可能です。</li></ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | 取引先コード | [required] |
**partner_update_params** | [**PartnerUpdateParams**](PartnerUpdateParams.md) | 取引先の更新 | [required] |

### Return type

[**crate::models::PartnerResponse**](partnerResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

