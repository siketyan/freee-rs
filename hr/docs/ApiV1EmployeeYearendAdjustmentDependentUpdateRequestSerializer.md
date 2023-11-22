# ApiV1EmployeeYearendAdjustmentDependentUpdateRequestSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 家族情報ID（idがない場合は新規作成になる) | [optional]
**destroy** | Option<**bool**> | 家族情報を削除するか true: 削除する, false: 削除しない | [optional]
**last_name** | **String** | 姓 null不可 | 
**first_name** | **String** | 名 null不可 | 
**last_name_kana** | Option<**String**> | 姓カナ | [optional]
**first_name_kana** | Option<**String**> | 名カナ | [optional]
**relationship** | **String** | 続柄 null不可 spouse: 配偶者, father: 父, mother: 母, child: 子, senior_brother: 兄, junior_brother: 弟, senior_sister: 姉, junior_sister: 妹, grandchild: 孫, grandfather: 祖父, grandmother: 祖母, father_in_law: 義父, mother_in_law: 義母, grandfather_in_law: 義祖父, grandmother_in_law: 義祖母, other: その他, great_grandfather: 曽祖父, great_grandmother: 曽祖母, spouses_child: 配偶者の連れ子 | 
**birth_date** | [**String**](string.md) | 生年月日 null不可 1900年1月1日から現在年+5の12月31日まで登録可能 | 
**social_insurance_and_tax_dependent** | **String** | 扶養状況 social_insurance_and_tax: 所得税・住民税と社会保険, tax_only: 所得税・住民税のみ, social_insurance_only: 社会保険のみ, not_dependent: 扶養していない | 
**income** | Option<**i32**> | 所得 配偶者は「扶養状況」がsocial_insurance_only又はnot_dependentの場合のみ更新可能。配偶者以外は更新可能。 配偶者で「扶養状況」がsocial_insurance_and_tax又はtax_onlyの場合、「給与収入」、「給与以外の所得」から自動で「所得」が計算されます。 | [optional]
**employment_revenue** | Option<**i32**> | 給与収入 配偶者は「扶養状況」がsocial_insurance_and_tax又はtax_onlyの場合のみ更新可能。配偶者以外は更新不可。更新不可の場合は0となります。 | [optional]
**all_other_income** | Option<**i32**> | 給与以外の所得 配偶者は「扶養状況」がsocial_insurance_and_tax又はtax_onlyの場合のみ更新可能。配偶者以外は更新不可。更新不可の場合は0となります。 | [optional]
**disability_type** | **String** | 障害に該当するか null不可 na: 障害なし, general: 一般の障害者, heavy: 特別障害者 | 
**residence_type** | **String** | 同居・別居 null不可 live_in: 同居, resident: 別居(国内), non_resident: 別居(国外) | 
**zipcode1** | Option<**String**> | 住民票住所の郵便番号1 「同居・別居」が「同居」の場合は「年末調整従業員情報」の「住民票住所の郵便番号1」を登録 | [optional]
**zipcode2** | Option<**String**> | 住民票住所の郵便番号2 「同居・別居」が「同居」の場合は「年末調整従業員情報」の「住民票住所の郵便番号2」を登録 | [optional]
**prefecture_code** | Option<**i32**> | 住民票住所の都道府県コード（-1: 設定しない、0: 北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄)  「同居・別居」が「同居」の場合は「年末調整従業員情報」の「住民票住所の都道府県コード」を登録 | [optional]
**address** | Option<**String**> | 住民票住所の市区町村以降の住所 「同居・別居」が「同居」の場合は「年末調整従業員情報」の「住民票住所の市区町村以降の住所」を登録 | [optional]
**address_kana** | Option<**String**> | 住民票住所の市区町村以降の住所カナ 「同居・別居」が「同居」の場合は「年末調整従業員情報」の「住民票住所の市区町村以降の住所カナ」を登録 | [optional]
**annual_remittance_amount** | Option<**i32**> | 国外居住親族への年間の送金額 「同居・別居」が「同居」、「別居(国内)」の場合は登録不可 | [optional]
**non_resident_dependents_reason** | Option<**String**> | 非居住者である親族の条件 none: なし, over_16_to_under_30_or_over_70: 16歳以上30歳未満又は70歳以上, study_abroad: 留学, handicapped: 障害者, over_38_man: 38万円以上の支払 続柄が「配偶者」または「同居・別居」が「同居」、「別居(国内)」の場合は登録不可 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


