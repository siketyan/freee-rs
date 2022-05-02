# ApprovalFlowRouteResponseApprovalFlowRoute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 申請経路ID | 
**name** | Option<**String**> | 申請経路名 | [optional]
**description** | Option<**String**> | 申請経路の説明 | [optional]
**user_id** | Option<**i32**> | 更新したユーザーのユーザーID | [optional]
**definition_system** | Option<**bool**> | システム作成の申請経路かどうか | [optional]
**first_step_id** | Option<**i32**> | 最初の承認ステップのID | [optional]
**usages** | Option<**Vec<String>**> | 申請種別（申請経路を使用できる申請種別を示します。例えば、ApprovalRequest の場合は、各種申請で使用できる申請経路です。） * `TxnApproval` - 仕訳承認 * `ExpenseApplication` - 経費精算 * `PaymentRequest` - 支払依頼 * `ApprovalRequest` - 各種申請 * `DocApproval` - 請求書等 (見積書・納品書・請求書・発注書) | [optional]
**request_form_ids** | **Vec<i32>** | 申請経路で利用できる申請フォームID配列 | 
**steps** | Option<[**Vec<crate::models::ApprovalFlowRouteResponseApprovalFlowRouteSteps>**](approvalFlowRouteResponse_approval_flow_route_steps.md)> | 承認ステップ（配列） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


