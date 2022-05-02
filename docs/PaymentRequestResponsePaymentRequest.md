# PaymentRequestResponsePaymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 支払依頼ID | 
**company_id** | **i32** | 事業所ID | 
**title** | **String** | 申請タイトル | 
**application_date** | **String** | 申請日 (yyyy-mm-dd) | 
**description** | **String** | 備考 | 
**total_amount** | **i32** | 合計金額 | 
**status** | **String** | 申請ステータス(draft:下書き, in_progress:申請中, approved:承認済, rejected:却下, feedback:差戻し) | 
**payment_request_lines** | [**Vec<crate::models::PaymentRequestResponsePaymentRequestPaymentRequestLines>**](paymentRequestResponse_payment_request_payment_request_lines.md) | 支払依頼の項目行一覧（配列） | 
**deal_id** | Option<**i32**> | 取引ID (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_idが表示されます) | 
**deal_status** | Option<**String**> | 取引ステータス (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_statusが表示されます settled:支払済み, unsettled:支払待ち) | 
**applicant_id** | **i32** | 申請者のユーザーID | 
**approvers** | [**Vec<crate::models::ExpenseApplicationResponseExpenseApplicationApprovers>**](expenseApplicationResponse_expense_application_approvers.md) | 承認者（配列）   承認ステップのresource_typeがunspecified (指定なし)の場合はapproversはレスポンスに含まれません。   しかし、resource_typeがunspecifiedの承認ステップにおいて誰かが承認・却下・差し戻しのいずれかのアクションを取った後は、   approversはレスポンスに含まれるようになります。   その場合approversにはアクションを行ったステップのIDとアクションを行ったユーザーのIDが含まれます。 | 
**application_number** | **String** | 申請No. | 
**approval_flow_route_id** | **i32** | 申請経路ID | 
**comments** | [**Vec<crate::models::ExpenseApplicationResponseExpenseApplicationComments>**](expenseApplicationResponse_expense_application_comments.md) | 支払依頼のコメント一覧（配列） | 
**approval_flow_logs** | [**Vec<crate::models::ExpenseApplicationResponseExpenseApplicationApprovalFlowLogs>**](expenseApplicationResponse_expense_application_approval_flow_logs.md) | 支払依頼の承認履歴（配列） | 
**current_step_id** | Option<**i32**> | 現在承認ステップID | 
**current_round** | **i32** | 現在のround。差し戻し等により申請がstepの最初からやり直しになるとroundの値が増えます。 | 
**document_code** | **String** | 請求書番号 | 
**receipt_ids** | **Vec<i32>** | 証憑ファイルID（ファイルボックスのファイルID） | 
**issue_date** | **String** | 発生日 (yyyy-mm-dd) | 
**payment_date** | Option<**String**> | 支払期限 (yyyy-mm-dd) | 
**payment_method** | **String** | 支払方法(none: 指定なし, domestic_bank_transfer: 国内振込, abroad_bank_transfer: 国外振込, account_transfer: 口座振替, credit_card: クレジットカード) | 
**partner_id** | Option<**i32**> | 取引先ID | 
**partner_code** | Option<**String**> | 取引先コード | [optional]
**partner_name** | Option<**String**> | 取引先名 | 
**bank_name** | **String** | 銀行名 | 
**bank_name_kana** | **String** | 銀行名（カナ） | 
**bank_code** | **String** | 銀行コード | 
**branch_name** | **String** | 支店名 | 
**branch_kana** | **String** | 支店名（カナ） | 
**branch_code** | **String** | 支店番号 | 
**account_type** | **String** | 口座種別(ordinary:普通、checking:当座、earmarked:納税準備預金、savings:貯蓄、other:その他) | 
**account_number** | **String** | 口座番号 | 
**account_name** | **String** | 受取人名（カナ） | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


