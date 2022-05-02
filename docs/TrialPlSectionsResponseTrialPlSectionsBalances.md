# TrialPlSectionsResponseTrialPlSectionsBalances

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_item_id** | Option<**i32**> | 勘定科目ID(勘定科目の時のみ含まれる) | [optional]
**account_item_name** | Option<**String**> | 勘定科目名(勘定科目の時のみ含まれる) | [optional]
**account_group_name** | Option<**String**> | 決算書表示名(account_item_display_type:group指定時に決算書表示名の時のみ含まれる) | [optional]
**sections** | Option<[**Vec<crate::models::TrialPlSectionsResponseTrialPlSectionsSections>**](trialPlSectionsResponse_trial_pl_sections_sections.md)> | 部門 | [optional]
**account_category_name** | Option<**String**> | 勘定科目カテゴリー名 | [optional]
**total_line** | Option<**bool**> | 合計行(勘定科目カテゴリーの時のみ含まれる) | [optional]
**hierarchy_level** | Option<**i32**> | 階層レベル | [optional]
**parent_account_category_name** | Option<**String**> | 上位勘定科目カテゴリー名(勘定科目カテゴリーの時のみ、上層が存在する場合含まれる) | [optional]
**closing_balance** | Option<**i32**> | 期末残高 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


