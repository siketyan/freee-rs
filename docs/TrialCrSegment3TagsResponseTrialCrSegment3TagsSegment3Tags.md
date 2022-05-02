# TrialCrSegment3TagsResponseTrialCrSegment3TagsSegment3Tags

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | セグメント3タグID | 
**name** | Option<**String**> | セグメント3タグ名 | [optional]
**closing_balance** | Option<**i32**> | 期末残高 | [optional]
**partners** | Option<[**Vec<crate::models::TrialCrSectionsResponseTrialCrSectionsPartners>**](trialCrSectionsResponse_trial_cr_sections_partners.md)> | breakdown_display_type:partner, account_item_display_type:account_item指定時のみ含まれる | [optional]
**items** | Option<[**Vec<crate::models::TrialCrSectionsResponseTrialCrSectionsItems>**](trialCrSectionsResponse_trial_cr_sections_items.md)> | breakdown_display_type:item, account_item_display_type:account_item指定時のみ含まれる | [optional]
**sections** | Option<[**Vec<crate::models::TrialCrSegment1TagsResponseTrialCrSegment1TagsSections>**](trialCrSegment_1TagsResponse_trial_cr_segment_1_tags_sections.md)> | breakdown_display_type:section, account_item_display_type:account_item指定時のみ含まれる | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


