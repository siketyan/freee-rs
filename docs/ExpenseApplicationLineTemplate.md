# ExpenseApplicationLineTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 経費科目ID | 
**name** | **String** | 経費科目名 | 
**account_item_id** | Option<**i32**> | 勘定科目ID | [optional]
**account_item_name** | **String** | 勘定科目名 | 
**tax_code** | Option<**i32**> | 税区分コード | [optional]
**tax_name** | **String** | 税区分名 | 
**description** | Option<**String**> | 経費科目の説明 | [optional]
**line_description** | Option<**String**> | 内容の補足 | [optional]
**required_receipt** | Option<**bool**> | 添付ファイルの必須/任意 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


