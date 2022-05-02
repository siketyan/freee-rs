# ManualJournal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 振替伝票ID | 
**company_id** | **i32** | 事業所ID | 
**issue_date** | **String** | 発生日 (yyyy-mm-dd) | 
**adjustment** | **bool** | 決算整理仕訳フラグ（falseまたは未指定の場合: 日常仕訳） | 
**txn_number** | Option<**String**> | 仕訳番号 | 
**details** | [**Vec<crate::models::ManualJournalDetails>**](manual_journal_details.md) | 貸借行一覧（配列）: 貸借合わせて100行まで登録できます。 | 
**receipt_ids** | Option<**Vec<i32>**> | 証憑ファイルID（ファイルボックスのファイルID） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


