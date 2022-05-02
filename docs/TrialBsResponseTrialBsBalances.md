# TrialBsResponseTrialBsBalances

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_item_id** | Option<**i32**> | 勘定科目ID(勘定科目の時のみ含まれる) | [optional]
**account_item_name** | Option<**String**> | 勘定科目名(勘定科目の時のみ含まれる) | [optional]
**account_group_name** | Option<**String**> | 決算書表示名(account_item_display_type:group指定時に決算書表示名の時のみ含まれる) | [optional]
**partners** | Option<[**Vec<crate::models::TrialBsResponseTrialBsPartners>**](trialBsResponse_trial_bs_partners.md)> | breakdown_display_type:partner, account_item_display_type:account_item指定時のみ含まれる | [optional]
**items** | Option<[**Vec<crate::models::TrialBsResponseTrialBsItems>**](trialBsResponse_trial_bs_items.md)> | breakdown_display_type:item, account_item_display_type:account_item指定時のみ含まれる | [optional]
**sections** | Option<[**Vec<crate::models::TrialBsResponseTrialBsSections>**](trialBsResponse_trial_bs_sections.md)> | breakdown_display_type:section, account_item_display_type:account_item指定時のみ含まれる | [optional]
**segment_1_tags** | Option<[**Vec<crate::models::TrialBsResponseTrialBsSegment1Tags>**](trialBsResponse_trial_bs_segment_1_tags.md)> | breakdown_display_type:segment_1_tag, account_item_display_type:account_item指定時のみ含まれる | [optional]
**segment_2_tags** | Option<[**Vec<crate::models::TrialBsResponseTrialBsSegment2Tags>**](trialBsResponse_trial_bs_segment_2_tags.md)> | breakdown_display_type:segment_2_tag, account_item_display_type:account_item指定時のみ含まれる | [optional]
**segment_3_tags** | Option<[**Vec<crate::models::TrialBsResponseTrialBsSegment3Tags>**](trialBsResponse_trial_bs_segment_3_tags.md)> | breakdown_display_type:segment_3_tag, account_item_display_type:account_item指定時のみ含まれる | [optional]
**account_category_name** | Option<**String**> | 勘定科目カテゴリー名 | [optional]
**total_line** | Option<**bool**> | 合計行(勘定科目カテゴリーの時のみ含まれる) | [optional]
**hierarchy_level** | Option<**i32**> | 階層レベル | [optional]
**parent_account_category_name** | Option<**String**> | 上位勘定科目カテゴリー名(勘定科目カテゴリーの時のみ、上層が存在する場合含まれる) | [optional]
**opening_balance** | Option<**i32**> | 期首残高 | [optional]
**debit_amount** | Option<**i32**> | 借方金額 | [optional]
**credit_amount** | Option<**i32**> | 貸方金額 | [optional]
**closing_balance** | Option<**i32**> | 期末残高 | [optional]
**composition_ratio** | Option<**f32**> | 構成比 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


