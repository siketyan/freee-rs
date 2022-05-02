# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 品目ID | 
**company_id** | **i32** | 事業所ID | 
**name** | **String** | 品目名 (30文字以内) | 
**update_date** | **String** | 更新日(yyyy-mm-dd) | 
**available** | **bool** | 品目の使用設定（true: 使用する、false: 使用しない） <br> <ul>   <li>     本APIでitemを作成した場合はtrueになります。   </li>   <li>     falseにする場合はWeb画面から変更できます。   </li>   <li>     trueの場合、Web画面での取引登録時などに入力候補として表示されます。   </li>   <li>     falseの場合、品目自体は削除せず、Web画面での取引登録時などに入力候補として表示されません。ただし取引（収入／支出）の作成APIなどでfalseの品目をパラメータに指定すれば、取引などにfalseの品目を設定できます。   </li> </ul> | 
**shortcut1** | Option<**String**> | ショートカット１ (20文字以内) | [optional]
**shortcut2** | Option<**String**> | ショートカット２ (20文字以内) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


