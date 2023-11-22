# ExpenseApplicationResponseExpenseApplicationPurchaseLinesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | 経費申請の申請行ID | 
**transaction_date** | Option<**String**> | 発生日(yyyy-mm-dd) | [optional]
**receipt_id** | Option<**i32**> | ファイルボックス（証憑ファイル）ID | [optional]
**sub_receipt_ids** | Option<**Vec<i32>**> | 補足資料（配列）   receipt_id（証憑ファイル）を指定してください。 | [optional]
**expense_application_lines** | Option<[**Vec<crate::models::ExpenseApplicationsIndexResponseExpenseApplicationsInnerPurchaseLinesInnerExpenseApplicationLinesInner>**](expenseApplicationsIndexResponse_expense_applications_inner_purchase_lines_inner_expense_application_lines_inner.md)> | 明細行一覧（配列） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


