# ManualJournalDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | 貸借行ID | 
**entry_side** | **String** | 貸借(貸方: credit, 借方: debit) | 
**account_item_id** | **i32** | 勘定科目ID | 
**tax_code** | **i32** | 税区分コード | 
**partner_id** | Option<**i32**> | 取引先ID | 
**partner_name** | Option<**String**> | 取引先名 | 
**partner_code** | Option<**String**> | 取引先コード | [optional]
**partner_long_name** | Option<**String**> | 正式名称（255文字以内） | 
**item_id** | Option<**i32**> | 品目ID | 
**item_name** | Option<**String**> | 品目 | 
**section_id** | Option<**i32**> | 部門ID | 
**section_name** | Option<**String**> | 部門 | 
**tag_ids** | **Vec<i32>** |  | 
**tag_names** | **Vec<String>** |  | 
**segment_1_tag_id** | Option<**i64**> | セグメント１ID | [optional]
**segment_1_tag_name** | Option<**i32**> | セグメント１ID | [optional]
**segment_2_tag_id** | Option<**i64**> | セグメント２ID | [optional]
**segment_2_tag_name** | Option<**i32**> | セグメント２ | [optional]
**segment_3_tag_id** | Option<**i64**> | セグメント３ID | [optional]
**segment_3_tag_name** | Option<**i32**> | セグメント３ | [optional]
**amount** | **i32** | 金額（税込で指定してください） | 
**vat** | **i32** | 消費税額（指定しない場合は自動で計算されます） | 
**description** | **String** | 備考 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


