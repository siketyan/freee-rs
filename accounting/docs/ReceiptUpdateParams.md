# ReceiptUpdateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | 
**description** | Option<**String**> | メモ (255文字以内) | [optional]
**receipt_metadatum** | Option<[**crate::models::ReceiptUpdateParamsReceiptMetadatum**](receiptUpdateParams_receipt_metadatum.md)> |  | [optional]
**qualified_invoice** | Option<**String**> | 適格請求書等（qualified: 該当する、not_qualified: 該当しない、unselected: 未選択） | [optional]
**invoice_registration_number** | Option<**String**> | インボイス制度適格請求書発行事業者登録番号 - 先頭T数字13桁の固定14桁の文字列 <a target=\"_blank\" href=\"https://www.invoice-kohyo.nta.go.jp/index.html\">国税庁インボイス制度適格請求書発行事業者公表サイト</a>  | [optional]
**document_type** | Option<**String**> | 書類の種類（receipt: 領収書、invoice: 請求書、other: その他） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


