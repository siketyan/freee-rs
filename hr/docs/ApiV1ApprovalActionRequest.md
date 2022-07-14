# ApiV1ApprovalActionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**approval_action** | **String** | 申請操作。（approve:承認、cancel:取り消し、feedback:差戻し、force_feedback:承認取り消し） | 
**target_round** | Option<**i32**> | 対象round。差戻し等により申請がstepの最初からやり直しになるとroundの値が増えます。取得APIレスポンス.current_roundを送信してください。 | 
**target_step_id** | Option<**i32**> | 対象承認ステップID。取得APIレスポンス.current_step_idを送信してください。 | 
**next_approver_id** | Option<**i32**> | 次のステップの承認者のユーザーID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


