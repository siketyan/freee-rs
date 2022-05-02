# ApprovalFlowRouteResponseApprovalFlowRouteSteps

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 承認ステップID | 
**next_step_id** | Option<**i32**> | 次の承認ステップID | 
**resource_type** | **String** | 承認方法( predefined_user: メンバー指定 (1人), selected_user: 申請時にメンバー指定,unspecified: 指定なし, and_resource: メンバー指定 (複数、全員の承認), or_resource: メンバー指定 (複数、1人の承認), and_position: 役職指定 (複数、全員の承認), or_position: 役職指定 (複数、1人の承認) )  | 
**user_ids** | Option<**Vec<i32>**> | 承認者のユーザーID (配列) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


