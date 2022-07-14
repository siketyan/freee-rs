# ExpenseApplicationLineTemplateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**name** | **String** | 経費科目名 (100文字以内) | 
**account_item_id** | **i32** | 勘定科目ID | 
**item_id** | Option<**i32**> | 品目ID | [optional]
**tax_code** | **i32** | 税区分コード（税区分のdisplay_categoryがtax_5: 5%表示の税区分, tax_r8: 軽減税率8%表示の税区分に該当するtax_codeのみ利用可能です。税区分のdisplay_categoryは /taxes/companies/{:company_id}のAPIから取得可能です。） | 
**description** | Option<**String**> | 経費科目の説明 (1000文字以内) | [optional]
**line_description** | Option<**String**> | 内容の補足 (1000文字以内) | [optional]
**required_receipt** | Option<**bool**> | 添付ファイルの必須/任意 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


