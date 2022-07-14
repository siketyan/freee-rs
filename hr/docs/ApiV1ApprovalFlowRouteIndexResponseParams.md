# ApiV1ApprovalFlowRouteIndexResponseParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 申請経路ID | 
**name** | Option<**String**> | 申請経路名 | [optional]
**description** | Option<**String**> | 申請経路の説明 | [optional]
**user_id** | Option<**i32**> | 更新したユーザーのユーザーID | [optional]
**definition_system** | Option<**bool**> | システム作成の申請経路かどうか | [optional]
**first_step_id** | Option<**i32**> | 最初の承認ステップのID | [optional]
**usages** | Option<**Vec<String>**> | 申請種別（申請経路を使用できる申請種別を示します。例えば、AttendanceWorkflow の場合は、勤怠申請で使用できる申請経路です。） - AttendanceWorkflow - 勤怠申請 - PersonalDataWorkflow - 身上変更申請 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


