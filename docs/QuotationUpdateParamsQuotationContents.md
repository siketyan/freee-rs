# QuotationUpdateParamsQuotationContents

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 見積内容ID | [optional]
**order** | **i32** | 順序 | 
**_type** | **String** | 行の種類 <ul> <li>normal、discountを指定する場合、account_item_id,tax_codeとunit_priceが必須となります。</li> <li>normalを指定した場合、qtyが必須となります。</li> </ul> | 
**qty** | Option<**f32**> | 数量 | [optional]
**unit** | Option<**String**> | 単位 | [optional]
**unit_price** | Option<**f32**> | 単価 (tax_entry_method: inclusiveの場合は税込価格、tax_entry_method: exclusiveの場合は税抜価格となります) | [optional]
**vat** | Option<**i32**> | 消費税額 | [optional]
**description** | Option<**String**> | 備考 | [optional]
**account_item_id** | Option<**i32**> | 勘定科目ID | [optional]
**tax_code** | Option<**i32**> | 税区分コード | [optional]
**item_id** | Option<**i32**> | 品目ID | [optional]
**section_id** | Option<**i32**> | 部門ID | [optional]
**tag_ids** | Option<**Vec<i32>**> |  | [optional]
**segment_1_tag_id** | Option<**i64**> | セグメント１ID | [optional]
**segment_2_tag_id** | Option<**i64**> | セグメント２ID | [optional]
**segment_3_tag_id** | Option<**i64**> | セグメント３ID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


