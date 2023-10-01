# Receipt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | ファイルボックス（証憑ファイル）ID | 
**status** | **String** | ステータス(confirmed:確認済み、deleted:削除済み、ignored:無視) | 
**description** | Option<**String**> | メモ | [optional]
**mime_type** | **String** | MIMEタイプ | 
**origin** | **String** | アップロード元種別 | 
**created_at** | **String** | アップロード日時（ISO8601形式） | 
**user** | [**crate::models::DealCreateResponseDealReceiptsInnerUser**](dealCreateResponse_deal_receipts_inner_user.md) |  | 
**receipt_metadatum** | Option<[**crate::models::ReceiptUpdateParamsReceiptMetadatum**](receiptUpdateParams_receipt_metadatum.md)> |  | [optional]
**qualified_invoice** | Option<**String**> | 適格請求書等（qualified: 該当する、not_qualified: 該当しない、unselected: 未選択、null: OCR解析結果が保存されている時等） | [optional]
**invoice_registration_number** | Option<**String**> | インボイス制度適格請求書発行事業者登録番号（null: OCR解析結果が保存されている時等） - 先頭T数字13桁の固定14桁の文字列 <a target=\"_blank\" href=\"https://www.invoice-kohyo.nta.go.jp/index.html\">国税庁インボイス制度適格請求書発行事業者公表サイト</a>  | [optional]
**document_type** | Option<**String**> | 書類の種類（receipt: 領収書、invoice: 請求書、other: その他、null: OCR解析結果が保存されている時等） | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


