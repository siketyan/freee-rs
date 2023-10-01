# \FixedAssetsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_fixed_assets**](FixedAssetsApi.md#get_fixed_assets) | **GET** /api/1/fixed_assets | 固定資産一覧の取得



## get_fixed_assets

> crate::models::FixedAssetResponse get_fixed_assets(company_id, target_date, offset, limit)
固定資産一覧の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の固定資産一覧を取得する</p>  <h2 id=\"\">定義</h2> <ul>   <li>このAPIは法人エンタープライズに加入している事業所のみが利用できます。</li>   <li><p>target_date : 表示したい会計期間の開始年月日。開始年月日以外を指定した場合は、その日付が含まれる会計期間が対象となります。</p></li>   <li><p>depreciation_amount : 本年分の償却費合計</p></li>   <li><p>depreciation_method : 償却方法</p></li>   <li><p>depreciation_account_item_id : 減価償却に使う勘定科目</p></li>   <li><p>acquisition_cost : 取得価額</p></li>   <li><p>opening_balance : 期首残高</p></li>   <li><p>undepreciated_balance : 未償却残高。土地などの償却しない固定資産はnullが返ります。</p></li>   <li><p>opening_accumulated_depreciation : 期首減価償却累計額</p></li>   <li><p>closing_accumulated_depreciation : 期末減価償却累計額</p></li> </ul> <h2 id=\"\">注意点</h2> <ul>   <li><p>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</p></li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**target_date** | **String** | 表示したい会計期間の開始年月日。開始年月日以外を指定した場合は、その日付が含まれる会計期間が対象となります。 | [required] |
**offset** | Option<**i32**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 50, 最小: 1, 最大: 200) |  |

### Return type

[**crate::models::FixedAssetResponse**](fixedAssetResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

