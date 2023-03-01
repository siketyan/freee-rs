# Receipt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | ファイルボックス（証憑ファイル）ID | 
**status** | **String** | ステータス(confirmed:確認済み、deleted:削除済み、ignored:無視) | 
**description** | Option<**String**> | メモ | [optional]
**mime_type** | **String** | MIMEタイプ | 
**issue_date** | Option<**String**> | 発生日 | [optional]
**origin** | **String** | アップロード元種別 | 
**created_at** | **String** | 作成日時（ISO8601形式） | 
**file_src** | **String** | ファイルのダウンロードURL（freeeにログインした状態でのみ閲覧可能です。） <br> <br> file_srcは廃止予定の属性になります。<br> file_srcに替わり、証憑ファイルのダウンロード APIをご利用ください。<br> 証憑ファイルのダウンロードAPIを利用することで、以下のようになります。 <ul>   <li>アプリケーション利用者はfreee APIアプリケーションにログインしていれば、証憑ダウンロード毎にfreeeに改めてログインすることなくファイルが参照できるようになります。</li> </ul> | 
**user** | [**crate::models::DealCreateResponseDealReceiptsInnerUser**](dealCreateResponse_deal_receipts_inner_user.md) |  | 
**receipt_metadatum** | Option<[**crate::models::ReceiptUpdateParamsReceiptMetadatum**](receiptUpdateParams_receipt_metadatum.md)> |  | [optional]
**qualified_invoice** | Option<**String**> | この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 適格請求書等（qualified: 該当する、not_qualified: 該当しない）  | [optional]
**invoice_registration_number** | Option<**String**> | この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 インボイス制度適格請求書発行事業者登録番号 - 先頭T数字13桁の固定14桁の文字列 <a target=\"_blank\" href=\"https://www.invoice-kohyo.nta.go.jp/index.html\">国税庁インボイス制度適格請求書発行事業者公表サイト</a>  | [optional]
**document_type** | Option<**String**> | この項目はインボイス制度で利用する項目です。2023年4月頃から利用できる予定です。 書類の種類（receipt: 領収書、invoice: 請求書、other: その他）  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


