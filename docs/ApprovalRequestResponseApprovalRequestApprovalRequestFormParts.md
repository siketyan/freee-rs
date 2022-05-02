# ApprovalRequestResponseApprovalRequestApprovalRequestFormParts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 項目ID | 
**order** | Option<**i32**> | 順序 | [optional]
**_type** | Option<**String**> | 項目種別 (title: 申請タイトル, single_line: 自由記述形式 1行, multi_line: 自由記述形式 複数行, select: プルダウン, date: 日付, amount: 金額, receipt: 添付ファイル, section: 部門ID, partner: 取引先ID, ninja_sign_document: 契約書（freeeサイン連携）) | [optional]
**label** | Option<**String**> | 項目名 | [optional]
**annotation** | Option<**String**> | 追加説明 | [optional]
**required** | Option<**bool**> | 必須かどうか | [optional]
**values** | Option<[**Vec<crate::models::ApprovalRequestResponseApprovalRequestApprovalRequestFormValues>**](approvalRequestResponse_approval_request_approval_request_form_values.md)> | 選択項目 | [optional]
**max_amount** | Option<**i32**> | 上限金額 | [optional]
**min_amount** | Option<**i32**> | 下限金額 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


