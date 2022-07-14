# ApiV1EmployeesTimeClocksControllerCreateBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | (required) | 
**_type** | **String** | 打刻種別（required）['clock_in','break_begin','break_end','clock_out']の何れか | 
**base_date** | Option<[**String**](string.md)> | 打刻日。打刻が日をまたぐ場合に、前日の日付を指定します。(YYYY-MM-DD)(例:2018-07-31) | [optional]
**datetime** | Option<**String**> | 打刻時刻。(YYYY-MM-DD&nbsp;HH:MM:SS)(例:2018-07-31&nbsp;08:00:00) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


