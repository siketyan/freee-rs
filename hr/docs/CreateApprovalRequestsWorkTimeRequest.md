# CreateApprovalRequestsWorkTimeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID（必須） | 
**target_date** | [**String**](string.md) | 対象日（必須） | 
**clear_work_time** | Option<**bool**> | false: 勤務時間を修正する   true: 勤務時間を削除する  勤務時間を削除する場合は以下のパラメータは指定しないでください。 - clock_in_at - clock_out_at - lateness_mins - early_leaving_mins - break_records | [optional][default to false]
**clock_in_at** | Option<**String**> | 勤務開始時間（条件付き必須）   勤務時間を修正する場合は必須 | [optional]
**clock_out_at** | Option<**String**> | 勤務終了時間（条件付き必須）   勤務時間を修正する場合は必須 | [optional]
**lateness_mins** | Option<**i32**> | 遅刻分の時間（分単位） | [optional]
**early_leaving_mins** | Option<**i32**> | 早退分の時間（分単位） | [optional]
**break_records** | Option<[**Vec<crate::models::Items>**](items.md)> | 休憩時間のリスト | [optional]
**comment** | Option<**String**> | 申請理由 | [optional]
**approval_flow_route_id** | **i32** | 申請経路ID（必須） | 
**approver_id** | Option<**i32**> | 承認者のユーザーID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


