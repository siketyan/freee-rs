# ApiV1EmployeeCreateRequestSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**num** | Option<**String**> | 従業員番号<br> 従業員を判別しやすいよう管理することができます。（例: 1人目の正社員を A-001 と入力） | [optional]
**working_hours_system_name** | Option<**String**> | <a href=\"https://support.freee.co.jp/hc/ja/articles/360000562403-労働時間制度と勤務-賃金設定について\" target=\"_blank\">勤務・賃金設定名</a> で設定した名称を指定してください。 - 未指定の際は、最初に登録したデータが利用されます。 - 入力パラメータのno_payroll_calculationがtrueの場合に指定するとエラーになります。 | [optional]
**company_reference_date_rule_name** | Option<**String**> | <a href=\"https://support.freee.co.jp/hc/ja/articles/360000666303-締め日支払い日を変更する方法は-\" target=\"_blank\">締め日支払い日グループ名</a> で設定した締め日支払い日を指定してください。 - 未指定の際は、最初に登録したデータが利用されます。 - 入力パラメータのno_payroll_calculationがtrueの場合に指定するとエラーになります。 | [optional]
**last_name** | **String** | 姓（必須） | 
**first_name** | **String** | 名（必須） | 
**last_name_kana** | **String** | 姓カナ（必須） | 
**first_name_kana** | **String** | 名カナ（必須） | 
**birth_date** | [**String**](string.md) | 生年月日（必須） | 
**entry_date** | [**String**](string.md) | 入社日（必須） | 
**pay_calc_type** | Option<**String**> | 給与方式 monthly: 月給, daily: 日給, hourly: 時給 - フレックスタイム制を使用している場合はmonthly以外指定できません。 - 入力パラメータのno_payroll_calculationがfalseの場合は必須になります。 - 入力パラメータのno_payroll_calculationがtrueの場合に指定するとエラーになります。 | [optional]
**pay_amount** | Option<**i32**> | 基本給 - 入力パラメータのno_payroll_calculationがfalseの場合は必須になります。 - 入力パラメータのno_payroll_calculationがtrueの場合に指定するとエラーになります。 | [optional]
**gender** | Option<**String**> | 性別　unselected: 未選択, male: 男性, female: 女性（デフォルト: unselected: 未選択） | [optional]
**married** | Option<**bool**> | 配偶者の有無（デフォルト: false） | [optional]
**no_payroll_calculation** | Option<**bool**> | 給与計算対象外の従業員情報を作成する場合はtrueを指定します | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


