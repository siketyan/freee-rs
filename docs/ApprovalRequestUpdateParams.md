# ApprovalRequestUpdateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**application_date** | Option<**String**> | 申請日 (yyyy-mm-dd)<br> 指定しない場合は当日の日付が登録されます。  | [optional]
**approval_flow_route_id** | **i32** | 申請経路ID | 
**approver_id** | Option<**i32**> | 承認者のユーザーID | [optional]
**draft** | **bool** | falseの時、in_progress:申請中で更新する。それ以外の時はdraft:下書きで更新する | 
**request_items** | [**Vec<crate::models::ApprovalRequestCreateParamsRequestItems>**](approvalRequestCreateParams_request_items.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


