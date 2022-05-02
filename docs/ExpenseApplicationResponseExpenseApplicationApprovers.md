# ExpenseApplicationResponseExpenseApplicationApprovers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**step_id** | **i32** | 承認ステップID | 
**user_id** | Option<**i32**> | 承認者のユーザーID 下記の場合はnullになります。 <ul>   <li>resource_type:selected_userの場合で承認者未指定時</li>   <li>resource_type:or_positionで前stepで部門未指定の場合</li> </ul> | 
**status** | **String** | 承認者の承認状態 * `initial` - 初期状態 * `approved` - 承認済 * `rejected` - 却下 * `feedback` - 差戻し | 
**is_force_action** | **bool** | 代理承認済みかどうか | 
**resource_type** | **String** | 承認ステップの承認方法 * ` predefined_user` - メンバー指定 (1人), * ` selected_user` - 申請時にメンバー指定 * ` unspecified` - 指定なし * ` and_resource` - メンバー指定 (複数、全員の承認), * ` or_resource` - メンバー指定 (複数、1人の承認) * ` and_position` - 役職指定 (複数、全員の承認) * ` or_position` - 役職指定 (複数、1人の承認) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


