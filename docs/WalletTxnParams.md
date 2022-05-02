# WalletTxnParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entry_side** | **String** | 入金／出金 (入金: income, 出金: expense) | 
**description** | Option<**String**> | 取引内容 | [optional]
**amount** | **i64** | 取引金額 | 
**walletable_id** | **i32** | 口座ID | 
**walletable_type** | **String** | 口座区分 (銀行口座: bank_account, クレジットカード: credit_card, 現金: wallet) | 
**date** | **String** | 取引日 (yyyy-mm-dd) | 
**company_id** | **i32** | 事業所ID | 
**balance** | Option<**i64**> | 残高 (銀行口座等) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


