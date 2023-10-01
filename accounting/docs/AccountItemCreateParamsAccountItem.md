# AccountItemCreateParamsAccountItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | 勘定科目名 (30文字以内) | 
**shortcut** | Option<**String**> | ショートカット1 (20文字以内) | [optional]
**shortcut_num** | Option<**String**> | ショートカット2(勘定科目コード)(20文字以内) | [optional]
**tax_code** | **i32** | 税区分コード 指定できるコードは本APIの注意点をご確認ください。 | 
**group_name** | **String** | 決算書表示名（小カテゴリー） Selectablesフォーム用選択項目情報エンドポイント(account_groups.name)で取得可能です | 
**account_category_id** | **i32** | 勘定科目カテゴリーID Selectablesフォーム用選択項目情報エンドポイント(account_groups.account_category_id)で取得可能です | 
**corresponding_income_id** | **i32** | 収入取引相手勘定科目ID | 
**corresponding_expense_id** | **i32** | 支出取引相手勘定科目ID | 
**accumulated_dep_account_item_id** | Option<**i32**> | 減価償却累計額勘定科目ID（法人のみ利用可能） | [optional]
**searchable** | Option<**i32**> | 検索可能:2, 検索不可：3(登録時未指定の場合は2で登録されます。更新時未指定の場合はsearchableは変更されません。) | [optional]
**items** | Option<[**Vec<crate::models::AccountItemCreateParamsAccountItemItemsInner>**](accountItemCreateParams_account_item_items_inner.md)> | 品目 | [optional]
**partners** | Option<[**Vec<crate::models::AccountItemCreateParamsAccountItemItemsInner>**](accountItemCreateParams_account_item_items_inner.md)> | 取引先 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


