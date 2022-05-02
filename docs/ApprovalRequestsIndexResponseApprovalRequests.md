# ApprovalRequestsIndexResponseApprovalRequests

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 各種申請ID | 
**company_id** | **i32** | 事業所ID | 
**application_date** | **String** | 申請日 (yyyy-mm-dd) | 
**title** | **String** | 申請タイトル | 
**applicant_id** | **i32** | 申請者のユーザーID | 
**application_number** | **String** | 申請No. | 
**status** | **String** | 申請ステータス(draft:下書き, in_progress:申請中, approved:承認済, rejected:却下, feedback:差戻し) | 
**request_items** | [**Vec<crate::models::ApprovalRequestsIndexResponseRequestItems>**](approvalRequestsIndexResponse_request_items.md) | 各種申請の項目一覧（配列） | 
**form_id** | **i32** | 申請フォームID | 
**current_step_id** | Option<**i32**> | 現在承認ステップID | 
**current_round** | **i32** | 現在のround。差し戻し等により申請がstepの最初からやり直しになるとroundの値が増えます。 | 
**deal_id** | Option<**i32**> | 取引ID (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_idが表示されます) | 
**manual_journal_id** | Option<**i32**> | 振替伝票のID (申請ステータス:statusがapprovedで、関連する振替伝票が存在する時のみmanual_journal_idが表示されます)  <a href=\"https://support.freee.co.jp/hc/ja/articles/115003827683-#5\" target=\"_blank\">承認された各種申請から支払依頼等を作成する</a>  | 
**deal_status** | Option<**String**> | 取引ステータス (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_statusが表示されます settled:決済済み, unsettled:未決済) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


