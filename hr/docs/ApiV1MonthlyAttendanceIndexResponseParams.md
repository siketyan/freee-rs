# ApiV1MonthlyAttendanceIndexResponseParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 申請ID | 
**company_id** | **i32** | 事業所ID | 
**application_number** | **i32** | 申請No | 
**applicant_id** | **i32** | 申請者のユーザーID | 
**approver_ids** | Option<**Vec<i32>**> | 承認者のユーザーID配列<br> 次の場合、空配列になります。 - 指定なしの申請経路を利用した、申請ステータスが承認済み以外の申請 - 申請が差戻された | [optional]
**target_date** | [**String**](string.md) | 対象日 | 
**issue_date** | [**String**](string.md) | 申請日 | 
**status** | **String** | 申請ステータス。（draft:下書き、in_progress:申請中、approved:承認済、feedback:差戻し） | 
**passed_auto_check** | **bool** | 自動チェック結果 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


