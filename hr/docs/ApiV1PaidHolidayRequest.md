# ApiV1PaidHolidayRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID（必須） | 
**target_date** | [**String**](string.md) | 対象日（必須） | 
**holiday_type** | **String** | 取得単位（必須）（full:全休、half:半休、hour:時間休） | 
**start_at** | Option<**String**> | 取得予定開始時間 | [optional]
**end_at** | Option<**String**> | 取得予定終了時間 | [optional]
**comment** | Option<**String**> | 申請理由 | [optional]
**approval_flow_route_id** | **i32** | 申請経路ID（必須） | 
**approver_id** | Option<**i32**> | 承認者のユーザーID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


