# AccountItemResponseAccountItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 勘定科目ID | 
**name** | **String** | 勘定科目名 (30文字以内) | 
**company_id** | **i32** | 事業所ID | 
**tax_code** | **i32** | 税区分コード | 
**account_category** | **String** | 勘定科目カテゴリー | 
**account_category_id** | **i32** | 勘定科目のカテゴリーID | 
**shortcut** | Option<**String**> | ショートカット1 (20文字以内) | [optional]
**shortcut_num** | Option<**String**> | ショートカット2(勘定科目コード) (20文字以内) | [optional]
**searchable** | **i32** | 検索可能:2, 検索不可：3 | 
**accumulated_dep_account_item_name** | Option<**String**> | 減価償却累計額勘定科目（法人のみ利用可能） | [optional]
**accumulated_dep_account_item_id** | Option<**i32**> | 減価償却累計額勘定科目ID（法人のみ利用可能） | [optional]
**items** | Option<[**Vec<crate::models::AccountItemResponseAccountItemItems>**](accountItemResponse_account_item_items.md)> |  | [optional]
**partners** | Option<[**Vec<crate::models::AccountItemResponseAccountItemPartners>**](accountItemResponse_account_item_partners.md)> |  | [optional]
**available** | **bool** | 勘定科目の使用設定（true: 使用する、false: 使用しない） | 
**walletable_id** | Option<**i32**> | 口座ID | 
**group_name** | Option<**String**> | 決算書表示名（小カテゴリー） | [optional]
**corresponding_income_name** | Option<**String**> | 収入取引相手勘定科目名 | [optional]
**corresponding_income_id** | Option<**i32**> | 収入取引相手勘定科目ID | [optional]
**corresponding_expense_name** | Option<**String**> | 支出取引相手勘定科目名 | [optional]
**corresponding_expense_id** | Option<**i32**> | 支出取引相手勘定科目ID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


