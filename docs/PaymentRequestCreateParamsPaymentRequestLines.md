# PaymentRequestCreateParamsPaymentRequestLines

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**line_type** | Option<**String**> | '行の種類 (deal_line: 支払依頼, withholding_tax: 源泉徴収税)'<br> 'デフォルトは deal_line: 支払依頼 です'  | [optional]
**description** | Option<**String**> | 内容 | [optional]
**amount** | **i32** | 金額 | 
**account_item_id** | Option<**i32**> | 勘定科目ID | [optional]
**tax_code** | Option<**i32**> | 税区分コード<br> 勘定科目IDを指定する場合は必須です。  | [optional]
**item_id** | Option<**i32**> | 品目ID | [optional]
**section_id** | Option<**i32**> | 部門ID | [optional]
**tag_ids** | Option<**Vec<i32>**> | メモタグID | [optional]
**segment_1_tag_id** | Option<**i64**> | セグメント１ID<br> セグメントタグ一覧APIを利用して取得してください。<br> <a href=\"https://support.freee.co.jp/hc/ja/articles/360020679611\" target=\"_blank\">セグメント（分析用タグ）の設定</a><br>  | [optional]
**segment_2_tag_id** | Option<**i64**> | セグメント２ID(法人向けエンタープライズプラン)<br> セグメントタグ一覧APIを利用して取得してください。<br> <a href=\"https://support.freee.co.jp/hc/ja/articles/360020679611\" target=\"_blank\">セグメント（分析用タグ）の設定</a><br>  | [optional]
**segment_3_tag_id** | Option<**i64**> | セグメント３ID(法人向けエンタープライズプラン)<br> セグメントタグ一覧APIを利用して取得してください。<br> <a href=\"https://support.freee.co.jp/hc/ja/articles/360020679611\" target=\"_blank\">セグメント（分析用タグ）の設定</a><br>  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


