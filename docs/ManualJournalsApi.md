# \ManualJournalsApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_manual_journal**](ManualJournalsApi.md#create_manual_journal) | **POST** /api/1/manual_journals | 振替伝票の作成
[**destroy_manual_journal**](ManualJournalsApi.md#destroy_manual_journal) | **DELETE** /api/1/manual_journals/{id} | 振替伝票の削除
[**get_manual_journal**](ManualJournalsApi.md#get_manual_journal) | **GET** /api/1/manual_journals/{id} | 振替伝票の取得
[**get_manual_journals**](ManualJournalsApi.md#get_manual_journals) | **GET** /api/1/manual_journals | 振替伝票一覧の取得
[**update_manual_journal**](ManualJournalsApi.md#update_manual_journal) | **PUT** /api/1/manual_journals/{id} | 振替伝票の更新



## create_manual_journal

> crate::models::ManualJournalResponse create_manual_journal(manual_journal_create_params)
振替伝票の作成

 <h2 id=\"\">概要</h2>  <p>指定した事業所の振替伝票を作成する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>issue_date : 発生日</p> </li>  <li> <p>adjustment : 決算整理仕訳フラグ（true: 決算整理仕訳, false: 日常仕訳）</p> </li>  <li> <p>txn_number : 仕訳番号</p> </li>  <li> <p>details : 振替伝票の貸借行</p> </li>  <li> <p>entry_side : 貸借区分</p>  <ul> <li>credit : 貸方</li>  <li>debit : 借方</li> </ul> </li>  <li> <p>amount : 金額</p> </li> </ul>  <h2 id=\"_3\">注意点</h2>  <ul> <li>振替伝票は売掛・買掛レポートには反映されません。債権・債務データの登録は取引(Deals)をお使いください。</li> <li>事業所の仕訳番号形式が有効な場合のみ、レスポンスで仕訳番号(txn_number)を返します。</li> <li>貸借合わせて100行まで仕訳行を登録できます。</li> <li>セグメントタグ情報は法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li> <li>partner_codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。またpartner_codeとpartner_idは同時に指定することはできません。</li></ul>  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manual_journal_create_params** | Option<[**ManualJournalCreateParams**](ManualJournalCreateParams.md)> | 振替伝票の作成 |  |

### Return type

[**crate::models::ManualJournalResponse**](manualJournalResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_manual_journal

> destroy_manual_journal(id, company_id)
振替伝票の削除

 <h2 id=\"\">概要</h2>  <p>指定した事業所の振替伝票を削除する</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**company_id** | **i32** | 事業所ID | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_manual_journal

> crate::models::ManualJournalResponse get_manual_journal(company_id, id)
振替伝票の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の振替伝票を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>issue_date : 発生日</p> </li> <li> <p>adjustment : 決算整理仕訳フラグ（true: 決算整理仕訳, false: 日常仕訳）</p> </li> <li> <p>txn_number : 仕訳番号</p> </li> <li> <p>details : 振替伝票の貸借行</p> </li> <li> <p>entry_side : 貸借区分</p> <ul> <li>credit : 貸方</li> <li>debit : 借方</li> </ul> </li> <li> <p>amount : 金額</p> </li> </ul>  <h2 id=\"_3\">注意点</h2>  <ul> <li>振替伝票は売掛・買掛レポートには反映されません。債権・債務データの登録は取引(Deals)をお使いください。</li> <li>事業所の仕訳番号形式が有効な場合のみ、レスポンスで仕訳番号(txn_number)を返します。</li> <li>セグメントタグ情報は法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**id** | **i32** |  | [required] |

### Return type

[**crate::models::ManualJournalResponse**](manualJournalResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_manual_journals

> crate::models::InlineResponse2003 get_manual_journals(company_id, start_issue_date, end_issue_date, entry_side, account_item_id, min_amount, max_amount, partner_id, partner_code, item_id, section_id, segment_1_tag_id, segment_2_tag_id, segment_3_tag_id, comment_status, comment_important, adjustment, txn_number, offset, limit)
振替伝票一覧の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の振替伝票一覧を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>issue_date : 発生日</p> </li>  <li> <p>adjustment : 決算整理仕訳フラグ（true: 決算整理仕訳, false: 日常仕訳）</p> </li>  <li> <p>txn_number : 仕訳番号</p> </li>  <li> <p>details : 振替伝票の貸借行</p> </li>  <li> <p>entry_side : 貸借区分</p>  <ul> <li>credit : 貸方</li>  <li>debit : 借方</li> </ul> </li>  <li> <p>amount : 金額</p> </li> </ul>  <h2 id=\"_3\">注意点</h2>  <ul> <li>振替伝票は売掛・買掛レポートには反映されません。債権・債務データの登録は取引(Deals)をお使いください。</li> <li>事業所の仕訳番号形式が有効な場合のみ、レスポンスで仕訳番号(txn_number)を返します。</li> <li>セグメントタグ情報は法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li> <li>partner_codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。またpartner_codeとpartner_idは同時に指定することはできません。</li></ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**start_issue_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_issue_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**entry_side** | Option<**String**> | 貸借で絞込 (貸方: credit, 借方: debit) |  |
**account_item_id** | Option<**i32**> | 勘定科目IDで絞込 |  |
**min_amount** | Option<**i64**> | 金額で絞込：下限 |  |
**max_amount** | Option<**i64**> | 金額で絞込：上限 |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択の貸借行を絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込 |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択の貸借行を絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択の貸借行を絞り込めます） |  |
**segment_1_tag_id** | Option<**i64**> | セグメント１IDで絞込（0を指定すると、セグメント１が未選択の貸借行を絞り込めます） |  |
**segment_2_tag_id** | Option<**i64**> | セグメント２IDで絞込（0を指定すると、セグメント２が未選択の貸借行を絞り込めます） |  |
**segment_3_tag_id** | Option<**i64**> | セグメント３IDで絞込（0を指定すると、セグメント３が未選択の貸借行を絞り込めます） |  |
**comment_status** | Option<**String**> | コメント状態で絞込（自分宛のコメント: posted_with_mention, 自分宛のコメント-未解決: raised_with_mention, 自分宛のコメント-解決済: resolved_with_mention, コメントあり: posted, 未解決: raised, 解決済: resolved, コメントなし: none） |  |
**comment_important** | Option<**bool**> | 重要コメント付きの振替伝票を絞込 |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without） |  |
**txn_number** | Option<**String**> | 仕訳番号で絞込（事業所の仕訳番号形式が有効な場合のみ） |  |
**offset** | Option<**i64**> | 取得レコードのオフセット (デフォルト: 0) |  |
**limit** | Option<**i32**> | 取得レコードの件数 (デフォルト: 20, 最小: 1, 最大: 500)  |  |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_manual_journal

> crate::models::ManualJournalResponse update_manual_journal(id, manual_journal_update_params)
振替伝票の更新

 <h2 id=\"\">概要</h2>  <p>指定した事業所の振替伝票を更新する</p>  <h2 id=\"_2\">定義</h2>  <ul> <li> <p>issue_date : 発生日</p> </li>  <li> <p>adjustment : 決算整理仕訳フラグ（true: 決算整理仕訳, false: 日常仕訳）</p> </li>  <li> <p>txn_number : 仕訳番号</p> </li>  <li> <p>details : 振替伝票の貸借行</p> </li>  <li> <p>entry_side : 貸借区分</p>  <ul> <li>credit : 貸方</li>  <li>debit : 借方</li> </ul> </li>  <li> <p>amount : 金額</p> </li> </ul>  <h2 id=\"_3\">注意点</h2>  <ul> <li>振替伝票は売掛・買掛レポートには反映されません。債権・債務データの登録は取引(Deals)をお使いください。</li>  <li>事業所の仕訳番号形式が有効な場合のみ、レスポンスで仕訳番号(txn_number)を返します。</li> <li>貸借合わせて100行まで仕訳行を登録できます。</li>  <li>detailsに含まれない既存の貸借行は削除されます。更新後も残したい行は、必ず貸借行IDを指定してdetailsに含めてください。</li>  <li>detailsに含まれる貸借行IDの指定がある行は、更新行として扱われ更新されます。</li>  <li>detailsに含まれる貸借行IDの指定がない行は、新規行として扱われ追加されます。</li> <li>セグメントタグ情報は法人向けのプロフェッショナルプラン以上で利用可能です。利用可能なセグメントの数は、法人向けのプロフェッショナルプランの場合は1つ、エンタープライズプランの場合は3つです。</li> <li>partner_codeを利用するには、事業所の設定から取引先コードの利用を有効にする必要があります。またpartner_codeとpartner_idは同時に指定することはできません。</li></ul>  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**manual_journal_update_params** | Option<[**ManualJournalUpdateParams**](ManualJournalUpdateParams.md)> | 振替伝票の更新 |  |

### Return type

[**crate::models::ManualJournalResponse**](manualJournalResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

