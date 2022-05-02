# ApprovalFlowRoutesIndexResponseApprovalFlowRoutes

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
**request_form_ids** | Option<**Vec<i32>**> | 申請経路で利用できる申請フォームID配列 | [optional]
**default_route** | **bool** | 基本経路として設定されているかどうか<br><br> リクエストパラメータusageに下記のいずれかが指定され、かつ、基本経路の場合はtrueになります。 * `TxnApproval` - 仕訳承認 * `ExpenseApplication` - 経費精算 * `PaymentRequest` - 支払依頼 * `ApprovalRequest`(リクエストパラメータrequest_form_idを同時に指定) - 各種申請 * `DocApproval` - 請求書等 (見積書・納品書・請求書・発注書)  <a href=\"https://support.freee.co.jp/hc/ja/articles/900000507963\" target=\"_blank\">申請フォームの基本経路設定</a>  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


