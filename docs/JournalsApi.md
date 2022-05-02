# \JournalsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_journal**](JournalsApi.md#download_journal) | **GET** /api/1/journals/reports/{id}/download | ダウンロード実行
[**get_journal_status**](JournalsApi.md#get_journal_status) | **GET** /api/1/journals/reports/{id}/status | ステータス確認
[**get_journals**](JournalsApi.md#get_journals) | **GET** /api/1/journals | ダウンロード要求



## download_journal

> String download_journal(id, company_id)
ダウンロード実行

 <h2 id=\"\">概要</h2>  <p>ダウンロードを実行する</p>  <p>＊このAPIは無料プランのアカウントではご利用になれません</p>  <h2 id=\"_2\">定義</h2>  <ul> <li>id : 受け付けID</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | 受け付けID | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

**String**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journal_status

> crate::models::JournalStatusResponse get_journal_status(company_id, id)
ステータス確認

 <h2 id=\"\">概要</h2>  <p>ダウンロードリクエストのステータスを確認する</p>  <p>＊このAPIは無料プランのアカウントではご利用になれません</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>status</p>  <ul> <li>enqueued : 実行待ち</li>  <li>working : 実行中</li>  <li>uploaded : 準備完了</li> </ul> </li>  <li> <p>id : 受け付けID</p> </li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** | 受け付けID | [required] |

### Return type

[**crate::models::JournalStatusResponse**](journalStatusResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journals

> crate::models::JournalsResponse get_journals(download_type, company_id, visible_tags, visible_ids, start_date, end_date)
ダウンロード要求

 <h2 id=\"\">概要</h2>  <p>ユーザーが所属する事業所の仕訳帳のダウンロードをリクエストします 生成されるファイルに関しては、<a href=\"https://support.freee.co.jp/hc/ja/articles/204599604#2\">ヘルプページ</a>をご参照ください</p>  <p>＊このAPIは無料プランのアカウントではご利用になれません</p>  <h2 id=\"_2\">定義</h2>  <ul>   <li>download_type     <ul>       <li>generic(freee Webからダウンロードできるものと同じ)</li>       <li>csv (yayoi形式)</li>       <li>pdf</li>     </ul>   </li>   <li>visible_tags : 指定しない場合は従来の仕様の仕訳帳が出力されます     <ul>       <li>partner : 取引先タグ</li>       <li>item : 品目タグ</li>       <li>tag : メモタグ</li>       <li>section : 部門タグ</li>       <li>description : 備考欄</li>       <li>wallet_txn_description : 明細の備考欄</li>       <li>         segment_1_tag : セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン)<br>         segment_2_tag : セグメント2(法人向け エンタープライズプラン)<br>         segment_3_tag : セグメント3(法人向け エンタープライズプラン)<br><br>         <a href=\"https://support.freee.co.jp/hc/ja/articles/360020679611\" target=\"_blank\">セグメント（分析用タグ）の設定</a><br>       </li>       <li>all : 指定された場合は上記の設定をすべて有効として扱いますが、セグメント1、セグメント2、セグメント3は含みません。セグメントが必要な場合はallではなく、segment_1_tag, segment_2_tag, segment_3_tagを指定してください。</li>     </ul>   </li>   <li>visible_ids : download_typeがgenericの場合のみ利用可能です     <ul>       <li>deal_id : 取引ID</li>       <li>transfer_id : 取引(振替)ID</li>       <li>manual_journal_id : 振替伝票ID</li>     </ul>   </li>    <li>id : 受け付けID</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download_type** | **String** | ダウンロード形式 | [required] |
**company_id** | **i32** | 事業所ID | [required] |
**visible_tags** | Option<[**Vec<String>**](String.md)> | 補助科目やコメントとして出力する項目 |  |
**visible_ids** | Option<[**Vec<String>**](String.md)> | 追加出力するID項目 |  |
**start_date** | Option<**String**> | 取得開始日 (yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 取得終了日 (yyyy-mm-dd) |  |

### Return type

[**crate::models::JournalsResponse**](journalsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

