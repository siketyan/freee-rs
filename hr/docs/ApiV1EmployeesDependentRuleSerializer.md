# ApiV1EmployeesDependentRuleSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 扶養親族ルールID | [optional]
**company_id** | Option<**i32**> | 事業所ID | [optional]
**employee_id** | Option<**i32**> | 従業員ID | [optional]
**last_name** | Option<**String**> | 姓 | [optional]
**first_name** | Option<**String**> | 名 | [optional]
**last_name_kana** | Option<**String**> | 姓カナ | [optional]
**first_name_kana** | Option<**String**> | 名カナ | [optional]
**gender** | Option<**String**> | 性別　unselected: 未選択, male: 男性, female: 女性 | [optional]
**relationship** | Option<**String**> | 続柄 spouse: 配偶者, father: 父, mother: 母, child: 子, senior_brother: 兄, junior_brother: 弟, senior_sister: 姉, junior_sister: 妹, grandchild: 孫, grandfather: 祖父, grandmother: 祖母, father_in_law: 義父, mother_in_law: 義母, grandfather_in_law: 義祖父, grandmother_in_law: 義祖母, other: その他, great_grandfather: 曽祖父, great_grandmother: 曽祖母, spouses_child: 配偶者の連れ子 | [optional]
**birth_date** | Option<[**String**](string.md)> | 生年月日 | [optional]
**residence_type** | Option<**String**> | 同居・別居 live_in: 同居, resident: 別居(国内), non_resident: 別居(国外) | [optional]
**zipcode1** | Option<**String**> | 住民票住所の郵便番号1 | [optional]
**zipcode2** | Option<**String**> | 住民票住所の郵便番号2 | [optional]
**prefecture_code** | Option<**i32**> | 住民票住所の都道府県コード（-1: 設定しない、0: 北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄) | [optional]
**address** | Option<**String**> | 住民票住所の市区町村以降の住所 | [optional]
**address_kana** | Option<**String**> | 住民票住所の市区町村以降の住所カナ | [optional]
**base_pension_num** | Option<**String**> | 基礎年金番号 | [optional]
**income** | Option<**i32**> | 年間所得 | [optional]
**annual_revenue** | Option<**i32**> | 年間収入 | [optional]
**disability_type** | Option<**String**> | 障害に該当するか na: 障害なし, general: 一般の障害者, heavy: 特別障害者 | [optional]
**occupation** | Option<**String**> | 職業 | [optional]
**annual_remittance_amount** | Option<**i32**> | 一年間の送金額 | [optional]
**employment_insurance_receive_status** | Option<**String**> | 雇用保険受給の有無 - unselected 未選択 - receiving_employment_insurance 雇用保険受給有り - not_receiving_employment_insurance 雇用保険受給無し - pending_employment_insurance 申請中 | [optional]
**employment_insurance_receives_from** | Option<[**String**](string.md)> | 雇用保険受給開始年月日 employment_insurance_receive_statusが未選択、無しの場合は指定できません。 | [optional]
**phone_type** | Option<**String**> | 電話番号の種別 - unselected 未選択 - home 自宅 - office 勤務先 - mobile 携帯 - other その他 | [optional]
**phone1** | Option<**String**> | 電話番号1（先頭番号、例:03-1111-222x の03部分） | [optional]
**phone2** | Option<**String**> | 電話番号2（中間番号、例:03-1111-222x の1111部分） | [optional]
**phone3** | Option<**String**> | 電話番号3（末尾番号、例:03-1111-222x の222x部分） | [optional]
**social_insurance_and_tax_dependent** | Option<**String**> | 扶養状況 social_insurance_and_tax: 所得税・住民税と社会保険, tax_only: 所得税・住民税のみ, social_insurance_only: 社会保険のみ | [optional]
**social_insurance_dependent_acquisition_date** | Option<[**String**](string.md)> | 社会保険の扶養加入日 | [optional]
**social_insurance_dependent_acquisition_reason** | Option<**String**> | 社会保険の扶養加入理由 配偶者の場合 \"\": 未選択, start_working: 配偶者の就職, marriage: 婚姻, turnover: 離職, decrease_in_income: 収入減少, other: その他 配偶者以外の場合 \"\": 未選択, birth: 出生, turnover: 離職, decrease_in_income: 収入減, live_in: 同居, other: その他 | [optional]
**social_insurance_other_dependent_acquisition_reason** | Option<**String**> | 社会保険のその他の扶養加入理由 | [optional]
**social_insurance_dependent_disqualification_date** | Option<[**String**](string.md)> | 社会保険の扶養喪失日 | [optional]
**social_insurance_dependent_disqualification_reason** | Option<**String**> | 社会保険の扶養喪失理由 配偶者の場合 \"\": 未選択, death: 死亡, divorce: 離婚, start_working_or_increase_in_income: 就職・収入増加, reach_75_years_old: 歳到達, disability: 障害認定, other: その他 配偶者以外の場合 \"\": 未選択, death: 死亡, start_working: 就職, increase_in_income: 収入増加, reach_75_years_old: ７５歳到達, disability: 障害認定, other: その他 | [optional]
**social_insurance_other_dependent_disqualification_reason** | Option<**String**> | 社会保険のその他の扶養喪失理由 | [optional]
**tax_dependent_acquisition_date** | Option<[**String**](string.md)> | 税扶養の加入日 | [optional]
**tax_dependent_acquisition_reason** | Option<**String**> | 税扶養の加入理由 配偶者の場合 \"\": 未選択, start_working: 配偶者の就職, marriage: 婚姻, turnover: 離職, decrease_in_income: 収入減少, other: その他 配偶者以外の場合 \"\": birth: 出生, turnover: 離職, decrease_in_income: 収入減, live_in: 同居, other: その他 | [optional]
**tax_other_dependent_acquisition_reason** | Option<**String**> | 税扶養のその他の加入理由 | [optional]
**tax_dependent_disqualification_date** | Option<[**String**](string.md)> | 税扶養の喪失日 | [optional]
**tax_dependent_disqualification_reason** | Option<**String**> | 税扶養の喪失理由 配偶者の場合 \"\": 未選択, death: 死亡, divorce: 離婚, start_working_or_increase_in_income: 就職・収入増加, reach_75_years_old: 歳到達, disability: 障害認定, other: その他 配偶者以外の場合 \"\": 未選択, death: 死亡, start_working: 就職, increase_in_income: 収入増加, reach_75_years_old: ７５歳到達, disability: 障害認定, other: その他 | [optional]
**tax_other_dependent_disqualification_reason** | Option<**String**> | 税扶養のその他の喪失理由 | [optional]
**non_resident_dependents_reason** | Option<**String**> | 非居住者である親族の条件 none: なし, over_16_to_under_30_or_over_70: 16歳以上30歳未満又は70歳以上, study_abroad: 留学, handicapped: 障害者, over_38_man: 38万円以上の支払 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


