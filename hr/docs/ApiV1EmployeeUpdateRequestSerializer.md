# ApiV1EmployeeUpdateRequestSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**num** | Option<**String**> | 従業員番号<br> 従業員を判別しやすいよう管理することができます。（例: 1人目の正社員を A-001 と入力） | [optional]
**display_name** | Option<**String**> | 従業員名（freee人事労務上での表示にのみ使用される名前です。出力書類には姓名が使用されます。） - 給与計算対象外の従業員情報の場合は必須になります。 | [optional]
**base_pension_num** | Option<**String**> | 基礎年金番号 数値文字列10桁固定長 例: 1111111111 | [optional]
**employment_insurance_reference_number** | Option<**String**> | 被保険者番号（雇用保険） 数値文字列11桁固定長 例: 11111111111 - 給与計算対象外の従業員情報の場合に指定するとエラーになります。 | [optional]
**birth_date** | [**String**](string.md) | 生年月日 null不可 | 
**entry_date** | [**String**](string.md) | 入社日 null不可 | 
**retire_date** | Option<[**String**](string.md)> | 退職日 - 退職していない場合は指定不要です。 - 指定する場合はentry_date以降の日付を指定してください。 - retire_dateをクリアする場合、nullを指定してください。 | [optional]
**company_reference_date_rule_name** | Option<**String**> | <a href=\"https://support.freee.co.jp/hc/ja/articles/360000666303-締め日支払い日を変更する方法は-\" target=\"_blank\">締め日支払い日グループ名</a> で設定した締め日支払い日を指定してください。 - 未指定の際は、締め日支払い日は変わりません。 - 指定した従業員が給与計算対象外の場合、指定するとエラーになります。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


