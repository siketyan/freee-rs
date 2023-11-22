# JournalsResponseJournals

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 受け付けID | 
**messages** | Option<**Vec<String>**> |  | [optional]
**company_id** | **i32** | 事業所ID | 
**download_type** | Option<**String**> | ダウンロード形式 | [optional]
**encoding** | Option<**String**> | 文字コード | [optional]
**start_date** | Option<**String**> | 取得開始日 (yyyy-mm-dd) | [optional]
**end_date** | Option<**String**> | 取得終了日 (yyyy-mm-dd) | [optional]
**visible_tags** | Option<**Vec<String>**> |  | [optional]
**visible_ids** | Option<**Vec<String>**> |  | [optional]
**status_url** | Option<**String**> | 仕訳帳のステータスの取得用URL | [optional]
**up_to_date** | Option<**bool**> | 集計結果が最新かどうか | [optional]
**up_to_date_reasons** | Option<[**Vec<crate::models::JournalsResponseJournalsUpToDateReasonsInner>**](journalsResponse_journals_up_to_date_reasons_inner.md)> | 集計が最新でない場合の要因情報 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


