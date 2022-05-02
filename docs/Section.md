# Section

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 部門ID | 
**name** | **String** | 部門名 (30文字以内) | 
**available** | **bool** | 部門の使用設定（true: 使用する、false: 使用しない） <br> <ul>   <li>     本APIでsectionを作成した場合はtrueになります。   </li>   <li>     falseにする場合はWeb画面から変更できます。   </li>   <li>     trueの場合、Web画面での取引登録時などに入力候補として表示されます。   </li>   <li>     falseの場合、部門自体は削除せず、Web画面での取引登録時などに入力候補として表示されません。ただし取引（収入／支出）の作成APIなどでfalseの部門をパラメータに指定すれば、取引などにfalseの部門を設定できます。   </li> </ul> | 
**long_name** | Option<**String**> | 正式名称（255文字以内） | [optional]
**company_id** | **i32** | 事業所ID | 
**shortcut1** | Option<**String**> | ショートカット１ (20文字以内) | [optional]
**shortcut2** | Option<**String**> | ショートカット２ (20文字以内) | [optional]
**indent_count** | Option<**i32**> | <a target=\"_blank\" href=\"https://support.freee.co.jp/hc/ja/articles/209093566\">部門階層</a> <br> ※ indent_count が 0 のときは第一階層の親部門です。  | [optional]
**parent_id** | Option<**i32**> | <a target=\"_blank\" href=\"https://support.freee.co.jp/hc/ja/articles/209093566\">親部門ID</a> <br> ※ parent_id が null のときは第一階層の親部門です。  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


