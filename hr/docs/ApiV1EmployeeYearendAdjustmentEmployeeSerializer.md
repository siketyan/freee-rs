# ApiV1EmployeeYearendAdjustmentEmployeeSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**employee_id** | Option<**i32**> | 従業員ID | [optional]
**num** | Option<**String**> | 従業員番号<br> 従業員を判別しやすいよう管理することができます。（例: 1人目の正社員を A-001 と入力） | [optional]
**last_name** | Option<**String**> | 姓 null不可 | [optional]
**first_name** | Option<**String**> | 名 null不可 | [optional]
**last_name_kana** | Option<**String**> | 姓カナ | [optional]
**first_name_kana** | Option<**String**> | 名カナ | [optional]
**birth_date** | Option<[**String**](string.md)> | 生年月日 | [optional]
**entry_date** | Option<[**String**](string.md)> | 入社日 | [optional]
**retire_date** | Option<[**String**](string.md)> | 退職日 | [optional]
**employment_type** | Option<**String**> | 雇用形態 board-member: 役員, (空文字列): 役員以外 | [optional]
**title** | Option<**String**> | 肩書 | [optional]
**zipcode1** | Option<**String**> | 住民票住所の郵便番号1 | [optional]
**zipcode2** | Option<**String**> | 住民票住所の郵便番号2 | [optional]
**prefecture_code** | Option<**i32**> | 住民票住所の都道府県コード（-1: 設定しない、0: 北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄) | [optional]
**address** | Option<**String**> | 住民票住所の市区町村以降の住所 | [optional]
**address_kana** | Option<**String**> | 住民票住所の市区町村以降の住所カナ | [optional]
**payer_type** | Option<**String**> | 所得税納税者区分 kou: 甲, otsu: 乙, hei: 丙 | [optional]
**widow_type** | Option<**String**> | 寡夫／寡婦かどうか null不可 na: 空白, widow: 寡婦, one_parent: ひとり親 | [optional]
**disability_type** | Option<**String**> | 障害者かどうか na: 空白, general: 障害者, heavy: 特別障害者 | [optional]
**married** | Option<**bool**> | 配偶者の有無 | [optional]
**is_working_student** | Option<**bool**> | 勤労学生かどうか | [optional]
**is_foreigner** | Option<**bool**> | 外国人かどうか | [optional]
**other_company_revenue** | Option<**i32**> | その他の事業所の給与収入 | [optional]
**all_other_income** | Option<**i32**> | 給与以外の所得 | [optional]
**householder** | Option<**String**> | 世帯主の続柄 myself: 本人, husband: 夫, wife: 妻, father: 父, mother: 母, child: 子, senior_brother: 兄, junior_brother: 弟, senior_sister: 姉, junior_sister: 妹, grandchild: 孫, grandfather: 祖父, grandmother: 祖母, father_in_law: 義父, mother_in_law: 義母, grandfather_in_law: 義祖父, grandmother_in_law: 義祖母, other: その他 | [optional]
**householder_name** | Option<**String**> | 世帯主の名前 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


