# \UsersApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_users**](UsersApi.md#get_users) | **GET** /api/1/users | 事業所に所属するユーザー一覧の取得
[**get_users_capabilities**](UsersApi.md#get_users_capabilities) | **GET** /api/1/users/capabilities | ログインユーザーの権限の取得
[**get_users_me**](UsersApi.md#get_users_me) | **GET** /api/1/users/me | ログインユーザー情報の取得
[**update_user**](UsersApi.md#update_user) | **PUT** /api/1/users/me | ユーザー情報の更新



## get_users

> crate::models::InlineResponse2004 get_users(company_id, limit)
事業所に所属するユーザー一覧の取得

 <h2 id=\"\">概要</h2>  <p>事業所に所属するユーザーの一覧を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 3000) |  |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_capabilities

> crate::models::InlineResponse2005 get_users_capabilities(company_id)
ログインユーザーの権限の取得

 <h2 id=\"\">概要</h2>  <p>ユーザーの権限情報を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_me

> crate::models::MeResponse get_users_me(companies, advisor)
ログインユーザー情報の取得

 <h2 id=\"\">概要</h2>  <p>ユーザーの情報を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**companies** | Option<**bool**> | 取得情報にユーザーが所属する事業所一覧を含める |  |
**advisor** | Option<**bool**> | 取得情報に事業がアドバイザー事象所の場合は事業所毎の一意なプロフィールIDを含める |  |

### Return type

[**crate::models::MeResponse**](meResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::UserResponse update_user(user_params)
ユーザー情報の更新

 <h2 id=\"\">概要</h2>  <p>ユーザー情報を更新する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_params** | Option<[**UserParams**](UserParams.md)> | ユーザー情報の更新 |  |

### Return type

[**crate::models::UserResponse**](userResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

