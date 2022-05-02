# MeResponseUserCompanies

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 事業所ID | 
**display_name** | **String** | 表示名 | 
**role** | **String** | ユーザーの権限 <ul> <li>admin: 管理者</li> <li>simple_accounting: 一般</li> <li>self_only: 取引登録のみ</li> <li>read_only: 閲覧のみ</li> <li>workflow: 申請・承認</li> </ul> | 
**use_custom_role** | **bool** | カスタム権限（true: 使用する、false: 使用しない） | 
**advisor_id** | Option<**i32**> | アドバイザープロファイルID（アドバイザー事業所で無い場合にnullになります） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


