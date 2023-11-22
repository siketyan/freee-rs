# ApiV1EmployeeYearendAdjustmentInsuranceUpdateRequestSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | 保険の種類 life_care_pension_insurance: 生命保険・介護医療保険・個人年金保険, earthquake_non_life_insurance: 地震保険・旧長期損害保険, social_insurance: 社会保険, other_insurance: その他の保険（小規模企業共済等） | 
**category** | **String** | 区分<br> 保険会社等が発行する証明書類に基づいて区分を設定してください。<br> 保険の種類によって設定可能な値が変わります。<br> ・life_care_pension_insurance<br> 　life: 生命保険<br> 　care: 介護保険<br> 　pension: 個人年金保険<br> ・earthquake_non_life_insurance<br> 　earthquake: 地震保険<br> 　old_long_term_non_life: 旧長期損害保険<br> ・social_insurance<br> 　national_pension: 国民年金<br> 　national_pension_fund_premium: 国民年金基金<br> 　national_health: 国民健康保険<br> 　health: 健康保険<br> 　care_insurance_deduction_of_pension: 介護保険<br> 　employee_pension: 厚生年金<br> 　advanced_elderly_medical: 後期高齢者医療保険<br> 　none: その他（印刷後に手書き）<br> ・other_insurance<br> 　sema: 独立行政法人中小企業基盤整備機構の共済契約の掛金<br> 　idc: 個人型確定拠出年金（iDeCo）<br> 　cdc: 企業型確定拠出年金<br> 　dsma: 心身障害者扶養共済制度に関する契約の掛金<br> | 
**new_or_old** | **String** | 新旧区分<br> 区分が生命保険または個人年金保険の時のみ、新制度なら new を、旧制度なら old を指定します。<br> 上記以外の保険では none を指定してください。 | 
**company_name** | Option<**String**> | 保険会社等の名称<br> 保険の種類にて、生命保険・介護医療保険・個人年金保険または地震保険・旧長期損害保険を選択している時のみ、入力した値が反映されます。<br> 上記以外の保険では入力した値は反映されません。 | [optional]
**kind_of_purpose** | Option<**String**> | 保険等の種類（目的）<br> 保険の種類にて、生命保険・介護医療保険・個人年金保険または地震保険・旧長期損害保険を選択している時のみ、入力した値が反映されます。<br> 上記以外の保険では入力した値は反映されません。 | [optional]
**period** | Option<**String**> | 保険期間又は年金支払期間<br> 保険の種類にて、生命保険・介護医療保険・個人年金保険または地震保険・旧長期損害保険を選択している時のみ、入力した値が反映されます。<br> 上記以外の保険では入力した値は反映されません。 | [optional]
**policyholder_last_name** | Option<**String**> | 保険等の契約者 姓<br> 保険の種類にて、生命保険・介護医療保険・個人年金保険または地震保険・旧長期損害保険を選択している時のみ、入力した値が反映されます。<br> 上記以外の保険では入力した値は反映されません。 | [optional]
**policyholder_first_name** | Option<**String**> | 保険等の契約者 名<br> 保険の種類にて、生命保険・介護医療保険・個人年金保険または地震保険・旧長期損害保険を選択している時のみ、入力した値が反映されます。<br> 上記以外の保険では入力した値は反映されません。 | [optional]
**recipient_last_name** | Option<**String**> | 保険金等の受取人 姓<br> 保険の種類にて、生命保険・介護医療保険・個人年金保険、地震保険・旧長期損害保険または社会保険を選択している時のみ、入力した値が反映されます。<br> 上記以外の保険では入力した値は反映されません。 | [optional]
**recipient_first_name** | Option<**String**> | 保険金等の受取人 名<br> 保険の種類にて、生命保険・介護医療保険・個人年金保険、地震保険・旧長期損害保険または社会保険を選択している時のみ、入力した値が反映されます。<br> 上記以外の保険では入力した値は反映されません。 | [optional]
**recipient_relationship** | Option<**String**> | 保険金等の受取人 あなたとの続柄 myself: 本人, husband: 夫, wife: 妻, father: 父, mother: 母, child: 子, senior_brother: 兄, junior_brother: 弟, senior_sister: 姉, junior_sister: 妹, grandchild: 孫, grandfather: 祖父, grandmother: 祖母, father_in_law: 義父, mother_in_law: 義母, grandfather_in_law: 義祖父, grandmother_in_law: 義祖母, other: その他, \"\": 空欄<br> 保険の種類にて、生命保険・介護医療保険・個人年金保険、地震保険・旧長期損害保険または社会保険を選択している時のみ、入力した値が反映されます。<br> 上記以外の保険では入力した値は反映されません。 | [optional]
**payment_start_date** | Option<**String**> | 年金の支払開始日 1900年1月1日から現在年+100の12月31日まで登録可能。<br> 区分が個人年金保険の時のみ、入力した値が反映されます。<br> 上記以外の保険では入力した値は反映されません。 | [optional]
**amount** | **i32** | 保険料額 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


