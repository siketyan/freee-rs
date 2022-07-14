# WalletTxn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 明細ID | 
**company_id** | **i32** | 事業所ID | 
**date** | **String** | 取引日(yyyy-mm-dd) | 
**amount** | **i64** | 取引金額 | 
**due_amount** | **i32** | 未決済金額 | 
**balance** | **i32** | 残高(銀行口座等) | 
**entry_side** | **String** | 入金／出金 (入金: income, 出金: expense) | 
**walletable_type** | **String** | 口座区分 (銀行口座: bank_account, クレジットカード: credit_card, 現金: wallet) | 
**walletable_id** | **i32** | 口座ID | 
**description** | **String** | 取引内容 | 
**status** | **i32** | 明細のステータス（消込待ち: 1, 消込済み: 2, 無視: 3, 消込中: 4, 対象外: 6） | 
**rule_matched** | **bool** | 登録時に<a href=\"https://support.freee.co.jp/hc/ja/articles/202848350-明細の自動登録ルールを設定する\" target=\"_blank\">自動登録ルールの設定</a>が適用され、登録処理が実行された場合、 trueになります。〜を推測する、〜の消込をするの条件の場合は一致してもfalseになります。  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


