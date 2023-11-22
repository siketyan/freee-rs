# \GeneralLedgersApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_general_ledgers**](GeneralLedgersApi.md#get_general_ledgers) | **GET** /api/1/reports/general_ledgers | 総勘定元帳一覧の取得（β版）



## get_general_ledgers

> crate::models::GeneralLedgersResponse get_general_ledgers(company_id, start_date, end_date, account_item_name, tax_name, tax_rate, adjustment, cost_allocation, partner_name, item_name, section_name, tag_name, segment_tag_1_name, segment_tag_2_name, segment_tag_3_name, approval_flow_status)
総勘定元帳一覧の取得（β版）

<h2 id=\"\">概要</h2> <p>指定した事業所の総勘定元帳一覧を取得する</p> <br> ※このAPIはβ版として提供しています。 <ul>   <li>このAPIは法人プロフェッショナル・法人エンタープライズに加入している事業所のみが利用できます。</li>   <li>利用状況によって事前の告知なく提供プラン・コール数の上限を変更する可能性があります。</li>   <li>他エンドポイントと比べてレスポンスタイムが遅い場合があります。</li> </ul> <h2 id=\"\">注意点</h2> <ul> <li>segment_tag_2_name、segment_tag_3_nameはエンタープライズプランのみ使用できます。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**start_date** | **String** | 期間で絞込：開始日 (yyyy-mm-dd) | [required] |
**end_date** | **String** | 期間で絞込：終了日 (yyyy-mm-dd) | [required] |
**account_item_name** | Option<**String**> | 勘定科目名で絞込 |  |
**tax_name** | Option<**String**> | 税区分名で絞込 |  |
**tax_rate** | Option<**String**> | 税率で絞込 |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ：only, 決算整理仕訳以外：without）。指定されない場合、決算整理仕訳を含む金額が返却されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**partner_name** | Option<**String**> | 取引先で絞込（未選択を指定すると、取引先が未選択で絞り込めます） |  |
**item_name** | Option<**String**> | 品目で絞込（未選択を指定すると、品目が未選択で絞り込めます） |  |
**section_name** | Option<**String**> | 部門で絞込（未選択を指定すると、部門が未選択で絞り込めます） |  |
**tag_name** | Option<**String**> | メモタグで絞込<br> 取引数が多すぎる場合はタイムアウトする場合があります。<br> その場合はWeb画面よりPDF/CSV出力をご利用ください。  |  |
**segment_tag_1_name** | Option<**String**> | セグメント1タグで絞込（未選択を指定すると、セグメント1タグが未選択で絞り込めます） |  |
**segment_tag_2_name** | Option<**String**> | セグメント2タグで絞込（未選択を指定すると、セグメント2タグが未選択で絞り込めます） |  |
**segment_tag_3_name** | Option<**String**> | セグメント3タグで絞込（未選択を指定すると、セグメント3タグが未選択で絞り込めます） |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::GeneralLedgersResponse**](generalLedgersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

