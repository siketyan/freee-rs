# WalletableCreateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | 口座名 (255文字以内) | 
**r#type** | **String** | 口座種別（bank_account : 銀行口座, credit_card : クレジットカード, wallet : その他の決済口座） | 
**company_id** | **i32** | 事業所ID | 
**bank_id** | Option<**i32**> | 連携サービスID（typeにbank_account、credit_cardを指定する場合は必須） | [optional]
**is_asset** | Option<**bool**> | 口座を資産口座とするか負債口座とするか（true: 資産口座 (デフォルト), false: 負債口座）<br> bank_idを指定しない場合にのみ使われます。<br> bank_idを指定する場合には資産口座か負債口座かはbank_idに指定したサービスに応じて決定され、is_assetに指定した値は無視されます。  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


