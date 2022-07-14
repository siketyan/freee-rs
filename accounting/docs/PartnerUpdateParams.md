# PartnerUpdateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**name** | **String** | 取引先名 (255文字以内) | 
**shortcut1** | Option<**String**> | ショートカット１ (255文字以内) | [optional]
**shortcut2** | Option<**String**> | ショートカット２ (255文字以内) | [optional]
**org_code** | Option<**i32**> | 事業所種別（null: 未設定、1: 法人、2: 個人） | [optional]
**country_code** | Option<**String**> | 地域（JP: 国内、ZZ:国外）、指定しない場合JPになります。 | [optional]
**long_name** | Option<**String**> | 正式名称（255文字以内） | [optional]
**name_kana** | Option<**String**> | カナ名称（255文字以内） | [optional]
**default_title** | Option<**String**> | 敬称（御中、様、(空白)の3つから選択） | [optional]
**phone** | Option<**String**> | 電話番号 | [optional]
**contact_name** | Option<**String**> | 担当者 氏名 (255文字以内) | [optional]
**email** | Option<**String**> | 担当者 メールアドレス (255文字以内) | [optional]
**payer_walletable_id** | Option<**i32**> | 振込元口座ID（一括振込ファイル用）:（walletableのtypeが'bank_account'のidのみ指定できます。また、未設定にする場合は、nullを指定してください。） | [optional]
**transfer_fee_handling_side** | Option<**String**> | 振込手数料負担（一括振込ファイル用）: (振込元(当方): payer, 振込先(先方): payee)、指定しない場合payerになります。 | [optional]
**address_attributes** | Option<[**crate::models::PartnerCreateParamsAddressAttributes**](partnerCreateParams_address_attributes.md)> |  | [optional]
**partner_doc_setting_attributes** | Option<[**crate::models::PartnerCreateParamsPartnerDocSettingAttributes**](partnerCreateParams_partner_doc_setting_attributes.md)> |  | [optional]
**partner_bank_account_attributes** | Option<[**crate::models::PartnerCreateParamsPartnerBankAccountAttributes**](partnerCreateParams_partner_bank_account_attributes.md)> |  | [optional]
**payment_term_attributes** | Option<[**crate::models::PartnerCreateParamsPaymentTermAttributes**](partnerCreateParams_payment_term_attributes.md)> |  | [optional]
**invoice_payment_term_attributes** | Option<[**crate::models::PartnerCreateParamsInvoicePaymentTermAttributes**](partnerCreateParams_invoice_payment_term_attributes.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

