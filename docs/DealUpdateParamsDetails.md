# DealUpdateParamsDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 取引行ID: 既存取引行を更新する場合に指定します。IDを指定しない取引行は、新規行として扱われ追加されます。また、detailsに含まれない既存の取引行は削除されます。更新後も残したい行は、必ず取引行IDを指定してdetailsに含めてください。 | [optional]
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


