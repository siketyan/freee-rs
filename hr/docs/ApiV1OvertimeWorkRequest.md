# ApiV1OvertimeWorkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID（必須） | 
**target_date** | [**String**](string.md) | 対象日（必須） | 
**start_at** | **String** | 取得予定開始時間（必須） | 
**end_at** | **String** | 取得予定終了時間（必須） | 
**comment** | Option<**String**> | 申請理由 | [optional]
**approval_flow_route_id** | **i32** | 申請経路ID（必須） | 
**approver_id** | Option<**i32**> | 承認者のユーザーID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


