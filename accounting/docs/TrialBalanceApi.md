# \TrialBalanceApi

All URIs are relative to *https://api.freee.co.jp*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_trial_bs**](TrialBalanceApi.md#get_trial_bs) | **GET** /api/1/reports/trial_bs | 貸借対照表の取得
[**get_trial_bs_three_years**](TrialBalanceApi.md#get_trial_bs_three_years) | **GET** /api/1/reports/trial_bs_three_years | 貸借対照表(３期間比較)の取得
[**get_trial_bs_two_years**](TrialBalanceApi.md#get_trial_bs_two_years) | **GET** /api/1/reports/trial_bs_two_years | 貸借対照表(前年比較)の取得
[**get_trial_cr**](TrialBalanceApi.md#get_trial_cr) | **GET** /api/1/reports/trial_cr | 製造原価報告書の取得
[**get_trial_cr_sections**](TrialBalanceApi.md#get_trial_cr_sections) | **GET** /api/1/reports/trial_cr_sections | 製造原価報告書(部門比較)の取得
[**get_trial_cr_segment1_tags**](TrialBalanceApi.md#get_trial_cr_segment1_tags) | **GET** /api/1/reports/trial_cr_segment_1_tags | 製造原価報告書(セグメント1比較)の取得
[**get_trial_cr_segment2_tags**](TrialBalanceApi.md#get_trial_cr_segment2_tags) | **GET** /api/1/reports/trial_cr_segment_2_tags | 製造原価報告書(セグメント2比較)の取得
[**get_trial_cr_segment3_tags**](TrialBalanceApi.md#get_trial_cr_segment3_tags) | **GET** /api/1/reports/trial_cr_segment_3_tags | 製造原価報告書(セグメント3比較)の取得
[**get_trial_cr_three_years**](TrialBalanceApi.md#get_trial_cr_three_years) | **GET** /api/1/reports/trial_cr_three_years | 製造原価報告書(３期間比較)の取得
[**get_trial_cr_two_years**](TrialBalanceApi.md#get_trial_cr_two_years) | **GET** /api/1/reports/trial_cr_two_years | 製造原価報告書(前年比較)の取得
[**get_trial_pl**](TrialBalanceApi.md#get_trial_pl) | **GET** /api/1/reports/trial_pl | 損益計算書の取得
[**get_trial_pl_sections**](TrialBalanceApi.md#get_trial_pl_sections) | **GET** /api/1/reports/trial_pl_sections | 損益計算書(部門比較)の取得
[**get_trial_pl_segment1_tags**](TrialBalanceApi.md#get_trial_pl_segment1_tags) | **GET** /api/1/reports/trial_pl_segment_1_tags | 損益計算書(セグメント1比較)の取得
[**get_trial_pl_segment2_tags**](TrialBalanceApi.md#get_trial_pl_segment2_tags) | **GET** /api/1/reports/trial_pl_segment_2_tags | 損益計算書(セグメント2比較)の取得
[**get_trial_pl_segment3_tags**](TrialBalanceApi.md#get_trial_pl_segment3_tags) | **GET** /api/1/reports/trial_pl_segment_3_tags | 損益計算書(セグメント3比較)の取得
[**get_trial_pl_three_years**](TrialBalanceApi.md#get_trial_pl_three_years) | **GET** /api/1/reports/trial_pl_three_years | 損益計算書(３期間比較)の取得
[**get_trial_pl_two_years**](TrialBalanceApi.md#get_trial_pl_two_years) | **GET** /api/1/reports/trial_pl_two_years | 損益計算書(前年比較)の取得



## get_trial_bs

> crate::models::TrialBsResponse get_trial_bs(company_id, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, approval_flow_status)
貸借対照表の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の貸借対照表を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>opening_balance : 期首残高 </p> </li>  <li> <p>debit_amount : 借方金額 </p> </li> <li> <p>credit_amount:  貸方金額 </p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <li> <p>composition_ratio : 構成比</p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_bs?company_id=1&amp;fiscal_year=2019&amp;breakdown_display_type=partner</p> </blockquote>  <pre><code>{   &quot;trial_bs&quot; :     {       &quot;company_id&quot; : 1,       &quot;fiscal_year&quot; : 2019,       &quot;breakdown_display_type&quot; : &quot;partner&quot;,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1000,         &quot;account_item_name&quot; : &quot;現金&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;流動資産&quot;,         &quot;opening_balance&quot; : 100000,         &quot;debit_amount&quot; : 50000,         &quot;credit_amount&quot; : 20000,         &quot;closing_balance&quot; : 130000,         &quot;composition_ratio&quot; : 0.25         &quot;partners&quot; : [{           &quot;id&quot; : 123,           &quot;name&quot; : &quot;freee&quot;,           &quot;opening_balance&quot; : 100000,           &quot;debit_amount&quot; : 50000,           &quot;credit_amount&quot; : 20000,           &quot;closing_balance&quot; : 130000,           &quot;composition_ratio&quot; : 0.25           },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialBsResponse**](trialBsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_bs_three_years

> crate::models::TrialBsThreeYearsResponse get_trial_bs_three_years(company_id, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, approval_flow_status)
貸借対照表(３期間比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の貸借対照表(３期間比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>two_years_before_closing_balance:  前々年度期末残高 </p> </li> <li> <p>last_year_closing_balance:  前年度期末残高 </p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <li> <p>year_on_year : 前年比</p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_bs_three_years?company_id=1&amp;fiscal_year=2019</p> </blockquote>  <pre><code>{   &quot;trial_bs_three_years&quot; :     {       &quot;company_id&quot; : 1,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1000,         &quot;account_item_name&quot; : &quot;現金&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;流動資産&quot;,         &quot;two_year_before_closing_balance&quot; : 50000,         &quot;last_year_closing_balance&quot; : 25000,         &quot;closing_balance&quot; : 100000,         &quot;year_on_year&quot; : 0.85       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialBsThreeYearsResponse**](trialBsThreeYearsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_bs_two_years

> crate::models::TrialBsTwoYearsResponse get_trial_bs_two_years(company_id, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, approval_flow_status)
貸借対照表(前年比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の貸借対照表(前年比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>last_year_closing_balance:  前年度期末残高 </p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <li> <p>year_on_year : 前年比</p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul>  <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_bs_two_years?company_id=1&amp;fiscal_year=2019</p> </blockquote>  <pre><code>{   &quot;trial_bs_two_years&quot; :     {       &quot;company_id&quot; : 1,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1000,         &quot;account_item_name&quot; : &quot;現金&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;流動資産&quot;,         &quot;last_year_closing_balance&quot; : 25000,         &quot;closing_balance&quot; : 100000,         &quot;year_on_year&quot; : 0.85        },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialBsTwoYearsResponse**](trialBsTwoYearsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_cr

> crate::models::TrialCrResponse get_trial_cr(company_id, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
製造原価報告書の取得

<h2 id=\"\">概要</h2> <p>指定した事業所の製造原価報告書を取得する</p> <h2 id=\"_2\">定義</h2> <ul> <li> <p>created_at : 作成日時</p> </li> <li> <p>account_item_name : 勘定科目名</p> </li> <li> <p>hierarchy_level: 階層レベル</p> </li> <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>opening_balance : 期首残高 </p> </li> <li> <p>debit_amount : 借方金額 </p> </li> <li> <p>credit_amount:  貸方金額 </p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <li> <p>composition_ratio : 構成比</p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>法人向けのプランで利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>配賦仕訳の絞り込み（cost_allocation）は法人向けのベーシックプラン以上で利用可能です。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2> <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_cr?company_id=1&amp;fiscal_year=2019&amp;breakdown_display_type=partner</p> </blockquote> <pre><code>{   &quot;trial_cr&quot; :     {       &quot;company_id&quot; : 1,       &quot;fiscal_year&quot; : 2019,       &quot;breakdown_display_type&quot; : &quot;partner&quot;,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;[製]期首材料棚卸高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;期首原材料棚卸&quot;,         &quot;opening_balance&quot; : 100000,         &quot;debit_amount&quot; : 50000,         &quot;credit_amount&quot; : 20000,         &quot;closing_balance&quot; : 130000,         &quot;composition_ratio&quot; : 0.25         &quot;partners&quot; : [{           &quot;id&quot; : 123,           &quot;name&quot; : &quot;freee&quot;,           &quot;opening_balance&quot; : 100000,           &quot;debit_amount&quot; : 50000,           &quot;credit_amount&quot; : 20000,           &quot;closing_balance&quot; : 130000,           &quot;composition_ratio&quot; : 0.25           },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト), 全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialCrResponse**](trialCrResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_cr_sections

> crate::models::TrialCrSectionsResponse get_trial_cr_sections(company_id, section_ids, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, adjustment, cost_allocation, approval_flow_status)
製造原価報告書(部門比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の製造原価報告書(部門比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>法人向けのベーシックプラン以上で利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>配賦仕訳の絞り込み（cost_allocation）は法人向けのベーシックプラン以上で利用可能です。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_cr_sections?company_id=1&amp;section_ids=1,2,3&amp;fiscal_year=2019</p></p> </blockquote>  <pre><code>{   &quot;trial_cr_sections&quot; :     {       &quot;company_id&quot; : 1,       &quot;section_ids&quot; : &quot;1,2,3&quot;,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;[製]期首材料棚卸高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;期首原材料棚卸&quot;,         &quot;closing_balance&quot; : 1000000,         &quot;sections&quot; : [{           &quot;id&quot;: 1           &quot;name&quot;: &quot;営業部&quot;,           &quot;closing_balance&quot; : 100000         },         {           &quot;id&quot;: 2           &quot;name&quot;: &quot;広報部&quot;,           &quot;closing_balance&quot; : 200000         },         {           &quot;id&quot;: 3           &quot;name&quot;: &quot;人事部&quot;,           &quot;closing_balance&quot; : 300000         },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**section_ids** | **String** | 出力する部門の指定（半角数字のidを半角カンマ区切りスペースなしで指定してください。0を指定すると、未選択の部門で比較できます） | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialCrSectionsResponse**](trialCrSectionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_cr_segment1_tags

> crate::models::TrialCrSegment1TagsResponse get_trial_cr_segment1_tags(company_id, segment_1_tag_ids, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
製造原価報告書(セグメント1比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の製造原価報告書(セグメント1比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>法人向けのプロフェッショナルプラン以上で利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_cr_segment_1_tags?company_id=1&amp;segment_1_tag_ids=1,2,3&amp;fiscal_year=2019</p></p> </blockquote>  <pre><code>{   &quot;trial_cr_segment_1_tags&quot; :     {       &quot;company_id&quot; : 1,       &quot;segment_1_tag_ids&quot; : &quot;1,2,3&quot;,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;[製]期首材料棚卸高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;期首原材料棚卸&quot;,         &quot;closing_balance&quot; : 1000000,         &quot;segment_1_tags&quot; : [{           &quot;id&quot;: 1           &quot;name&quot;: &quot;プロジェクトA&quot;,           &quot;closing_balance&quot; : 100000         },         {           &quot;id&quot;: 2           &quot;name&quot;: &quot;プロジェクトB&quot;,           &quot;closing_balance&quot; : 200000         },         {           &quot;id&quot;: 3           &quot;name&quot;: &quot;プロジェクトC&quot;,           &quot;closing_balance&quot; : 300000         },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**segment_1_tag_ids** | **String** | 出力するセグメント1タグの指定（半角数字のidを半角カンマ区切りスペースなしで指定してください。0を指定すると、未選択のセグメントで比較できます） | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialCrSegment1TagsResponse**](trialCrSegment_1TagsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_cr_segment2_tags

> crate::models::TrialCrSegment2TagsResponse get_trial_cr_segment2_tags(company_id, segment_2_tag_ids, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
製造原価報告書(セグメント2比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の製造原価報告書(セグメント2比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>法人向けのエンタープライズプランで利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_cr_segment_2_tags?company_id=1&amp;segment_2_tag_ids=1,2,3&amp;fiscal_year=2019</p></p> </blockquote>  <pre><code>{   &quot;trial_cr_segment_2_tags&quot; :     {       &quot;company_id&quot; : 1,       &quot;segment_2_tag_ids&quot; : &quot;1,2,3&quot;,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;[製]期首材料棚卸高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;期首原材料棚卸&quot;,         &quot;closing_balance&quot; : 1000000,         &quot;segment_2_tags&quot; : [{           &quot;id&quot;: 1           &quot;name&quot;: &quot;プロジェクトA&quot;,           &quot;closing_balance&quot; : 100000         },         {           &quot;id&quot;: 2           &quot;name&quot;: &quot;プロジェクトB&quot;,           &quot;closing_balance&quot; : 200000         },         {           &quot;id&quot;: 3           &quot;name&quot;: &quot;プロジェクトC&quot;,           &quot;closing_balance&quot; : 300000         },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**segment_2_tag_ids** | **String** | 出力するセグメント2タグの指定（半角数字のidを半角カンマ区切りスペースなしで指定してください。0を指定すると、未選択のセグメントで比較できます） | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialCrSegment2TagsResponse**](trialCrSegment_2TagsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_cr_segment3_tags

> crate::models::TrialCrSegment3TagsResponse get_trial_cr_segment3_tags(company_id, segment_3_tag_ids, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
製造原価報告書(セグメント3比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の製造原価報告書(セグメント3比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>法人向けのエンタープライズプランで利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_cr_segment_3_tags?company_id=1&amp;segment_3_tag_ids=1,2,3&amp;fiscal_year=2019</p></p> </blockquote>  <pre><code>{   &quot;trial_cr_segment_3_tags&quot; :     {       &quot;company_id&quot; : 1,       &quot;segment_3_tag_ids&quot; : &quot;1,2,3&quot;,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;[製]期首材料棚卸高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;期首原材料棚卸&quot;,         &quot;closing_balance&quot; : 1000000,         &quot;segment_3_tags&quot; : [{           &quot;id&quot;: 1           &quot;name&quot;: &quot;プロジェクトA&quot;,           &quot;closing_balance&quot; : 100000         },         {           &quot;id&quot;: 2           &quot;name&quot;: &quot;プロジェクトB&quot;,           &quot;closing_balance&quot; : 200000         },         {           &quot;id&quot;: 3           &quot;name&quot;: &quot;プロジェクトC&quot;,           &quot;closing_balance&quot; : 300000         },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**segment_3_tag_ids** | **String** | 出力するセグメント3タグの指定（半角数字のidを半角カンマ区切りスペースなしで指定してください。0を指定すると、未選択のセグメントで比較できます） | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialCrSegment3TagsResponse**](trialCrSegment_3TagsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_cr_three_years

> crate::models::TrialCrThreeYearsResponse get_trial_cr_three_years(company_id, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
製造原価報告書(３期間比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の製造原価報告書(３期間比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>two_years_before_closing_balance:  前々年度期末残高 </p> </li> <li> <p>last_year_closing_balance:  前年度期末残高 </p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <li> <p>year_on_year : 前年比</p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>法人向けのプランで利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>配賦仕訳の絞り込み（cost_allocation）は法人向けのベーシックプラン以上で利用可能です。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_cr_three_years?company_id=1&fiscal_year=2019</p> </blockquote>  <pre><code>{   &quot;trial_cr_three_years&quot; :     {       &quot;company_id&quot; : 1,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;[製]期首材料棚卸高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;期首原材料棚卸&quot;,         &quot;two_year_before_closing_balance&quot; : 50000,         &quot;last_year_closing_balance&quot; : 25000,         &quot;closing_balance&quot; : 100000,         &quot;year_on_year&quot; : 0.85       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト), 全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialCrThreeYearsResponse**](trialCrThreeYearsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_cr_two_years

> crate::models::TrialCrTwoYearsResponse get_trial_cr_two_years(company_id, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
製造原価報告書(前年比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の製造原価報告書(前年比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>last_year_closing_balance:  前年度期末残高 </p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <li> <p>year_on_year : 前年比</p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>法人向けのプランで利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>配賦仕訳の絞り込み（cost_allocation）は法人向けのベーシックプラン以上で利用可能です。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul>  <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_cr_two_years?company_id=1&amp;fiscal_year=2019</p> </blockquote>  <pre><code>{   &quot;trial_cr_two_years&quot; :     {       &quot;company_id&quot; : 1,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;[製]期首材料棚卸高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;期首原材料棚卸&quot;,         &quot;last_year_closing_balance&quot; : 25000,         &quot;closing_balance&quot; : 100000,         &quot;year_on_year&quot; : 0.85        },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト), 全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialCrTwoYearsResponse**](trialCrTwoYearsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_pl

> crate::models::TrialPlResponse get_trial_pl(company_id, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
損益計算書の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の損益計算書を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>opening_balance : 期首残高 </p> </li>  <li> <p>debit_amount : 借方金額 </p> </li> <li> <p>credit_amount:  貸方金額 </p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <li> <p>composition_ratio : 構成比</p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>配賦仕訳の絞り込み（cost_allocation）は法人向けのベーシックプラン以上で利用可能です。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_pl?company_id=1&amp;fiscal_year=2019&amp;breakdown_display_type=partner</p> </blockquote>  <pre><code>{   &quot;trial_pl&quot; :     {       &quot;company_id&quot; : 1,       &quot;fiscal_year&quot; : 2019,       &quot;breakdown_display_type&quot; : &quot;partner&quot;,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;売上高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;営業収益&quot;,         &quot;opening_balance&quot; : 100000,         &quot;debit_amount&quot; : 50000,         &quot;credit_amount&quot; : 20000,         &quot;closing_balance&quot; : 130000,         &quot;composition_ratio&quot; : 0.25         &quot;partners&quot; : [{           &quot;id&quot; : 123,           &quot;name&quot; : &quot;freee&quot;,           &quot;opening_balance&quot; : 100000,           &quot;debit_amount&quot; : 50000,           &quot;credit_amount&quot; : 20000,           &quot;closing_balance&quot; : 130000,           &quot;composition_ratio&quot; : 0.25           },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialPlResponse**](trialPlResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_pl_sections

> crate::models::TrialPlSectionsResponse get_trial_pl_sections(company_id, section_ids, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, adjustment, cost_allocation, approval_flow_status)
損益計算書(部門比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の損益計算書(部門比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>個人向けのプレミアムプラン、法人向けのベーシックプラン以上で利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>配賦仕訳の絞り込み（cost_allocation）は法人向けのベーシックプラン以上で利用可能です。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_pl_sections?company_id=1&amp;section_ids=1,2,3&amp;fiscal_year=2019</p></p> </blockquote>  <pre><code>{   &quot;trial_pl_sections&quot; :     {       &quot;company_id&quot; : 1,       &quot;section_ids&quot; : &quot;1,2,3&quot;,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;売上高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;営業収益&quot;,         &quot;closing_balance&quot; : 1000000,         &quot;sections&quot; : [{           &quot;id&quot;: 1           &quot;name&quot;: &quot;営業部&quot;,           &quot;closing_balance&quot; : 100000         },         {           &quot;id&quot;: 2           &quot;name&quot;: &quot;広報部&quot;,           &quot;closing_balance&quot; : 200000         },         {           &quot;id&quot;: 3           &quot;name&quot;: &quot;人事部&quot;,           &quot;closing_balance&quot; : 300000         },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**section_ids** | **String** | 出力する部門の指定（半角数字のidを半角カンマ区切りスペースなしで指定してください。0を指定すると、未選択の部門で比較できます。） | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialPlSectionsResponse**](trialPlSectionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_pl_segment1_tags

> crate::models::TrialPlSegment1TagsResponse get_trial_pl_segment1_tags(company_id, segment_1_tag_ids, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
損益計算書(セグメント1比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の損益計算書(セグメント1比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>法人向けのプロフェッショナルプラン以上で利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_pl_segment_1_tags?company_id=1&amp;segment_1_tag_ids=1,2,3&amp;fiscal_year=2019</p></p> </blockquote>  <pre><code>{   &quot;trial_pl_segment_1_tags&quot; :     {       &quot;company_id&quot; : 1,       &quot;segment_1_tag_ids&quot; : &quot;1,2,3&quot;,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;売上高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;営業収益&quot;,         &quot;closing_balance&quot; : 1000000,         &quot;segment_1_tags&quot; : [{           &quot;id&quot;: 1           &quot;name&quot;: &quot;プロジェクトA&quot;,           &quot;closing_balance&quot; : 100000         },         {           &quot;id&quot;: 2           &quot;name&quot;: &quot;プロジェクトB&quot;,           &quot;closing_balance&quot; : 200000         },         {           &quot;id&quot;: 3           &quot;name&quot;: &quot;プロジェクトC&quot;,           &quot;closing_balance&quot; : 300000         },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**segment_1_tag_ids** | **String** | 出力するセグメント1タグの指定（半角数字のidを半角カンマ区切りスペースなしで指定してください。0を指定すると、未選択のセグメントで比較できます） | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialPlSegment1TagsResponse**](trialPlSegment_1TagsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_pl_segment2_tags

> crate::models::TrialPlSegment2TagsResponse get_trial_pl_segment2_tags(company_id, segment_2_tag_ids, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
損益計算書(セグメント2比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の損益計算書(セグメント2比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>法人向けのエンタープライズプランで利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_pl_segment_2_tags?company_id=1&amp;segment_2_tag_ids=1,2,3&amp;fiscal_year=2019</p></p> </blockquote>  <pre><code>{   &quot;trial_pl_segment_2_tags&quot; :     {       &quot;company_id&quot; : 1,       &quot;segment_2_tag_ids&quot; : &quot;1,2,3&quot;,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;売上高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;営業収益&quot;,         &quot;closing_balance&quot; : 1000000,         &quot;segment_2_tags&quot; : [{           &quot;id&quot;: 1           &quot;name&quot;: &quot;プロジェクトA&quot;,           &quot;closing_balance&quot; : 100000         },         {           &quot;id&quot;: 2           &quot;name&quot;: &quot;プロジェクトB&quot;,           &quot;closing_balance&quot; : 200000         },         {           &quot;id&quot;: 3           &quot;name&quot;: &quot;プロジェクトC&quot;,           &quot;closing_balance&quot; : 300000         },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**segment_2_tag_ids** | **String** | 出力するセグメント2タグの指定（半角数字のidを半角カンマ区切りスペースなしで指定してください。0を指定すると、未選択のセグメントで比較できます） | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialPlSegment2TagsResponse**](trialPlSegment_2TagsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_pl_segment3_tags

> crate::models::TrialPlSegment3TagsResponse get_trial_pl_segment3_tags(company_id, segment_3_tag_ids, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
損益計算書(セグメント3比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の損益計算書(セグメント3比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>法人向けのエンタープライズプランで利用可能なAPIです。対象外のプランでは401エラーを返却します。</li> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_pl_segment_3_tags?company_id=1&amp;segment_3_tag_ids=1,2,3&amp;fiscal_year=2019</p></p> </blockquote>  <pre><code>{   &quot;trial_pl_segment_3_tags&quot; :     {       &quot;company_id&quot; : 1,       &quot;segment_3_tag_ids&quot; : &quot;1,2,3&quot;,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;売上高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;営業収益&quot;,         &quot;closing_balance&quot; : 1000000,         &quot;segment_3_tags&quot; : [{           &quot;id&quot;: 1           &quot;name&quot;: &quot;プロジェクトA&quot;,           &quot;closing_balance&quot; : 100000         },         {           &quot;id&quot;: 2           &quot;name&quot;: &quot;プロジェクトB&quot;,           &quot;closing_balance&quot; : 200000         },         {           &quot;id&quot;: 3           &quot;name&quot;: &quot;プロジェクトC&quot;,           &quot;closing_balance&quot; : 300000         },         ...         ]       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**segment_3_tag_ids** | **String** | 出力するセグメント3タグの指定（半角数字のidを半角カンマ区切りスペースなしで指定してください。0を指定すると、未選択のセグメントで比較できます） | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialPlSegment3TagsResponse**](trialPlSegment_3TagsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_pl_three_years

> crate::models::TrialPlThreeYearsResponse get_trial_pl_three_years(company_id, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
損益計算書(３期間比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の損益計算書(３期間比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>two_years_before_closing_balance:  前々年度期末残高 </p> </li> <li> <p>last_year_closing_balance:  前年度期末残高 </p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <li> <p>year_on_year : 前年比</p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>配賦仕訳の絞り込み（cost_allocation）は法人向けのベーシックプラン以上で利用可能です。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul> <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_pl_three_years?company_id=1&fiscal_year=2019</p> </blockquote>  <pre><code>{   &quot;trial_pl_three_years&quot; :     {       &quot;company_id&quot; : 1,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;売上高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;営業収益&quot;,         &quot;two_year_before_closing_balance&quot; : 50000,         &quot;last_year_closing_balance&quot; : 25000,         &quot;closing_balance&quot; : 100000,         &quot;year_on_year&quot; : 0.85       },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialPlThreeYearsResponse**](trialPlThreeYearsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trial_pl_two_years

> crate::models::TrialPlTwoYearsResponse get_trial_pl_two_years(company_id, fiscal_year, start_month, end_month, start_date, end_date, account_item_display_type, breakdown_display_type, partner_id, partner_code, item_id, section_id, adjustment, cost_allocation, approval_flow_status)
損益計算書(前年比較)の取得

 <h2 id=\"\">概要</h2>  <p>指定した事業所の損益計算書(前年比較)を取得する</p>  <h2 id=\"_2\">定義</h2>  <ul>  <li> <p>created_at : 作成日時</p> </li>  <li> <p>account_item_name : 勘定科目名</p> </li>  <li> <p>hierarchy_level: 階層レベル</p> </li>  <li> <p>parent_account_category_name: 上位勘定科目カテゴリー名</p> </li> <li> <p>last_year_closing_balance:  前年度期末残高 </p> </li> <li> <p>closing_balance : 期末残高 </p> </li> <li> <p>year_on_year : 前年比</p> </li> <h2 id=\"_3\">注意点</h2> <ul> <li>会計年度が指定されない場合、現在の会計年度がデフォルトとなります。</li> <li>up_to_dateがfalseの場合、残高の集計が完了していません。最新の集計結果を確認したい場合は、時間を空けて再度取得する必要があります。</li> <li>配賦仕訳の絞り込み（cost_allocation）は法人向けのベーシックプラン以上で利用可能です。</li> <li>取引先、品目、部門、セグメントはそれぞれの合計値が1000を超えるとレスポンスを得ることができません。</li> <li>partner_codeとpartner_idは同時に指定することはできません。</li> <li>start_date / end_date を指定した場合、以下を同時に指定することはできません。</li>   <ul>   <li>fiscal_year</li>   <li>start_month</li>   <li>end_month</li>   </ul> <li>0を指定すると未選択で絞り込めます</li>   <ul>   <li>partner_idに0を指定して絞り込んだ場合</li>     <ul>     <li>取引先が設定されていない取引、振替伝票の金額がレスポンスに返却されます</li>     </ul>   </ul> </ul>  <h2 id=\"_4\">レスポンスの例</h2>  <blockquote> <p>GET https://api.freee.co.jp/api/1/reports/trial_pl_two_years?company_id=1&amp;fiscal_year=2019</p> </blockquote>  <pre><code>{   &quot;trial_pl_two_years&quot; :     {       &quot;company_id&quot; : 1,       &quot;fiscal_year&quot; : 2019,       &quot;created_at&quot; : &quot;2019-12-17 12:00:50&quot       &quot;balances&quot; : [{         &quot;account_item_id&quot; : 1500,         &quot;account_item_name&quot; : &quot;売上高&quot;,         &quot;hierarchy_level&quot; : 2,         &quot;account_category_name&quot; : &quot;営業収益&quot;,         &quot;last_year_closing_balance&quot; : 25000,         &quot;closing_balance&quot; : 100000,         &quot;year_on_year&quot; : 0.85        },       ...       ]     } }</code></pre> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | 事業所ID | [required] |
**fiscal_year** | Option<**i32**> | 会計年度 |  |
**start_month** | Option<**i32**> | 発生月で絞込：開始会計月(1-12)。指定されない場合、現在の会計年度の期首月が指定されます。 |  |
**end_month** | Option<**i32**> | 発生月で絞込：終了会計月(1-12)(会計年度が10月始まりでstart_monthが11なら11, 12, 1, ... 9のいずれかを指定する)。指定されない場合、現在の会計年度の期末月が指定されます。 |  |
**start_date** | Option<**String**> | 発生日で絞込：開始日(yyyy-mm-dd) |  |
**end_date** | Option<**String**> | 発生日で絞込：終了日(yyyy-mm-dd) |  |
**account_item_display_type** | Option<**String**> | 勘定科目の表示（勘定科目: account_item, 決算書表示:group）。指定されない場合、勘定科目: account_itemが指定されます。 |  |
**breakdown_display_type** | Option<**String**> | 内訳の表示（取引先: partner, 品目: item, 部門: section, 勘定科目: account_item, セグメント1(法人向けプロフェッショナル, 法人向けエンタープライズプラン): segment_1_tag, セグメント2(法人向け エンタープライズプラン):segment_2_tag, セグメント3(法人向け エンタープライズプラン): segment_3_tag） ※勘定科目はaccount_item_display_typeが「group」の時のみ指定できます |  |
**partner_id** | Option<**i32**> | 取引先IDで絞込（0を指定すると、取引先が未選択で絞り込めます） |  |
**partner_code** | Option<**String**> | 取引先コードで絞込（事業所設定で取引先コードの利用を有効にしている場合のみ利用可能です） |  |
**item_id** | Option<**i32**> | 品目IDで絞込（0を指定すると、品目が未選択で絞り込めます） |  |
**section_id** | Option<**i32**> | 部門IDで絞込（0を指定すると、部門が未選択で絞り込めます） |  |
**adjustment** | Option<**String**> | 決算整理仕訳で絞込（決算整理仕訳のみ: only, 決算整理仕訳以外: without）。指定されない場合、決算整理仕訳以外: withoutが指定されます。 |  |
**cost_allocation** | Option<**String**> | 配賦仕訳で絞込（配賦仕訳のみ：only,配賦仕訳以外：without）。指定されない場合、配賦仕訳を含む金額が返却されます。 |  |
**approval_flow_status** | Option<**String**> | 承認ステータスで絞込 (未承認を除く: without_in_progress (デフォルト)、全てのステータス: all)<br> 個人: プレミアムプラン、法人: プロフェッショナルプラン以上で指定可能です。<br> 事業所の設定から仕訳承認フローの利用を有効にした場合に指定可能です。  |  |

### Return type

[**crate::models::TrialPlTwoYearsResponse**](trialPlTwoYearsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

