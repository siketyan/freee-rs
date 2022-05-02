# \SegmentTagsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_segment_tag**](SegmentTagsApi.md#create_segment_tag) | **POST** /api/1/segments/{segment_id}/tags | セグメントの作成
[**destroy_segments_tag**](SegmentTagsApi.md#destroy_segments_tag) | **DELETE** /api/1/segments/{segment_id}/tags/{id} | セグメントタグの削除
[**get_segment_tags**](SegmentTagsApi.md#get_segment_tags) | **GET** /api/1/segments/{segment_id}/tags | セグメントタグ一覧の取得
[**update_segment_tag**](SegmentTagsApi.md#update_segment_tag) | **PUT** /api/1/segments/{segment_id}/tags/{id} | セグメントタグの更新



## create_segment_tag

> crate::models::SegmentTagResponse create_segment_tag(segment_id, segment_tag_params)
セグメントの作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所のセグメントタグを作成する</p>  <h2 id=\"\">注意点</h2>  <ul>  <li>本APIは法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li>  </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | **i32** | セグメントID（1,2,3のいずれか） 該当プラン以外で参照した場合にはエラーとなります。   1: 法人向けプロフェッショナル, 法人向けエンタープライズプラン   2,3: 法人向け エンタープライズプラン  | [required] |
**segment_tag_params** | [**SegmentTagParams**](SegmentTagParams.md) | セグメントタグの作成 | [required] |

### Return type

[**crate::models::SegmentTagResponse**](segmentTagResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_segments_tag

> destroy_segments_tag(segment_id, id, company_id)
セグメントタグの削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所のセグメントタグを削除する</p>  <h2 id=\"\">注意点</h2>  <ul>  <li>本APIは法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li>  </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | **i32** | セグメントID（1,2,3のいずれか） 該当プラン以外で参照した場合にはエラーとなります。   1: 法人向けプロフェッショナル, 法人向けエンタープライズプラン   2,3: 法人向け エンタープライズプラン  | [required] |
**id** | **i32** | セグメントタグID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_segment_tags

> crate::models::InlineResponse20016 get_segment_tags(company_id, segment_id, offset, limit)
セグメントタグ一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所のセグメントタグ一覧を取得する</p>  <h2 id=\"\">注意点</h2>  <ul>  <li>本APIは法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li>  </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**segment_id** | **i32** | セグメントID（1,2,3のいずれか） 該当プラン以外で参照した場合にはエラーとなります。   1: 法人向けプロフェッショナル, 法人向けエンタープライズプラン   2,3: 法人向け エンタープライズプラン  | [required] |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 20, 最小: 1, 最大: 500)  |  |

### Return type

[**crate::models::InlineResponse20016**](inline_response_200_16.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_segment_tag

> crate::models::SegmentTagResponse update_segment_tag(segment_id, id, segment_tag_params)
セグメントタグの更新

 <h2 id=\"\">概要</h2>  <p>指定した事業所のセグメントタグを更新する</p>  <h2 id=\"\">注意点</h2>  <ul>  <li>本APIは法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li>  </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | **i32** | セグメントID（1,2,3のいずれか） 該当プラン以外で参照した場合にはエラーとなります。   1: 法人向けプロフェッショナル, 法人向けエンタープライズプラン   2,3: 法人向け エンタープライズプラン  | [required] |
**id** | **i32** | セグメントタグID | [required] |
**segment_tag_params** | [**SegmentTagParams**](SegmentTagParams.md) | セグメントタグの作成 | [required] |

### Return type

[**crate::models::SegmentTagResponse**](segmentTagResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

