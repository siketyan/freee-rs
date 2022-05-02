# PartnersResponsePartners

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 取引先ID | 
**code** | Option<**String**> | 取引先コード | 
**company_id** | **i32** | 事業所ID | 
**name** | **String** | 取引先名 | 
**update_date** | **String** | 更新日 (yyyy-mm-dd) | 
**available** | **bool** | 取引先の使用設定（true: 使用する、false: 使用しない） <br> <ul>   <li>     本APIでpartnerを作成した場合はtrueになります。   </li>   <li>     falseにする場合はWeb画面から変更できます。   </li>   <li>     trueの場合、Web画面での取引登録時などに入力候補として表示されます。   </li>   <li>     falseの場合、取引先自体は削除せず、Web画面での取引登録時などに入力候補として表示されません。ただし取引（収入／支出）の作成APIなどでfalseの取引先をパラメータに指定すれば、取引などにfalseの取引先を設定できます。   </li> </ul> | 
**shortcut1** | Option<**String**> | ショートカット1 (255文字以内) | [optional]
**shortcut2** | Option<**String**> | ショートカット2 (255文字以内) | [optional]
**org_code** | Option<**i32**> | 事業所種別（null: 未設定、1: 法人、2: 個人） | [optional]
**country_code** | Option<**String**> | 地域（JP: 国内、ZZ:国外） | [optional]
**long_name** | Option<**String**> | 正式名称（255文字以内） | [optional]
**name_kana** | Option<**String**> | カナ名称（255文字以内） | [optional]
**default_title** | Option<**String**> | 敬称（御中、様、(空白)の3つから選択） | [optional]
**phone** | Option<**String**> | 電話番号 | [optional]
**contact_name** | Option<**String**> | 担当者 氏名 | [optional]
**email** | Option<**String**> | 担当者 メールアドレス | [optional]
**payer_walletable_id** | Option<**i32**> | 振込元口座ID（一括振込ファイル用）:（未設定の場合は、nullです。） | [optional]
**transfer_fee_handling_side** | Option<**String**> | 振込手数料負担（一括振込ファイル用）: (振込元(当方): payer, 振込先(先方): payee) | [optional]
**address_attributes** | Option<[**crate::models::PartnersResponseAddressAttributes**](partnersResponse_address_attributes.md)> |  | [optional]
**partner_doc_setting_attributes** | Option<[**crate::models::PartnersResponsePartnerDocSettingAttributes**](partnersResponse_partner_doc_setting_attributes.md)> |  | [optional]
**partner_bank_account_attributes** | Option<[**crate::models::PartnersResponsePartnerBankAccountAttributes**](partnersResponse_partner_bank_account_attributes.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


