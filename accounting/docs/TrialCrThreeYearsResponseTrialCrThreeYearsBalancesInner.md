# TrialCrThreeYearsResponseTrialCrThreeYearsBalancesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_item_id** | Option<**i32**> | 勘定科目ID(勘定科目の時のみ含まれる) | [optional]
**account_item_name** | Option<**String**> | 勘定科目名(勘定科目の時のみ含まれる) | [optional]
**account_group_name** | Option<**String**> | 決算書表示名(account_item_display_type:group指定時に決算書表示名の時のみ含まれる) | [optional]
**partners** | Option<[**Vec<crate::models::TrialBsThreeYearsResponseTrialBsThreeYearsBalancesInnerPartnersInner>**](trialBsThreeYearsResponse_trial_bs_three_years_balances_inner_partners_inner.md)> | breakdown_display_type:partner, account_item_display_type:account_item指定時のみ含まれる | [optional]
**items** | Option<[**Vec<crate::models::TrialCrThreeYearsResponseTrialCrThreeYearsBalancesInnerItemsInner>**](trialCrThreeYearsResponse_trial_cr_three_years_balances_inner_items_inner.md)> | breakdown_display_type:item, account_item_display_type:account_item指定時のみ含まれる | [optional]
**sections** | Option<[**Vec<crate::models::TrialBsThreeYearsResponseTrialBsThreeYearsBalancesInnerSectionsInner>**](trialBsThreeYearsResponse_trial_bs_three_years_balances_inner_sections_inner.md)> | breakdown_display_type:section, account_item_display_type:account_item指定時のみ含まれる | [optional]
**segment_1_tags** | Option<[**Vec<crate::models::TrialCrThreeYearsResponseTrialCrThreeYearsBalancesInnerSegment1TagsInner>**](trialCrThreeYearsResponse_trial_cr_three_years_balances_inner_segment_1_tags_inner.md)> | breakdown_display_type:segment_1_tag, account_item_display_type:account_item指定時のみ含まれる | [optional]
**segment_2_tags** | Option<[**Vec<crate::models::TrialCrThreeYearsResponseTrialCrThreeYearsBalancesInnerSegment2TagsInner>**](trialCrThreeYearsResponse_trial_cr_three_years_balances_inner_segment_2_tags_inner.md)> | breakdown_display_type:segment_2_tag, account_item_display_type:account_item指定時のみ含まれる | [optional]
**segment_3_tags** | Option<[**Vec<crate::models::TrialCrThreeYearsResponseTrialCrThreeYearsBalancesInnerSegment3TagsInner>**](trialCrThreeYearsResponse_trial_cr_three_years_balances_inner_segment_3_tags_inner.md)> | breakdown_display_type:segment_3_tag, account_item_display_type:account_item指定時のみ含まれる | [optional]
**account_category_name** | Option<**String**> | 勘定科目カテゴリー名 | [optional]
**total_line** | Option<**bool**> | 合計行(勘定科目カテゴリーの時のみ含まれる) | [optional]
**hierarchy_level** | Option<**i32**> | 階層レベル | [optional]
**parent_account_category_name** | Option<**String**> | 上位勘定科目カテゴリー名(勘定科目カテゴリーの時のみ、上層が存在する場合含まれる) | [optional]
**two_years_before_closing_balance** | Option<**i32**> | 前々年度期末残高 | [optional]
**last_year_closing_balance** | Option<**i32**> | 前年度期末残高 | [optional]
**closing_balance** | Option<**i32**> | 期末残高 | [optional]
**year_on_year** | Option<**f32**> | 前年比 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


