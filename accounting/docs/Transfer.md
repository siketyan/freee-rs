# Transfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 取引(振替)ID | 
**company_id** | **i32** | 事業所ID | 
**amount** | **i64** | 金額 | 
**date** | **String** | 振替日 (yyyy-mm-dd) | 
**from_walletable_type** | Option<**String**> | 振替元口座区分 (銀行口座: bank_account, クレジットカード: credit_card, 現金: wallet) | 
**from_walletable_id** | **i32** | 振替元口座ID | 
**to_walletable_type** | Option<**String**> | 振替先口座区分 (銀行口座: bank_account, クレジットカード: credit_card, 現金: wallet) | 
**to_walletable_id** | **i32** | 振替先口座ID | 
**description** | **String** | 備考 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


