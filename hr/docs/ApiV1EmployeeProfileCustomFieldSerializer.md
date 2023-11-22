# ApiV1EmployeeProfileCustomFieldSerializer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | カスタム項目ID | [optional]
**field_type** | Option<**String**> | 項目タイプ file: ファイル, text: テキスト, number: 数値, date: 日付, selector: セレクター, time: 時間 | [optional]
**name** | Option<**String**> | 項目名 | [optional]
**value** | Option<**String**> | 値（項目タイプがfileの場合、nullを返します） | [optional]
**file_name** | Option<**String**> | ファイル名（項目タイプがfile以外の場合、nullを返します） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


