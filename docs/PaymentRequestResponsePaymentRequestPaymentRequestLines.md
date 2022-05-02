# PaymentRequestResponsePaymentRequestPaymentRequestLines

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | 支払依頼の項目行ID | 
**line_type** | **String** | 行の種類 (deal_line: 支払依頼, withholding_tax: 源泉徴収税) | 
**description** | **String** | 内容 | 
**amount** | **i32** | 金額 | 
**account_item_id** | Option<**i32**> | 勘定科目ID | 
**tax_code** | Option<**i32**> | 税区分コード | 
**item_id** | Option<**i32**> | 品目ID | 
**section_id** | Option<**i32**> | 部門ID | 
**tag_ids** | **Vec<i32>** | メモタグID | 
**segment_1_tag_id** | Option<**i64**> | セグメント１ID。セグメント１が使用可能なプランの時のみレスポンスに含まれます。 | [optional]
**segment_2_tag_id** | Option<**i64**> | セグメント２ID。セグメント２が使用可能なプランの時のみレスポンスに含まれます。 | [optional]
**segment_3_tag_id** | Option<**i64**> | セグメント３ID。セグメント３が使用可能なプランの時のみレスポンスに含まれます。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


