# ApiV1EmployeeGroupMembershipSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 従業員ID | [optional]
**num** | Option<**String**> | 従業員番号 | [optional]
**display_name** | Option<**String**> | 従業員名（表示名） | [optional]
**entry_date** | Option<[**String**](string.md)> | 入社日 | [optional]
**retire_date** | Option<[**String**](string.md)> | 退職日 | [optional]
**user_id** | Option<**i32**> | ユーザーID(従業員詳細未設定の場合、nullになります。) | [optional]
**login_email** | Option<**String**> | ログイン用メールアドレス(従業員詳細未設定の場合、nullになります。) | [optional]
**birth_date** | Option<[**String**](string.md)> | 生年月日 | [optional]
**gender** | Option<**String**> | 性別　unselected: 未選択, male: 男性, female: 女性 | [optional]
**payroll_calculation** | Option<**bool**> | 給与計算対象従業員の場合trueを返します | [optional]
**group_memberships** | Option<[**Vec<crate::models::ApiV1GroupMembershipSerializer>**](ApiV1GroupMembershipSerializer.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


