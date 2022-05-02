# QuotationResponseQuotation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 見積書ID | 
**company_id** | **i32** | 事業所ID | 
**issue_date** | **String** | 見積日 (yyyy-mm-dd) | 
**partner_id** | Option<**i32**> | 取引先ID | 
**partner_code** | Option<**String**> | 取引先コード | [optional]
**quotation_number** | **String** | 見積書番号 | 
**title** | Option<**String**> | タイトル | [optional]
**total_amount** | **i32** | 合計金額 | 
**total_vat** | Option<**i32**> | 消費税 | [optional]
**sub_total** | Option<**i32**> | 小計 | [optional]
**description** | Option<**String**> | 概要 | [optional]
**quotation_status** | **String** | 見積書ステータス  (unsubmitted: 送付待ち, submitted: 送付済み, all: 全て) | 
**web_published_at** | Option<**String**> | Web共有日時(最新) | [optional]
**web_downloaded_at** | Option<**String**> | Web共有ダウンロード日時(最新) | [optional]
**web_confirmed_at** | Option<**String**> | Web共有取引先確認日時(最新) | [optional]
**mail_sent_at** | Option<**String**> | メール送信日時(最新) | [optional]
**partner_name** | Option<**String**> | 取引先名 | [optional]
**partner_display_name** | Option<**String**> | 見積書に表示する取引先名 | [optional]
**partner_title** | Option<**String**> | 敬称（御中、様、(空白)の3つから選択） | 
**partner_zipcode** | Option<**String**> | 郵便番号 | [optional]
**partner_prefecture_code** | Option<**i32**> | 都道府県コード（-1: 設定しない、0:北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄 | [optional]
**partner_prefecture_name** | Option<**String**> | 都道府県 | [optional]
**partner_address1** | Option<**String**> | 市区町村・番地 | [optional]
**partner_address2** | Option<**String**> | 建物名・部屋番号など | [optional]
**partner_contact_info** | Option<**String**> | 取引先担当者名 | [optional]
**company_name** | **String** | 事業所名 | 
**company_zipcode** | Option<**String**> | 郵便番号 | [optional]
**company_prefecture_code** | Option<**i32**> | 都道府県コード（-1: 設定しない、0:北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄 | [optional]
**company_prefecture_name** | Option<**String**> | 都道府県 | [optional]
**company_address1** | Option<**String**> | 市区町村・番地 | [optional]
**company_address2** | Option<**String**> | 建物名・部屋番号など | [optional]
**company_contact_info** | Option<**String**> | 事業所担当者名 | [optional]
**message** | Option<**String**> | メッセージ | [optional]
**notes** | Option<**String**> | 備考 | [optional]
**quotation_layout** | **String** | 見積書レイアウト * `default_classic` - レイアウト１/クラシック (デフォルト)  * `standard_classic` - レイアウト２/クラシック  * `envelope_classic` - 封筒１/クラシック  * `default_modern` - レイアウト１/モダン  * `standard_modern` - レイアウト２/モダン  * `envelope_modern` - 封筒/モダン | 
**tax_entry_method** | **String** | 見積書の消費税計算方法(inclusive: 内税, exclusive: 外税) | 
**quotation_contents** | Option<[**Vec<crate::models::QuotationIndexResponseQuotationContents>**](quotationIndexResponse_quotation_contents.md)> | 見積内容 | [optional]
**total_amount_per_vat_rate** | [**crate::models::InvoiceIndexResponseTotalAmountPerVatRate**](invoiceIndexResponse_total_amount_per_vat_rate.md) |  | 
**related_invoice_id** | Option<**i32**> | 関連する請求書ID<br> 下記で作成したものが該当します。  <a href=\"https://support.freee.co.jp/hc/ja/articles/203318410#1-2\" target=\"_blank\">見積書・納品書を納品書・請求書に変換する</a><br> <a href=\"https://support.freee.co.jp/hc/ja/articles/209076226\" target=\"_blank\">複数の見積書・納品書から合算請求書を作成する</a><br>  | [optional]
**related_quotation_ids** | Option<**Vec<i32>**> | 関連する見積書ID(配列)<br> 下記で作成したものが該当します。  <a href=\"https://support.freee.co.jp/hc/ja/articles/203318410#1-2\" target=\"_blank\">見積書・納品書を納品書・請求書に変換する</a><br> <a href=\"https://support.freee.co.jp/hc/ja/articles/209076226\" target=\"_blank\">複数の見積書・納品書から合算請求書を作成する</a><br>  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


