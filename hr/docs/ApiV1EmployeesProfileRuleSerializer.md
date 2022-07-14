# ApiV1EmployeesProfileRuleSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**company_id** | Option<**i32**> | 事業所ID | [optional]
**employee_id** | Option<**i32**> | 従業員ID | [optional]
**last_name** | Option<**String**> | 姓 | [optional]
**first_name** | Option<**String**> | 名 | [optional]
**last_name_kana** | Option<**String**> | 姓カナ | [optional]
**first_name_kana** | Option<**String**> | 名カナ | [optional]
**zipcode1** | Option<**String**> | 住民票住所の郵便番号1 | [optional]
**zipcode2** | Option<**String**> | 住民票住所の郵便番号2 | [optional]
**prefecture_code** | Option<**i32**> | 住民票住所の都道府県コード（-1: 設定しない、0: 北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄) | [optional]
**address** | Option<**String**> | 住民票住所の市区町村以降の住所 | [optional]
**address_kana** | Option<**String**> | 住民票住所の市区町村以降の住所カナ | [optional]
**phone1** | Option<**String**> | 電話番号1 | [optional]
**phone2** | Option<**String**> | 電話番号2 | [optional]
**phone3** | Option<**String**> | 電話番号3 | [optional]
**residential_zipcode1** | Option<**String**> | 現住所の郵便番号１ | [optional]
**residential_zipcode2** | Option<**String**> | 現住所の郵便番号２ | [optional]
**residential_prefecture_code** | Option<**i32**> | 現住所の都道府県コード（-1: 設定しない、0: 北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄) | [optional]
**residential_address** | Option<**String**> | 現住所の住所 | [optional]
**residential_address_kana** | Option<**String**> | 現住所の住所カナ | [optional]
**employment_type** | Option<**String**> | 雇用形態 board-member: 役員, (空文字列): 役員以外 | [optional]
**title** | Option<**String**> | 肩書 | [optional]
**gender** | Option<**String**> | 性別　unselected: 未選択, male: 男性, female: 女性 | [optional]
**married** | Option<**bool**> | 配偶者の有無 | [optional]
**is_working_student** | Option<**bool**> | 勤労学生かどうか | [optional]
**widow_type** | Option<**String**> | 寡夫／寡婦かどうか na: 空白, widower: 寡夫, widow: 寡婦, special_widow: 特別寡婦, one_parent: ひとり親 | [optional]
**disability_type** | Option<**String**> | 障害者かどうか na: 空白, general: 障害者, heavy: 特別障害者 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


