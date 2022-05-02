# DealCreateResponseDealDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | 取引行ID | 
**account_item_id** | **i32** | 勘定科目ID | 
**tax_code** | **i32** | 税区分コード | 
**item_id** | Option<**i32**> | 品目ID | [optional]
**section_id** | Option<**i32**> | 部門ID | [optional]
**tag_ids** | Option<**Vec<i32>**> | メモタグID | [optional]
**segment_1_tag_id** | Option<**i64**> | セグメント１ID | [optional]
**segment_2_tag_id** | Option<**i64**> | セグメント２ID | [optional]
**segment_3_tag_id** | Option<**i64**> | セグメント３ID | [optional]
**amount** | **i64** | 取引金額 | 
**vat** | **i32** | 消費税額 | 
**description** | Option<**String**> | 備考 | [optional]
**entry_side** | **String** | 貸借（貸方: credit, 借方: debit） | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


