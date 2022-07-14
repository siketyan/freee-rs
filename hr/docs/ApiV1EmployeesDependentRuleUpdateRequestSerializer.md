# ApiV1EmployeesDependentRuleUpdateRequestSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | 扶養親族ID（idがない場合は新規作成になる) | [optional]
**last_name** | **String** | 姓 null不可 | 
**first_name** | **String** | 名 null不可 | 
**last_name_kana** | Option<**String**> | 姓カナ | [optional]
**first_name_kana** | Option<**String**> | 名カナ | [optional]
**gender** | **String** | 性別　unselected: 未選択, male: 男性, female: 女性（デフォルト: unselected: 未選択） | 
**relationship** | **String** | 続柄 null不可 spouse: 配偶者, father: 父, mother: 母, child: 子, senior_brother: 兄, junior_brother: 弟, senior_sister: 姉, junior_sister: 妹, grandchild: 孫, grandfather: 祖父, grandmother: 祖母, father_in_law: 義父, mother_in_law: 義母, grandfather_in_law: 義祖父, grandmother_in_law: 義祖母, other: その他, great_grandfather: 曽祖父, great_grandmother: 曽祖母, spouses_child: 配偶者の連れ子 | 
**birth_date** | [**String**](string.md) | 生年月日 null不可 | 
**residence_type** | **String** | 同居・別居 null不可 live_in: 同居, resident: 別居(国内), non_resident: 別居(国外) | 
**zipcode1** | Option<**String**> | 住民票住所の郵便番号1 | [optional]
**zipcode2** | Option<**String**> | 住民票住所の郵便番号2 | [optional]
**prefecture_code** | Option<**i32**> | 住民票住所の都道府県コード（-1: 設定しない、0: 北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄) | [optional]
**address** | Option<**String**> | 住民票住所の市区町村以降の住所 | [optional]
**address_kana** | Option<**String**> | 住民票住所の市区町村以降の住所カナ | [optional]
**base_pension_num** | Option<**String**> | 基礎年金番号 | [optional]
**income** | **i32** | 年間所得 null不可 | 
**annual_revenue** | **i32** | 年間収入 null不可 | 
**disability_type** | **String** | 障害に該当するか null不可 na: 障害なし, general: 一般の障害者, heavy: 特別障害者 | 
**occupation** | Option<**String**> | 職業 | [optional]
**annual_remittance_amount** | Option<**i32**> | 一年間の送金額 | [optional]
**destroy** | Option<**bool**> | 扶養親族を削除するか | [optional]
**social_insurance_and_tax_dependent** | **String** | 扶養状況 social_insurance_and_tax: 所得税・住民税と社会保険, tax_only: 所得税・住民税のみ, social_insurance_only: 社会保険のみ, not_dependent: 扶養していない | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


