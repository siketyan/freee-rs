# \UsersApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_users**](UsersApi.md#get_users) | **GET** /api/1/users | 事業所に所属するユーザー一覧の取得
[**get_users_capabilities**](UsersApi.md#get_users_capabilities) | **GET** /api/1/users/capabilities | ログインユーザーの権限の取得
[**get_users_me**](UsersApi.md#get_users_me) | **GET** /api/1/users/me | ログインユーザーの取得
[**update_user**](UsersApi.md#update_user) | **PUT** /api/1/users/me | ログインユーザーの更新



## get_users

> crate::models::GetUsers200Response get_users(company_id, limit)
事業所に所属するユーザー一覧の取得

 <h2 id=\"\">概要</h2>  <p>事業所に所属するユーザー一覧を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 3000) |  |

### Return type

[**crate::models::GetUsers200Response**](get_users_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_capabilities

> crate::models::GetUsersCapabilities200Response get_users_capabilities(company_id)
ログインユーザーの権限の取得

 <h2 id=\"\">概要</h2>  <p>ログインユーザーの権限を取得する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |

### Return type

[**crate::models::GetUsersCapabilities200Response**](get_users_capabilities_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_me

> crate::models::MeResponse get_users_me(companies, advisor)
ログインユーザーの取得

 <h2 id=\"\">概要</h2>  <p>ログインユーザーを取得する</p>

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
ログインユーザーの更新

 <h2 id=\"\">概要</h2>  <p>ログインユーザーを更新する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_params** | Option<[**UserParams**](UserParams.md)> | ログインユーザーの更新 |  |

### Return type

[**crate::models::UserResponse**](userResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

