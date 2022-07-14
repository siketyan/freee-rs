# Walletable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 口座ID | 
**name** | **String** | 口座名 (255文字以内) | 
**bank_id** | Option<**i32**> | サービスID | 
**_type** | **String** | 口座区分 (銀行口座: bank_account, クレジットカード: credit_card, 現金: wallet) | 
**last_balance** | Option<**i32**> | 同期残高 | [optional]
**walletable_balance** | Option<**i32**> | 登録残高 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


