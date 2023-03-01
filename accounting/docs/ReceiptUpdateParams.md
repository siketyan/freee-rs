# ReceiptUpdateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**description** | Option<**String**> | メモ (255文字以内) | [optional]
**issue_date** | **String** | 取引日 (yyyy-mm-dd) | 
**receipt_metadatum** | Option<[**crate::models::ReceiptUpdateParamsReceiptMetadatum**](receiptUpdateParams_receipt_metadatum.md)> |  | [optional]
**qualified_invoice** | Option<**String**> | この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 適格請求書等（qualified: 該当する、not_qualified: 該当しない）  | [optional]
**invoice_registration_number** | Option<**String**> | この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 インボイス制度適格請求書発行事業者登録番号 - 先頭T数字13桁の固定14桁の文字列 <a target=\"_blank\" href=\"https://www.invoice-kohyo.nta.go.jp/index.html\">国税庁インボイス制度適格請求書発行事業者公表サイト</a>  | [optional]
**document_type** | Option<**String**> | この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 書類の種類（receipt: 領収書、invoice: 請求書、other: その他）  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


