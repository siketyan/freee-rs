# FixedAsset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**id** | **i32** | 固定資産ID | 
**name** | **String** | 固定資産名 | 
**management_number** | Option<**String**> | 管理番号 | [optional]
**account_item_id** | Option<**i32**> | 勘定科目ID | [optional]
**section_id** | Option<**i32**> | 部門ID | [optional]
**item_id** | Option<**i32**> | 品目ID | [optional]
**depreciation_method** | Option<**String**> | 償却方法:(少額償却: small_sum_method, 一括償却: lump_sum_method, 定額法: straight_line_method, 定率法: multiple_method, 旧定率法: old_multiple_method, 旧定額法: old_straight_line_method, 償却なし: non_depreciate_method, 任意償却: voluntary_method, 即時償却: immediate_method, 均等償却: equal_method) | [optional]
**depreciation_account_item_id** | Option<**i32**> | 減価償却に使う勘定科目ID | [optional]
**prefecture_code** | Option<**i32**> | 都道府県コード（-1: 設定しない、0:北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄 | [optional]
**city_name** | Option<**String**> | 申告先市区町村 | [optional]
**depreciation_amount** | Option<**i32**> | 減価償却費 | [optional]
**acquisition_cost** | Option<**i32**> | 取得価額 | [optional]
**opening_balance** | Option<**i32**> | 期首残高 | [optional]
**closing_balance** | Option<**i32**> | 未償却残高(期末残高) | [optional]
**opening_accumulated_depreciation** | Option<**i32**> | 期首減価償却累計額 | [optional]
**closing_accumulated_depreciation** | Option<**i32**> | 期末減価償却累計額 | [optional]
**life_years** | Option<**i32**> | 耐用年数 | [optional]
**acquisition_date** | Option<[**String**](string.md)> | 取得日 | [optional]
**updated_at** | Option<[**String**](string.md)> | 更新日 | [optional]
**retire_type** | Option<**String**> | 売却もしくは除却ステータス: (売却: sell, 除却: retire, 償却中: depreciating) | [optional]
**retire_date** | Option<[**String**](string.md)> | 除却日、もしくは売却日 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


