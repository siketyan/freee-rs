# TrialPlResponseTrialPl

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**fiscal_year** | Option<**i32**> | 会計年度(条件に指定した時、または条件に月、日条件がない時のみ含まれる） | [optional]
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)(条件に指定した時のみ含まれる） | [optional]
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(条件に指定した時のみ含まれる） | [optional]
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd)(条件に指定した時のみ含まれる） | [optional]
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd)(条件に指定した時のみ含まれる） | [optional]
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）(条件に指定した時のみ含まれる） | [optional]
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag）(条件に指定した時のみ含まれる） | [optional]
**partner_id** | Option<**i32**> | 取引先ID(条件に指定した時のみ含まれる） | [optional]
**partner_code** | Option<**String**> | 取引先コード(条件に指定した時のみ含まれる） | [optional]
**item_id** | Option<**i32**> | 品目ID(条件に指定した時のみ含まれる） | [optional]
**section_id** | Option<**i32**> | 部門ID(条件に指定した時のみ含まれる） | [optional]
**adjustment** | Option<**String**> | 決算整理仕訳のみ: only, 決算整理仕訳以外: without(条件に指定した時のみ含まれる） | [optional]
**cost_allocation** | Option<**String**> | 配賦仕訳のみ：only,配賦仕訳以外：without(条件に指定した時のみ含まれる） | [optional]
**approval_flow_status** | Option<**String**> | 未承認を除く: without_in_progress (デフォルト), 全てのステータス: all(条件に指定した時のみ含まれる） | [optional]
**created_at** | Option<**String**> | 作成日時 | [optional]
**balances** | [**Vec<crate::models::TrialPlResponseTrialPlBalances>**](trialPlResponse_trial_pl_balances.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


