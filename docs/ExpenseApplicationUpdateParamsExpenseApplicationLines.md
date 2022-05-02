# ExpenseApplicationUpdateParamsExpenseApplicationLines

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 経費申請の項目行ID: 既存項目行を更新する場合に指定します。IDを指定しない項目行は、新規行として扱われ追加されます。また、expense_application_linesに含まれない既存の項目行は削除されます。更新後も残したい行は、必ず経費申請の項目行IDを指定してexpense_application_linesに含めてください。 | [optional]
**transaction_date** | Option<**String**> | 日付 (yyyy-mm-dd) | [optional]
**description** | Option<**String**> | 内容 (250文字以内) | [optional]
**amount** | Option<**i32**> | 金額 | [optional]
**expense_application_line_template_id** | Option<**i32**> | 経費科目ID | [optional]
**receipt_id** | Option<**i32**> | 証憑ファイルID（ファイルボックスのファイルID） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


