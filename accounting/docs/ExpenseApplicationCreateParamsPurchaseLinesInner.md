# ExpenseApplicationCreateParamsPurchaseLinesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**receipt_id** | Option<**i32**> | ファイルボックス（証憑ファイル）ID | [optional]
**transaction_date** | **String** | 発生日(yyyy-mm-dd) | 
**sub_receipt_ids** | Option<**Vec<i32>**> | 補足資料（配列）   receipt_id（証憑ファイル）を指定してください。   receipt_id（証憑ファイル）は5個まで指定できます | [optional]
**expense_application_lines** | Option<[**Vec<crate::models::ExpenseApplicationCreateParamsPurchaseLinesInnerExpenseApplicationLinesInner>**](expenseApplicationCreateParams_purchase_lines_inner_expense_application_lines_inner.md)> | 明細行一覧（配列） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


