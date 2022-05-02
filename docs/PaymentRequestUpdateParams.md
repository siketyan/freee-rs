# PaymentRequestUpdateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**title** | **String** | 申請タイトル<br> 申請者が、下書き状態もしくは差戻し状態の支払依頼に対して指定する場合のみ有効  | 
**application_date** | Option<**String**> | 申請日 (yyyy-mm-dd)<br> 指定しない場合は当日の日付が登録されます。<br> 申請者が、下書き状態もしくは差戻し状態の支払依頼に対して指定する場合のみ有効  | [optional]
**description** | Option<**String**> | 備考 | [optional]
**payment_request_lines** | [**Vec<crate::models::PaymentRequestUpdateParamsPaymentRequestLines>**](paymentRequestUpdateParams_payment_request_lines.md) | 支払依頼の項目行一覧（配列） | 
**approver_id** | Option<**i32**> | 承認者のユーザーID<br> 「承認者を指定」の経路を申請経路として使用する場合に指定してください。<br> 指定する承認者のユーザーIDは、申請経路APIを利用して取得してください。  | [optional]
**approval_flow_route_id** | **i32** | 申請経路ID<br> 指定する申請経路IDは、申請経路APIを利用して取得してください。  | 
**draft** | **bool** | 支払依頼のステータス<br> falseを指定した時は申請中（in_progress）で支払依頼を更新します。<br> trueを指定した時は下書き（draft）で支払依頼を更新します。<br> 未指定の時は下書きとみなして支払依頼を更新します。  | 
**document_code** | Option<**String**> | 請求書番号（255文字以内） | [optional]
**receipt_ids** | Option<**Vec<i32>**> | 証憑ファイルID（ファイルボックスのファイルID）（配列） | [optional]
**issue_date** | **String** | 発生日 (yyyy-mm-dd) | 
**payment_date** | Option<**String**> | 支払期限 (yyyy-mm-dd) | [optional]
**payment_method** | Option<**String**> | '支払方法(none: 指定なし, domestic_bank_transfer: 国内振込, abroad_bank_transfer: 国外振込, account_transfer: 口座振替, credit_card: クレジットカード)'<br> 'デフォルトは none: 指定なし です。'  | [optional]
**partner_id** | Option<**i32**> | 支払先の取引先ID | [optional]
**partner_code** | Option<**String**> | 支払先の取引先コード<br> 支払先の取引先ID指定時には無効  | [optional]
**bank_code** | Option<**String**> | 銀行コード（半角数字1桁〜4桁）<br> 支払先指定時には無効  | [optional]
**bank_name** | Option<**String**> | 銀行名（255文字以内）<br> 支払先指定時には無効  | [optional]
**bank_name_kana** | Option<**String**> | 銀行名（カナ）（15文字以内）<br> 支払先指定時には無効  | [optional]
**branch_code** | Option<**String**> | 支店番号（半角数字1桁〜3桁）<br> 支払先指定時には無効  | [optional]
**branch_name** | Option<**String**> | 支店名（255文字以内）<br> 支払先指定時には無効  | [optional]
**branch_kana** | Option<**String**> | 支店名（カナ）（15文字以内）<br> 指定可能な文字は、英数・カナ・丸括弧・ハイフン・スペースのみです。<br> 支払先指定時には無効  | [optional]
**account_name** | Option<**String**> | 受取人名（カナ）（48文字以内）<br> 支払先指定時には無効  | [optional]
**account_number** | Option<**String**> | 口座番号（半角数字1桁〜7桁）<br> 支払先指定時には無効  | [optional]
**account_type** | Option<**String**> | '口座種別(ordinary: 普通、checking: 当座、earmarked: 納税準備預金、savings: 貯蓄、other: その他)'<br> '支払先指定時には無効'<br> 'デフォルトは ordinary: 普通 です'  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


