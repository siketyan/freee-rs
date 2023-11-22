# ExpenseApplicationUpdateParamsPurchaseLinesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 経費申請の申請行ID: 既存申請行を更新する場合に指定します。IDを指定しない申請行は、新規行として扱われ追加されます。また、purchase_linesに含まれない既存の申請行は削除されます。更新後も残したい行は、必ず経費申請の申請行IDを指定してpurchase_linesに含めてください。 | [optional]
**transaction_date** | **String** | 発生日(yyyy-mm-dd) | 
**receipt_id** | Option<**i32**> | ファイルボックス（証憑ファイル）ID | [optional]
**sub_receipt_ids** | Option<**Vec<i32>**> | 補足資料（配列）   receipt_id（証憑ファイル）を指定してください。   receipt_id（証憑ファイル）は5個まで指定できます | [optional]
**expense_application_lines** | Option<[**Vec<crate::models::ExpenseApplicationUpdateParamsPurchaseLinesInnerExpenseApplicationLinesInner>**](expenseApplicationUpdateParams_purchase_lines_inner_expense_application_lines_inner.md)> | 明細行一覧（配列） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


