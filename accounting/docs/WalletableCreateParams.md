# WalletableCreateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | 口座名 (255文字以内) | 
**_type** | **String** | 口座種別（bank_account : 銀行口座, credit_card : クレジットカード, wallet : その他の決済口座） | 
**company_id** | **i32** | 事業所ID | 
**bank_id** | Option<**i32**> | サービスID | [optional]
**group_name** | Option<**String**> | 決算書表示名（小カテゴリー）　例：売掛金, 受取手形, 未収入金（法人のみ）, 買掛金, 支払手形, 未払金, 預り金, 前受金 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


