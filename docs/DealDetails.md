# DealDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | +更新の明細行ID | 
**entry_side** | **String** | 貸借(貸方: credit, 借方: debit) | 
**account_item_id** | **i32** | 勘定科目ID | 
**tax_code** | **i32** | 税区分コード | 
**item_id** | Option<**i32**> | 品目ID | [optional]
**section_id** | Option<**i32**> | 部門ID | [optional]
**tag_ids** | **Vec<i32>** |  | 
**segment_1_tag_id** | Option<**i64**> | セグメント１ID | [optional]
**segment_2_tag_id** | Option<**i64**> | セグメント２ID | [optional]
**segment_3_tag_id** | Option<**i64**> | セグメント３ID | [optional]
**amount** | **i64** | 金額（税込で指定してください） | 
**vat** | **i32** | 消費税額（指定しない場合は自動で計算されます） | 
**description** | Option<**String**> | 備考 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


