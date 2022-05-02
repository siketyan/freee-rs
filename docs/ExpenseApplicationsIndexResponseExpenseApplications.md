# ExpenseApplicationsIndexResponseExpenseApplications

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 経費申請ID | 
**company_id** | **i32** | 事業所ID | 
**title** | **String** | 申請タイトル | 
**issue_date** | **String** | 申請日 (yyyy-mm-dd) | 
**description** | Option<**String**> | 備考 | [optional]
**total_amount** | Option<**i32**> | 合計金額 | [optional]
**status** | **String** | 申請ステータス(draft:下書き, in_progress:申請中, approved:承認済, rejected:却下, feedback:差戻し) | 
**section_id** | Option<**i32**> | 部門ID | [optional]
**tag_ids** | Option<**Vec<i32>**> | メモタグID | [optional]
**expense_application_lines** | [**Vec<crate::models::ExpenseApplicationsIndexResponseExpenseApplicationLines>**](expenseApplicationsIndexResponse_expense_application_lines.md) | 経費申請の項目行一覧（配列） | 
**deal_id** | Option<**i32**> | 取引ID (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_idが表示されます) | 
**deal_status** | Option<**String**> | 取引ステータス (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_statusが表示されます settled:精算済み, unsettled:清算待ち) | 
**applicant_id** | **i32** | 申請者のユーザーID | 
**application_number** | **String** | 申請No. | 
**current_step_id** | Option<**i32**> | 現在承認ステップID | [optional]
**current_round** | Option<**i32**> | 現在のround。差し戻し等により申請がstepの最初からやり直しになるとroundの値が増えます。 | [optional]
**segment_1_tag_id** | Option<**i64**> | セグメント１ID | [optional]
**segment_2_tag_id** | Option<**i64**> | セグメント２ID | [optional]
**segment_3_tag_id** | Option<**i64**> | セグメント３ID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


