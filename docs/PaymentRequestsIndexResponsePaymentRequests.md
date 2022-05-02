# PaymentRequestsIndexResponsePaymentRequests

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 支払依頼ID | 
**company_id** | **i32** | 事業所ID | 
**title** | **String** | 申請タイトル | 
**application_date** | **String** | 申請日 (yyyy-mm-dd) | 
**total_amount** | **i32** | 合計金額 | 
**status** | **String** | 申請ステータス(draft:下書き, in_progress:申請中, approved:承認済, rejected:却下, feedback:差戻し) | 
**deal_id** | Option<**i32**> | 取引ID (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_idが表示されます) | [optional]
**deal_status** | Option<**String**> | 取引ステータス (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_statusが表示されます settled:支払済み, unsettled:支払待ち) | [optional]
**applicant_id** | **i32** | 申請者のユーザーID | 
**approvers** | [**Vec<crate::models::ExpenseApplicationResponseExpenseApplicationApprovers>**](expenseApplicationResponse_expense_application_approvers.md) | 承認者（配列）   承認ステップのresource_typeがunspecified (指定なし)の場合はapproversはレスポンスに含まれません。   しかし、resource_typeがunspecifiedの承認ステップにおいて誰かが承認・却下・差し戻しのいずれかのアクションを取った後は、   approversはレスポンスに含まれるようになります。   その場合approversにはアクションを行ったステップのIDとアクションを行ったユーザーのIDが含まれます。 | 
**application_number** | **String** | 申請No. | 
**current_step_id** | Option<**i32**> | 現在承認ステップID | 
**current_round** | **i32** | 現在のround。差し戻し等により申請がstepの最初からやり直しになるとroundの値が増えます。 | 
**document_code** | **String** | 請求書番号 | 
**issue_date** | **String** | 発生日 (yyyy-mm-dd) | 
**payment_date** | Option<**String**> | 支払期限 (yyyy-mm-dd) | 
**payment_method** | **String** | 支払方法(none: 指定なし, domestic_bank_transfer: 国内振込, abroad_bank_transfer: 国外振込, account_transfer: 口座振替, credit_card: クレジットカード) | 
**partner_id** | Option<**i32**> | 取引先ID | 
**partner_code** | Option<**String**> | 取引先コード | 
**partner_name** | Option<**String**> | 取引先名 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


