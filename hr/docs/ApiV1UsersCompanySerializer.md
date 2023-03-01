# ApiV1UsersCompanySerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 事業所ID | [optional]
**name** | Option<**String**> | 事業所名 | [optional]
**role** | Option<**String**> | 事業所におけるロール。 - `company_admin`: 管理者 - `self_only`: 一般 - `clerk`: 事務担当者  [各権限でできることは各アカウントの権限についてのヘルプページを参照してください。](https://support.freee.co.jp/hc/ja/articles/204087410-%E6%93%8D%E4%BD%9C%E6%A8%A9%E9%99%90%E3%82%92%E7%AE%A1%E7%90%86%E3%81%99%E3%82%8B#3) | [optional]
**external_cid** | Option<**String**> | 事業所番号(半角数字10桁) | [optional]
**employee_id** | Option<**i32**> | 事業所に所属する従業員としての従業員ID、従業員情報が未登録の場合はnullになります。 | [optional]
**display_name** | Option<**String**> | 事業所に所属する従業員の表示名 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


