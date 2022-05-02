# DealCreateParamsDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tax_code** | **i32** | 税区分コード | 
**account_item_id** | **i32** | 勘定科目ID | 
**amount** | **i64** | 取引金額（税込で指定してください） | 
**item_id** | Option<**i32**> | 品目ID | [optional]
**section_id** | Option<**i32**> | 部門ID | [optional]
**tag_ids** | Option<**Vec<i32>**> | メモタグID | [optional]
**segment_1_tag_id** | Option<**i64**> | セグメント１ID | [optional]
**segment_2_tag_id** | Option<**i64**> | セグメント２ID | [optional]
**segment_3_tag_id** | Option<**i64**> | セグメント３ID | [optional]
**description** | Option<**String**> | 備考 | [optional]
**vat** | Option<**i32**> | 消費税額（指定しない場合は自動で計算されます） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


