# DealReceipts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | 証憑ファイルID（ファイルボックスのファイルID） | 
**status** | **String** | ステータス(unconfirmed:確認待ち、confirmed:確認済み、deleted:削除済み、ignored:無視) | 
**description** | Option<**String**> | メモ | [optional]
**mime_type** | **String** | MIMEタイプ | 
**issue_date** | Option<**String**> | 発生日 | [optional]
**origin** | **String** | アップロード元種別 | 
**created_at** | **String** | 作成日時（ISO8601形式） | 
**file_src** | **String** | ファイルのダウンロードURL（freeeにログインした状態でのみ閲覧可能です。） <br> <br> file_srcは廃止予定の属性になります。<br> file_srcに替わり、証憑ファイル（ファイルボックスのファイル）のダウンロード APIをご利用ください。<br> 証憑ファイル（ファイルボックスのファイル）のダウンロードAPIを利用することで、以下のようになります。 <ul>   <li>アプリケーション利用者はfreee APIアプリケーションにログインしていれば、証憑ダウンロード毎にfreeeに改めてログインすることなくファイルが参照できるようになります。</li> </ul> | 
**user** | [**crate::models::DealUser**](deal_user.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


