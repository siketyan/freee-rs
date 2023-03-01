# ManualJournalCreateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**issue_date** | **String** | 発生日 (yyyy-mm-dd) | 
**adjustment** | Option<**bool**> | 決算整理仕訳フラグ（falseまたは未指定の場合: 日常仕訳） | [optional]
**details** | [**Vec<crate::models::ManualJournalCreateParamsDetailsInner>**](manualJournalCreateParams_details_inner.md) |  | 
**receipt_ids** | Option<**Vec<i32>**> | ファイルボックス（証憑ファイル）ID（配列） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


