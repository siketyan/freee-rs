# ApprovalRequestUpdateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**application_date** | Option<**String**> | 申請日 (yyyy-mm-dd)<br> 指定しない場合は当日の日付が登録されます。  | [optional]
**approval_flow_route_id** | **i32** | 申請経路ID | 
**approver_id** | Option<**i32**> | 承認者のユーザーID | [optional]
**draft** | **bool** | 各種申請のステータス<br> falseを指定した時は申請中（in_progress）で各種申請を更新します。<br> trueを指定した時は下書き（draft）で各種申請を更新します。  | 
**request_items** | [**Vec<crate::models::ApprovalRequestCreateParamsRequestItemsInner>**](approvalRequestCreateParams_request_items_inner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


